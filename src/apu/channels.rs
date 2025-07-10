use crate::error::{MemoryError, MemoryResult};

/// Pulse channel (used for both pulse channels)
pub struct PulseChannel {
    pub enabled: bool,
    pub channel: u8,
    pub duty_cycle: u8,
    pub length_counter: u8,
    pub envelope_enabled: bool,
    pub envelope_loop: bool,
    pub envelope_start: bool,
    pub envelope_divider: u8,
    pub envelope_counter: u8,
    pub envelope_volume: u8,
    pub sweep_enabled: bool,
    pub sweep_period: u8,
    pub sweep_negate: bool,
    pub sweep_shift: u8,
    pub sweep_reload: bool,
    pub sweep_counter: u8,
    pub timer_period: u16,
    pub timer_counter: u16,
    pub sequence_counter: u8,
}

impl PulseChannel {
    pub fn new(channel: u8) -> Self {
        Self {
            enabled: false,
            channel,
            duty_cycle: 0,
            length_counter: 0,
            envelope_enabled: false,
            envelope_loop: false,
            envelope_start: false,
            envelope_divider: 0,
            envelope_counter: 0,
            envelope_volume: 0,
            sweep_enabled: false,
            sweep_period: 0,
            sweep_negate: false,
            sweep_shift: 0,
            sweep_reload: false,
            sweep_counter: 0,
            timer_period: 0,
            timer_counter: 0,
            sequence_counter: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.enabled = false;
        self.length_counter = 0;
        self.envelope_start = false;
        self.envelope_counter = 0;
        self.sweep_reload = false;
        self.sweep_counter = 0;
        self.timer_counter = 0;
        self.sequence_counter = 0;
    }
    
    pub fn step(&mut self) {
        if self.timer_counter == 0 {
            self.timer_counter = self.timer_period;
            self.sequence_counter = (self.sequence_counter + 1) % 8;
        } else {
            self.timer_counter -= 1;
        }
    }
    
    pub fn step_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_counter = 15;
            self.envelope_divider = self.envelope_volume;
        } else if self.envelope_divider == 0 {
            self.envelope_divider = self.envelope_volume;
            if self.envelope_counter == 0 {
                if self.envelope_loop {
                    self.envelope_counter = 15;
                }
            } else {
                self.envelope_counter -= 1;
            }
        } else {
            self.envelope_divider -= 1;
        }
    }
    
    pub fn step_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope_loop {
            self.length_counter -= 1;
        }
    }
    
    pub fn step_sweep(&mut self) {
        if self.sweep_reload {
            self.sweep_reload = false;
            self.sweep_counter = self.sweep_period;
        } else if self.sweep_counter == 0 {
            self.sweep_counter = self.sweep_period;
            if self.sweep_enabled {
                let change = self.timer_period >> self.sweep_shift;
                if self.sweep_negate {
                    self.timer_period = self.timer_period.saturating_sub(change);
                    if self.channel == 0 {
                        self.timer_period = self.timer_period.saturating_sub(1);
                    }
                } else {
                    self.timer_period = self.timer_period.saturating_add(change);
                }
            }
        } else {
            self.sweep_counter -= 1;
        }
    }
    
    pub fn write_register(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        let base = if self.channel == 0 { 0x4000 } else { 0x4004 };
        match address - base {
            0 => {
                // Duty cycle, length counter halt, envelope
                self.duty_cycle = (value & 0xC0) >> 6;
                self.envelope_loop = (value & 0x20) != 0;
                self.envelope_enabled = (value & 0x10) == 0;
                self.envelope_volume = value & 0x0F;
                Ok(())
            }
            1 => {
                // Sweep
                self.sweep_enabled = (value & 0x80) != 0;
                self.sweep_period = (value & 0x70) >> 4;
                self.sweep_negate = (value & 0x08) != 0;
                self.sweep_shift = value & 0x07;
                self.sweep_reload = true;
                Ok(())
            }
            2 => {
                // Timer low
                self.timer_period = (self.timer_period & 0xFF00) | (value as u16);
                Ok(())
            }
            3 => {
                // Timer high, length counter
                self.timer_period = (self.timer_period & 0x00FF) | (((value & 0x07) as u16) << 8);
                self.length_counter = LENGTH_TABLE[(value >> 3) as usize];
                self.envelope_start = true;
                self.sequence_counter = 0;
                Ok(())
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    pub fn get_sample(&self) -> f32 {
        if !self.enabled || self.length_counter == 0 || self.timer_period < 8 {
            return 0.0;
        }
        
        let duty_table = [
            [0, 1, 0, 0, 0, 0, 0, 0], // 12.5%
            [0, 1, 1, 0, 0, 0, 0, 0], // 25%
            [0, 1, 1, 1, 1, 0, 0, 0], // 50%
            [1, 0, 0, 1, 1, 1, 1, 1], // 25% negated
        ];
        
        let duty_output = duty_table[self.duty_cycle as usize][self.sequence_counter as usize];
        if duty_output == 0 {
            return 0.0;
        }
        
        let volume = if self.envelope_enabled {
            self.envelope_counter
        } else {
            self.envelope_volume
        };
        
        volume as f32 / 15.0
    }
}

/// Triangle channel
pub struct TriangleChannel {
    pub enabled: bool,
    pub length_counter: u8,
    pub linear_counter: u8,
    pub linear_counter_reload: u8,
    pub linear_counter_control: bool,
    pub linear_counter_reload_flag: bool,
    pub timer_period: u16,
    pub timer_counter: u16,
    pub sequence_counter: u8,
}

impl TriangleChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            length_counter: 0,
            linear_counter: 0,
            linear_counter_reload: 0,
            linear_counter_control: false,
            linear_counter_reload_flag: false,
            timer_period: 0,
            timer_counter: 0,
            sequence_counter: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.enabled = false;
        self.length_counter = 0;
        self.linear_counter = 0;
        self.linear_counter_reload_flag = false;
        self.timer_counter = 0;
        self.sequence_counter = 0;
    }
    
    pub fn step(&mut self) {
        if self.timer_counter == 0 {
            self.timer_counter = self.timer_period;
            if self.length_counter > 0 && self.linear_counter > 0 {
                self.sequence_counter = (self.sequence_counter + 1) % 32;
            }
        } else {
            self.timer_counter -= 1;
        }
    }
    
    pub fn step_envelope(&mut self) {
        if self.linear_counter_reload_flag {
            self.linear_counter = self.linear_counter_reload;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }
        
        if !self.linear_counter_control {
            self.linear_counter_reload_flag = false;
        }
    }
    
    pub fn step_length_counter(&mut self) {
        if self.length_counter > 0 && !self.linear_counter_control {
            self.length_counter -= 1;
        }
    }
    
    pub fn write_register(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        match address {
            0x4008 => {
                // Linear counter
                self.linear_counter_control = (value & 0x80) != 0;
                self.linear_counter_reload = value & 0x7F;
                Ok(())
            }
            0x400A => {
                // Timer low
                self.timer_period = (self.timer_period & 0xFF00) | (value as u16);
                Ok(())
            }
            0x400B => {
                // Timer high, length counter
                self.timer_period = (self.timer_period & 0x00FF) | (((value & 0x07) as u16) << 8);
                self.length_counter = LENGTH_TABLE[(value >> 3) as usize];
                self.linear_counter_reload_flag = true;
                Ok(())
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    pub fn get_sample(&self) -> f32 {
        if !self.enabled || self.length_counter == 0 || self.linear_counter == 0 {
            return 0.0;
        }
        
        let triangle_table = [
            15, 14, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1,  0,
             0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
        ];
        
        let output = triangle_table[self.sequence_counter as usize];
        output as f32 / 15.0
    }
}

/// Noise channel
pub struct NoiseChannel {
    pub enabled: bool,
    pub length_counter: u8,
    pub envelope_enabled: bool,
    pub envelope_loop: bool,
    pub envelope_start: bool,
    pub envelope_divider: u8,
    pub envelope_counter: u8,
    pub envelope_volume: u8,
    pub timer_period: u16,
    pub timer_counter: u16,
    pub mode: bool,
    pub shift_register: u16,
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            length_counter: 0,
            envelope_enabled: false,
            envelope_loop: false,
            envelope_start: false,
            envelope_divider: 0,
            envelope_counter: 0,
            envelope_volume: 0,
            timer_period: 0,
            timer_counter: 0,
            mode: false,
            shift_register: 1,
        }
    }
    
    pub fn reset(&mut self) {
        self.enabled = false;
        self.length_counter = 0;
        self.envelope_start = false;
        self.envelope_counter = 0;
        self.timer_counter = 0;
        self.shift_register = 1;
    }
    
    pub fn step(&mut self) {
        if self.timer_counter == 0 {
            self.timer_counter = self.timer_period;
            
            let feedback = if self.mode {
                (self.shift_register & 1) ^ ((self.shift_register >> 6) & 1)
            } else {
                (self.shift_register & 1) ^ ((self.shift_register >> 1) & 1)
            };
            
            self.shift_register >>= 1;
            self.shift_register |= feedback << 14;
        } else {
            self.timer_counter -= 1;
        }
    }
    
    pub fn step_envelope(&mut self) {
        if self.envelope_start {
            self.envelope_start = false;
            self.envelope_counter = 15;
            self.envelope_divider = self.envelope_volume;
        } else if self.envelope_divider == 0 {
            self.envelope_divider = self.envelope_volume;
            if self.envelope_counter == 0 {
                if self.envelope_loop {
                    self.envelope_counter = 15;
                }
            } else {
                self.envelope_counter -= 1;
            }
        } else {
            self.envelope_divider -= 1;
        }
    }
    
    pub fn step_length_counter(&mut self) {
        if self.length_counter > 0 && !self.envelope_loop {
            self.length_counter -= 1;
        }
    }
    
    pub fn write_register(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        match address {
            0x400C => {
                // Envelope
                self.envelope_loop = (value & 0x20) != 0;
                self.envelope_enabled = (value & 0x10) == 0;
                self.envelope_volume = value & 0x0F;
                Ok(())
            }
            0x400E => {
                // Timer
                self.mode = (value & 0x80) != 0;
                let period_index = value & 0x0F;
                self.timer_period = NOISE_PERIOD_TABLE[period_index as usize];
                Ok(())
            }
            0x400F => {
                // Length counter
                self.length_counter = LENGTH_TABLE[(value >> 3) as usize];
                self.envelope_start = true;
                Ok(())
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    pub fn get_sample(&self) -> f32 {
        if !self.enabled || self.length_counter == 0 || (self.shift_register & 1) != 0 {
            return 0.0;
        }
        
        let volume = if self.envelope_enabled {
            self.envelope_counter
        } else {
            self.envelope_volume
        };
        
        volume as f32 / 15.0
    }
}

/// DMC channel
pub struct DmcChannel {
    pub enabled: bool,
    pub irq_enabled: bool,
    pub loop_flag: bool,
    pub rate_index: u8,
    pub timer_period: u16,
    pub timer_counter: u16,
    pub sample_address: u16,
    pub sample_length: u16,
    pub current_address: u16,
    pub bytes_remaining: u16,
    pub sample_buffer: u8,
    pub sample_buffer_empty: bool,
    pub shift_register: u8,
    pub bits_remaining: u8,
    pub silence_flag: bool,
    pub output_level: u8,
}

impl DmcChannel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            irq_enabled: false,
            loop_flag: false,
            rate_index: 0,
            timer_period: 0,
            timer_counter: 0,
            sample_address: 0,
            sample_length: 0,
            current_address: 0,
            bytes_remaining: 0,
            sample_buffer: 0,
            sample_buffer_empty: true,
            shift_register: 0,
            bits_remaining: 0,
            silence_flag: true,
            output_level: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.enabled = false;
        self.bytes_remaining = 0;
        self.sample_buffer_empty = true;
        self.bits_remaining = 0;
        self.silence_flag = true;
        self.output_level = 0;
    }
    
    pub fn step(&mut self) {
        if self.timer_counter == 0 {
            self.timer_counter = self.timer_period;
            
            if !self.silence_flag {
                if (self.shift_register & 1) != 0 {
                    if self.output_level <= 125 {
                        self.output_level += 2;
                    }
                } else if self.output_level >= 2 {
                    self.output_level -= 2;
                }
            }
            
            self.shift_register >>= 1;
            self.bits_remaining -= 1;
            
            if self.bits_remaining == 0 {
                self.bits_remaining = 8;
                if self.sample_buffer_empty {
                    self.silence_flag = true;
                } else {
                    self.silence_flag = false;
                    self.shift_register = self.sample_buffer;
                    self.sample_buffer_empty = true;
                }
            }
        } else {
            self.timer_counter -= 1;
        }
    }
    
    pub fn step_envelope(&mut self) {
        // DMC doesn't have envelope
    }
    
    pub fn step_length_counter(&mut self) {
        // DMC doesn't have length counter
    }
    
    pub fn write_register(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        match address {
            0x4010 => {
                // Rate and flags
                self.irq_enabled = (value & 0x80) != 0;
                self.loop_flag = (value & 0x40) != 0;
                self.rate_index = value & 0x0F;
                self.timer_period = DMC_RATE_TABLE[self.rate_index as usize];
                Ok(())
            }
            0x4011 => {
                // Direct load
                self.output_level = value & 0x7F;
                Ok(())
            }
            0x4012 => {
                // Sample address
                self.sample_address = 0xC000 + ((value as u16) << 6);
                Ok(())
            }
            0x4013 => {
                // Sample length
                self.sample_length = ((value as u16) << 4) + 1;
                Ok(())
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    pub fn get_sample(&self) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        
        self.output_level as f32 / 127.0
    }
}

// Lookup tables
const LENGTH_TABLE: [u8; 32] = [
    10, 254, 20,  2, 40,  4, 80,  6, 160,  8, 60, 10, 14, 12, 26, 14,
    12,  16, 24, 18, 48, 20, 96, 22, 192, 24, 72, 26, 16, 28, 32, 30,
];

const NOISE_PERIOD_TABLE: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

const DMC_RATE_TABLE: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

impl Default for PulseChannel {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Default for TriangleChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for NoiseChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DmcChannel {
    fn default() -> Self {
        Self::new()
    }
}
