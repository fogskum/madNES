use std::time::{Duration, Instant};

/// NES timing constants
pub const NES_CPU_FREQUENCY: u32 = 1_789_773; // Hz (NTSC)
pub const NES_PPU_FREQUENCY: u32 = NES_CPU_FREQUENCY * 3;
pub const NES_FRAME_RATE: f64 = 60.098814;
pub const CYCLES_PER_FRAME: u32 = NES_CPU_FREQUENCY / 60;

/// Timer for managing emulation timing
pub struct Timer {
    start_time: Instant,
    last_update: Instant,
    target_fps: f64,
    frame_duration: Duration,
}

impl Timer {
    pub fn new(target_fps: f64) -> Self {
        let frame_duration = Duration::from_secs_f64(1.0 / target_fps);
        let now = Instant::now();

        Self {
            start_time: now,
            last_update: now,
            target_fps,
            frame_duration,
        }
    }

    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_update = now;
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn delta_time(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update);
        self.last_update = now;
        delta
    }

    pub fn should_update(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);

        if elapsed >= self.frame_duration {
            self.last_update = now;
            true
        } else {
            false
        }
    }

    pub fn sleep_until_next_frame(&self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);

        if elapsed < self.frame_duration {
            let sleep_time = self.frame_duration - elapsed;
            std::thread::sleep(sleep_time);
        }
    }

    pub fn get_fps(&self) -> f64 {
        self.target_fps
    }

    pub fn set_fps(&mut self, fps: f64) {
        self.target_fps = fps;
        self.frame_duration = Duration::from_secs_f64(1.0 / fps);
    }
}

/// Cycle counter for precise emulation timing
pub struct CycleCounter {
    cpu_cycles: u64,
    ppu_cycles: u64,
    apu_cycles: u64,
    master_cycles: u64,
}

impl CycleCounter {
    pub fn new() -> Self {
        Self {
            cpu_cycles: 0,
            ppu_cycles: 0,
            apu_cycles: 0,
            master_cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cpu_cycles = 0;
        self.ppu_cycles = 0;
        self.apu_cycles = 0;
        self.master_cycles = 0;
    }

    pub fn step_cpu(&mut self, cycles: u64) {
        self.cpu_cycles += cycles;
        self.master_cycles += cycles;
    }

    pub fn step_ppu(&mut self, cycles: u64) {
        self.ppu_cycles += cycles;
    }

    pub fn step_apu(&mut self, cycles: u64) {
        self.apu_cycles += cycles;
    }

    pub fn get_cpu_cycles(&self) -> u64 {
        self.cpu_cycles
    }

    pub fn get_ppu_cycles(&self) -> u64 {
        self.ppu_cycles
    }

    pub fn get_apu_cycles(&self) -> u64 {
        self.apu_cycles
    }

    pub fn get_master_cycles(&self) -> u64 {
        self.master_cycles
    }

    /// Calculate how many PPU cycles should have elapsed for the given CPU cycles
    pub fn cpu_to_ppu_cycles(&self, cpu_cycles: u64) -> u64 {
        cpu_cycles * 3
    }

    /// Check if enough cycles have passed for a frame
    pub fn frame_complete(&self) -> bool {
        self.cpu_cycles >= CYCLES_PER_FRAME as u64
    }
}

impl Default for CycleCounter {
    fn default() -> Self {
        Self::new()
    }
}
