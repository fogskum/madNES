use crate::error::{EmulatorError, SdlError};
use crate::audio::audio_buffer::AudioBuffer;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

/// Audio manager for NES sound output
pub struct AudioManager {
    device: Option<AudioDevice<NesAudioCallback>>,
    sample_rate: u32,
    buffer_size: usize,
}

impl AudioManager {
    pub fn new(sample_rate: u32, buffer_size: usize) -> Self {
        Self {
            device: None,
            sample_rate,
            buffer_size,
        }
    }
    
    pub fn initialize(&mut self, audio_subsystem: &sdl2::AudioSubsystem) -> Result<(), EmulatorError> {
        let desired_spec = AudioSpecDesired {
            freq: Some(self.sample_rate as i32),
            channels: Some(1), // Mono
            samples: Some(self.buffer_size as u16),
        };
        
        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                NesAudioCallback::new(spec.freq as u32, self.buffer_size)
            })
            .map_err(|e| EmulatorError::Sdl(SdlError::AudioInitializationFailed(e)))?;
        
        self.device = Some(device);
        Ok(())
    }
    
    pub fn start(&mut self) -> Result<(), EmulatorError> {
        if let Some(ref device) = self.device {
            device.resume();
        }
        Ok(())
    }
    
    pub fn stop(&mut self) -> Result<(), EmulatorError> {
        if let Some(ref device) = self.device {
            device.pause();
        }
        Ok(())
    }
    
    pub fn add_sample(&mut self, sample: f32) -> Result<(), EmulatorError> {
        if let Some(ref mut device) = self.device {
            let mut lock = device.lock();
            lock.add_sample(sample);
        }
        Ok(())
    }
    
    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }
    
    pub fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }
}

struct NesAudioCallback {
    buffer: AudioBuffer,
    _sample_rate: u32,
}

impl NesAudioCallback {
    fn new(sample_rate: u32, buffer_size: usize) -> Self {
        Self {
            buffer: AudioBuffer::new(buffer_size),
            _sample_rate: sample_rate,
        }
    }
    
    fn add_sample(&mut self, sample: f32) {
        self.buffer.push(sample);
    }
}

impl AudioCallback for NesAudioCallback {
    type Channel = f32;
    
    fn callback(&mut self, out: &mut [f32]) {
        for sample in out.iter_mut() {
            *sample = self.buffer.pop().unwrap_or(0.0);
        }
    }
}
