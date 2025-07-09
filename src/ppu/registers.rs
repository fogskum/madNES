/// PPU Control Register ($2000)
pub struct PpuControl(pub u8);

impl PpuControl {
    pub fn nametable_x(&self) -> u8 { self.0 & 0x01 }
    pub fn nametable_y(&self) -> u8 { (self.0 & 0x02) >> 1 }
    pub fn vram_increment(&self) -> u8 { (self.0 & 0x04) >> 2 }
    pub fn sprite_pattern_table(&self) -> u8 { (self.0 & 0x08) >> 3 }
    pub fn background_pattern_table(&self) -> u8 { (self.0 & 0x10) >> 4 }
    pub fn sprite_size(&self) -> u8 { (self.0 & 0x20) >> 5 }
    pub fn master_slave(&self) -> u8 { (self.0 & 0x40) >> 6 }
    pub fn nmi_enable(&self) -> bool { (self.0 & 0x80) != 0 }
}

/// PPU Mask Register ($2001)
pub struct PpuMask(pub u8);

impl PpuMask {
    pub fn grayscale(&self) -> bool { (self.0 & 0x01) != 0 }
    pub fn show_background_left(&self) -> bool { (self.0 & 0x02) != 0 }
    pub fn show_sprites_left(&self) -> bool { (self.0 & 0x04) != 0 }
    pub fn show_background(&self) -> bool { (self.0 & 0x08) != 0 }
    pub fn show_sprites(&self) -> bool { (self.0 & 0x10) != 0 }
    pub fn emphasize_red(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn emphasize_green(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn emphasize_blue(&self) -> bool { (self.0 & 0x80) != 0 }
}

/// PPU Status Register ($2002)
pub struct PpuStatus(pub u8);

impl PpuStatus {
    pub fn sprite_overflow(&self) -> bool { (self.0 & 0x20) != 0 }
    pub fn sprite_zero_hit(&self) -> bool { (self.0 & 0x40) != 0 }
    pub fn vblank(&self) -> bool { (self.0 & 0x80) != 0 }
    
    pub fn set_sprite_overflow(&mut self, value: bool) {
        if value {
            self.0 |= 0x20;
        } else {
            self.0 &= !0x20;
        }
    }
    
    pub fn set_sprite_zero_hit(&mut self, value: bool) {
        if value {
            self.0 |= 0x40;
        } else {
            self.0 &= !0x40;
        }
    }
    
    pub fn set_vblank(&mut self, value: bool) {
        if value {
            self.0 |= 0x80;
        } else {
            self.0 &= !0x80;
        }
    }
}
