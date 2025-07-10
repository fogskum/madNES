use crate::error::{MemoryError, MemoryResult};

/// PPU (Picture Processing Unit) - Handles graphics rendering
pub struct Ppu {
    // PPU registers
    pub ctrl: u8,           // $2000
    pub mask: u8,           // $2001
    pub status: u8,         // $2002
    pub oam_addr: u8,       // $2003
    pub oam_data: u8,       // $2004
    pub scroll: u8,         // $2005
    pub addr: u8,           // $2006
    pub data: u8,           // $2007
    
    // PPU memory
    pub pattern_table: [u8; 0x2000],    // $0000-$1FFF
    pub name_table: [u8; 0x1000],       // $2000-$2FFF
    pub palette: [u8; 0x20],            // $3F00-$3F1F
    pub oam: [u8; 0x100],               // Object Attribute Memory
    
    // Internal state
    pub cycle: u16,
    pub scanline: u16,
    pub frame: u64,
    pub nmi_occurred: bool,
    pub nmi_output: bool,
    pub nmi_previous: bool,
    pub nmi_delay: u8,
    
    // Registers
    pub v: u16,             // Current VRAM address
    pub t: u16,             // Temporary VRAM address
    pub x: u8,              // Fine X scroll
    pub w: bool,            // Write toggle
    pub f: bool,            // Odd frame flag
    
    // Background rendering
    pub name_table_byte: u8,
    pub attribute_table_byte: u8,
    pub low_tile_byte: u8,
    pub high_tile_byte: u8,
    pub tile_data: u64,
    
    // Sprite rendering
    pub sprite_count: u8,
    pub sprite_patterns: [u32; 8],
    pub sprite_positions: [u8; 8],
    pub sprite_priorities: [u8; 8],
    pub sprite_indexes: [u8; 8],
    
