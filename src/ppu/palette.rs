/// NES color palette
pub struct Palette {
    data: [u8; 32],
}

impl Palette {
    pub fn new() -> Self {
        Self { data: [0; 32] }
    }

    pub fn read(&self, address: u8) -> u8 {
        let addr = address & 0x1F;
        // Handle palette mirroring
        match addr {
            0x10 => self.data[0x00],
            0x14 => self.data[0x04],
            0x18 => self.data[0x08],
            0x1C => self.data[0x0C],
            _ => self.data[addr as usize],
        }
    }

    pub fn write(&mut self, address: u8, value: u8) {
        let addr = address & 0x1F;
        // Handle palette mirroring
        match addr {
            0x10 => self.data[0x00] = value,
            0x14 => self.data[0x04] = value,
            0x18 => self.data[0x08] = value,
            0x1C => self.data[0x0C] = value,
            _ => self.data[addr as usize] = value,
        }
    }

    pub fn get_rgb(&self, palette_index: u8, color_index: u8) -> (u8, u8, u8) {
        let index = self.read(palette_index * 4 + color_index);
        self.system_palette_to_rgb(index)
    }

    fn system_palette_to_rgb(&self, index: u8) -> (u8, u8, u8) {
        // NES system palette (simplified)
        match index & 0x3F {
            0x00 => (84, 84, 84),
            0x01 => (0, 30, 116),
            0x02 => (8, 16, 144),
            0x03 => (48, 0, 136),
            0x04 => (68, 0, 100),
            0x05 => (92, 0, 48),
            0x06 => (84, 4, 0),
            0x07 => (60, 24, 0),
            0x08 => (32, 42, 0),
            0x09 => (8, 58, 0),
            0x0A => (0, 64, 0),
            0x0B => (0, 60, 0),
            0x0C => (0, 50, 60),
            0x0D => (0, 0, 0),
            0x0E => (0, 0, 0),
            0x0F => (0, 0, 0),

            0x10 => (152, 150, 152),
            0x11 => (8, 76, 196),
            0x12 => (48, 50, 236),
            0x13 => (92, 30, 228),
            0x14 => (136, 20, 176),
            0x15 => (160, 20, 100),
            0x16 => (152, 34, 32),
            0x17 => (120, 60, 0),
            0x18 => (84, 90, 0),
            0x19 => (40, 114, 0),
            0x1A => (8, 124, 0),
            0x1B => (0, 118, 40),
            0x1C => (0, 102, 120),
            0x1D => (0, 0, 0),
            0x1E => (0, 0, 0),
            0x1F => (0, 0, 0),

            0x20 => (236, 238, 236),
            0x21 => (76, 154, 236),
            0x22 => (120, 124, 236),
            0x23 => (176, 98, 236),
            0x24 => (228, 84, 236),
            0x25 => (236, 88, 180),
            0x26 => (236, 106, 100),
            0x27 => (212, 136, 32),
            0x28 => (160, 170, 0),
            0x29 => (116, 196, 0),
            0x2A => (76, 208, 32),
            0x2B => (56, 204, 108),
            0x2C => (56, 180, 204),
            0x2D => (60, 60, 60),
            0x2E => (0, 0, 0),
            0x2F => (0, 0, 0),

            0x30 => (236, 238, 236),
            0x31 => (168, 204, 236),
            0x32 => (188, 188, 236),
            0x33 => (212, 178, 236),
            0x34 => (236, 174, 236),
            0x35 => (236, 174, 212),
            0x36 => (236, 180, 176),
            0x37 => (228, 196, 144),
            0x38 => (204, 210, 120),
            0x39 => (180, 222, 120),
            0x3A => (168, 226, 144),
            0x3B => (152, 226, 180),
            0x3C => (160, 214, 228),
            0x3D => (160, 162, 160),
            0x3E => (0, 0, 0),
            0x3F => (0, 0, 0),

            _ => (0, 0, 0),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}
