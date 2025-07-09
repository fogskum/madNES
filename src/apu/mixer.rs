/// Audio mixer for combining APU channel outputs
pub struct Mixer {
    pub pulse_table: [f32; 31],
    pub tnd_table: [f32; 203],
}

impl Mixer {
    pub fn new() -> Self {
        let mut mixer = Self {
            pulse_table: [0.0; 31],
            tnd_table: [0.0; 203],
        };
        
        // Initialize lookup tables
        for i in 0..31 {
            mixer.pulse_table[i] = if i == 0 {
                0.0
            } else {
                95.52 / (8128.0 / (i as f32) + 100.0)
            };
        }
        
        for i in 0..203 {
            mixer.tnd_table[i] = if i == 0 {
                0.0
            } else {
                163.67 / (24329.0 / (i as f32) + 100.0)
            };
        }
        
        mixer
    }
    
    pub fn mix(&self, pulse1: f32, pulse2: f32, triangle: f32, noise: f32, dmc: f32) -> f32 {
        let pulse_index = ((pulse1 + pulse2) * 15.0) as usize;
        let pulse_out = if pulse_index < 31 {
            self.pulse_table[pulse_index]
        } else {
            self.pulse_table[30]
        };
        
        let tnd_index = ((triangle * 3.0 + noise * 2.0 + dmc) * 15.0) as usize;
        let tnd_out = if tnd_index < 203 {
            self.tnd_table[tnd_index]
        } else {
            self.tnd_table[202]
        };
        
        pulse_out + tnd_out
    }
}

impl Default for Mixer {
    fn default() -> Self {
        Self::new()
    }
}
