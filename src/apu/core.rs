use crate::apu::channels::*;
use crate::apu::mixer::Mixer;
use crate::error::{MemoryError, MemoryResult};

/// APU (Audio Processing Unit) - Handles sound generation
pub struct Apu {
    pub pulse1: PulseChannel,
    pub pulse2: PulseChannel,
    pub triangle: TriangleChannel,
    pub noise: NoiseChannel,
    pub dmc: DmcChannel,
    pub mixer: Mixer,
    pub frame_counter: u16,
    pub frame_sequence: u8,
    pub interrupt_inhibit: bool,
    pub five_step_mode: bool,
    pub cycle: u64,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            pulse1: PulseChannel::new(0),
            pulse2: PulseChannel::new(1),
            triangle: TriangleChannel::new(),
            noise: NoiseChannel::new(),
            dmc: DmcChannel::new(),
            mixer: Mixer::new(),
            frame_counter: 0,
            frame_sequence: 0,
            interrupt_inhibit: false,
            five_step_mode: false,
            cycle: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.pulse1.reset();
        self.pulse2.reset();
        self.triangle.reset();
        self.noise.reset();
        self.dmc.reset();
        self.frame_counter = 0;
        self.frame_sequence = 0;
        self.interrupt_inhibit = false;
        self.five_step_mode = false;
        self.cycle = 0;
    }
    
    pub fn step(&mut self) -> MemoryResult<()> {
        self.cycle += 1;
        
        // Step individual channels
        self.pulse1.step();
        self.pulse2.step();
        self.triangle.step();
        self.noise.step();
        self.dmc.step();
        
        // Handle frame counter
        if self.cycle % 7457 == 0 {
            self.step_frame_counter();
        }
        
        Ok(())
    }
    
    pub fn read_register(&self, address: u16) -> MemoryResult<u8> {
        match address {
            0x4015 => {
                // Status register
                let mut status = 0;
                if self.pulse1.length_counter > 0 { status |= 0x01; }
                if self.pulse2.length_counter > 0 { status |= 0x02; }
                if self.triangle.length_counter > 0 { status |= 0x04; }
                if self.noise.length_counter > 0 { status |= 0x08; }
                if self.dmc.bytes_remaining > 0 { status |= 0x10; }
                Ok(status)
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    pub fn write_register(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        match address {
            0x4000..=0x4003 => self.pulse1.write_register(address, value),
            0x4004..=0x4007 => self.pulse2.write_register(address, value),
            0x4008..=0x400B => self.triangle.write_register(address, value),
            0x400C..=0x400F => self.noise.write_register(address, value),
            0x4010..=0x4013 => self.dmc.write_register(address, value),
            0x4015 => {
                // Status register
                self.pulse1.enabled = (value & 0x01) != 0;
                self.pulse2.enabled = (value & 0x02) != 0;
                self.triangle.enabled = (value & 0x04) != 0;
                self.noise.enabled = (value & 0x08) != 0;
                self.dmc.enabled = (value & 0x10) != 0;
                
                if !self.pulse1.enabled { self.pulse1.length_counter = 0; }
                if !self.pulse2.enabled { self.pulse2.length_counter = 0; }
                if !self.triangle.enabled { self.triangle.length_counter = 0; }
                if !self.noise.enabled { self.noise.length_counter = 0; }
                if !self.dmc.enabled { self.dmc.bytes_remaining = 0; }
                
                Ok(())
            }
            0x4017 => {
                // Frame counter
                self.interrupt_inhibit = (value & 0x40) != 0;
                self.five_step_mode = (value & 0x80) != 0;
                
                if self.five_step_mode {
                    self.step_envelope();
                    self.step_length_counter();
                }
                
                Ok(())
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    pub fn get_sample(&mut self) -> f32 {
        let pulse1_sample = self.pulse1.get_sample();
        let pulse2_sample = self.pulse2.get_sample();
        let triangle_sample = self.triangle.get_sample();
        let noise_sample = self.noise.get_sample();
        let dmc_sample = self.dmc.get_sample();
        
        self.mixer.mix(pulse1_sample, pulse2_sample, triangle_sample, noise_sample, dmc_sample)
    }
    
    fn step_frame_counter(&mut self) {
        if self.five_step_mode {
            // 5-step mode
            match self.frame_sequence {
                0 => self.step_envelope(),
                1 => { self.step_envelope(); self.step_length_counter(); }
                2 => self.step_envelope(),
                3 => {},
                4 => { self.step_envelope(); self.step_length_counter(); }
                _ => {}
            }
            self.frame_sequence = (self.frame_sequence + 1) % 5;
        } else {
            // 4-step mode
            match self.frame_sequence {
                0 => self.step_envelope(),
                1 => { self.step_envelope(); self.step_length_counter(); }
                2 => self.step_envelope(),
                3 => { self.step_envelope(); self.step_length_counter(); }
                _ => {}
            }
            self.frame_sequence = (self.frame_sequence + 1) % 4;
        }
    }
    
    fn step_envelope(&mut self) {
        self.pulse1.step_envelope();
        self.pulse2.step_envelope();
        self.triangle.step_envelope();
        self.noise.step_envelope();
    }
    
    fn step_length_counter(&mut self) {
        self.pulse1.step_length_counter();
        self.pulse2.step_length_counter();
        self.triangle.step_length_counter();
        self.noise.step_length_counter();
    }
}

impl Default for Apu {
    fn default() -> Self {
        Self::new()
    }
}