    // Buffers
    pub front_buffer: [u8; 256 * 240 * 3],
    pub back_buffer: [u8; 256 * 240 * 3],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ctrl: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: 0,
            data: 0,
            pattern_table: [0; 0x2000],
            name_table: [0; 0x1000],
            palette: [0; 0x20],
            oam: [0; 0x100],
            cycle: 0,
            scanline: 0,
            frame: 0,
            nmi_occurred: false,
            nmi_output: false,
            nmi_previous: false,
            nmi_delay: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            f: false,
            name_table_byte: 0,
            attribute_table_byte: 0,
            low_tile_byte: 0,
            high_tile_byte: 0,
            tile_data: 0,
            sprite_count: 0,
            sprite_patterns: [0; 8],
            sprite_positions: [0; 8],
            sprite_priorities: [0; 8],
            sprite_indexes: [0; 8],
            front_buffer: [0; 256 * 240 * 3],
            back_buffer: [0; 256 * 240 * 3],
        }
    }
    
    pub fn reset(&mut self) {
        self.cycle = 0;
        self.scanline = 0;
        self.frame = 0;
        self.ctrl = 0;
        self.mask = 0;
        self.status = 0;
        self.oam_addr = 0;
        self.scroll = 0;
        self.addr = 0;
        self.data = 0;
        self.v = 0;
        self.t = 0;
        self.x = 0;
        self.w = false;
        self.f = false;
        self.nmi_occurred = false;
        self.nmi_output = false;
        self.nmi_previous = false;
        self.nmi_delay = 0;
    }
    
    pub fn step(&mut self) -> MemoryResult<bool> {
        // Increment cycle
        self.cycle += 1;
        
        // Check if we've completed a scanline
        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            
            // Check if we've completed a frame
            if self.scanline > 261 {
                self.scanline = 0;
                self.frame += 1;
                self.f = !self.f;
                return Ok(true); // Frame completed
            }
        }
        
        // Handle NMI
        if self.nmi_delay > 0 {
            self.nmi_delay -= 1;
            if self.nmi_delay == 0 && self.nmi_output && self.nmi_occurred {
                // NMI should be triggered
            }
        }
        
        Ok(false)
    }
    
    pub fn read_register(&mut self, address: u16) -> MemoryResult<u8> {
        match address {
            0x2002 => {
                // Status register
                let result = self.status;
                self.status &= 0x7F; // Clear VBlank flag
                self.w = false; // Reset write toggle
                Ok(result)
            }
            0x2004 => Ok(self.oam[self.oam_addr as usize]),
            0x2007 => {
                // Data register
                let result = self.data;
                self.data = self.read_memory(self.v)?;
                if self.v < 0x3F00 {
                    self.v += if self.ctrl & 0x04 != 0 { 32 } else { 1 };
                }
                Ok(result)
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    pub fn write_register(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        match address {
            0x2000 => {
                // Control register
                self.ctrl = value;
                self.t = (self.t & 0xF3FF) | ((value as u16 & 0x03) << 10);
                Ok(())
            }
            0x2001 => {
                // Mask register
                self.mask = value;
                Ok(())
            }
            0x2003 => {
                // OAM address register
                self.oam_addr = value;
                Ok(())
            }
            0x2004 => {
                // OAM data register
                self.oam[self.oam_addr as usize] = value;
                self.oam_addr = self.oam_addr.wrapping_add(1);
                Ok(())
            }
            0x2005 => {
                // Scroll register
                if !self.w {
                    // First write (X scroll)
                    self.t = (self.t & 0xFFE0) | ((value as u16) >> 3);
                    self.x = value & 0x07;
                    self.w = true;
                } else {
                    // Second write (Y scroll)
                    self.t = (self.t & 0x8FFF) | (((value as u16) & 0x07) << 12);
                    self.t = (self.t & 0xFC1F) | (((value as u16) & 0xF8) << 2);
                    self.w = false;
                }
                Ok(())
            }
            0x2006 => {
                // Address register
                if !self.w {
                    // First write (high byte)
                    self.t = (self.t & 0x80FF) | (((value as u16) & 0x3F) << 8);
                    self.w = true;
                } else {
                    // Second write (low byte)
                    self.t = (self.t & 0xFF00) | (value as u16);
                    self.v = self.t;
                    self.w = false;
                }
                Ok(())
            }
            0x2007 => {
                // Data register
                self.write_memory(self.v, value)?;
                self.v += if self.ctrl & 0x04 != 0 { 32 } else { 1 };
                Ok(())
            }
            _ => Err(MemoryError::InvalidRegion(address)),
        }
    }
    
    fn read_memory(&self, address: u16) -> MemoryResult<u8> {
        match address {
            0x0000..=0x1FFF => Ok(self.pattern_table[address as usize]),
            0x2000..=0x2FFF => Ok(self.name_table[(address - 0x2000) as usize]),
            0x3000..=0x3EFF => Ok(self.name_table[(address - 0x3000) as usize]),
            0x3F00..=0x3F1F => Ok(self.palette[(address - 0x3F00) as usize]),
            0x3F20..=0x3FFF => Ok(self.palette[(address - 0x3F20) as usize]),
            _ => Err(MemoryError::OutOfBounds { address, size: 0x4000 }),
        }
    }
    
    fn write_memory(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        match address {
            0x0000..=0x1FFF => {
                // CHR ROM is usually read-only, but some mappers allow writing
                self.pattern_table[address as usize] = value;
                Ok(())
            }
            0x2000..=0x2FFF => {
                self.name_table[(address - 0x2000) as usize] = value;
                Ok(())
            }
            0x3000..=0x3EFF => {
                self.name_table[(address - 0x3000) as usize] = value;
                Ok(())
            }
            0x3F00..=0x3F1F => {
                self.palette[(address - 0x3F00) as usize] = value;
                Ok(())
            }
            0x3F20..=0x3FFF => {
                self.palette[(address - 0x3F20) as usize] = value;
                Ok(())
            }
            _ => Err(MemoryError::OutOfBounds { address, size: 0x4000 }),
        }
    }
    
    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.front_buffer
    }
    
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.front_buffer, &mut self.back_buffer);
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}
