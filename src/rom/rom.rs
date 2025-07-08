#[derive(Debug, Clone)]
pub enum MirrorMode {
    Horizontal,
    Vertical,
    FourScreen,
}

#[derive(Debug, Clone)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mirror_mode: MirrorMode,
    pub mapper: u8,
    pub prg_ram_size: usize,
}

impl Rom {
    /// Load a ROM from NES file format
    /// NES file format:
    /// Bytes 0-3: "NES\x1A" header
    /// Byte 4: PRG ROM size in 16KB units
    /// Byte 5: CHR ROM size in 8KB units
    /// Byte 6: Flags 6 - Mapper, mirroring, battery, trainer
    /// Byte 7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
    /// Byte 8: PRG RAM size (rarely used)
    /// Byte 9: TV system (rarely used)
    /// Bytes 10-15: Unused padding
    /// Trainer (512 bytes, if present)
    /// PRG ROM data
    /// CHR ROM data
    pub fn new(rom_data: &[u8]) -> Result<Rom, String> {
        if rom_data.len() < 16 {
            return Err("ROM file too small".to_string());
        }
        
        // Check NES header
        if &rom_data[0..4] != b"NES\x1A" {
            return Err("Invalid NES ROM header".to_string());
        }

        let prg_rom_size = rom_data[4] as usize * 16 * 1024; // 16KB units
        let chr_rom_size = rom_data[5] as usize * 8 * 1024;  // 8KB units
        
        let flags6 = rom_data[6];
        let flags7 = rom_data[7];
        
        // Extract mirroring mode
        let mirror_mode = if flags6 & 0x08 != 0 {
            MirrorMode::FourScreen
        } else if flags6 & 0x01 != 0 {
            MirrorMode::Vertical
        } else {
            MirrorMode::Horizontal
        };
        
        // Extract mapper number
        let mapper = (flags7 & 0xF0) | (flags6 >> 4);
        
        // Check for trainer (512 bytes before PRG ROM)
        let has_trainer = flags6 & 0x04 != 0;
        let trainer_size = if has_trainer { 512 } else { 0 };
        
        // Calculate data offsets
        let prg_rom_start = 16 + trainer_size;
        let chr_rom_start = prg_rom_start + prg_rom_size;
        
        // Validate file size
        let expected_size = chr_rom_start + chr_rom_size;
        if rom_data.len() < expected_size {
            return Err(format!(
                "ROM file truncated: expected {} bytes, got {}",
                expected_size,
                rom_data.len()
            ));
        }
        
        // Extract PRG ROM
        let prg_rom = rom_data[prg_rom_start..prg_rom_start + prg_rom_size].to_vec();
        
        // Extract CHR ROM
        let chr_rom = if chr_rom_size > 0 {
            rom_data[chr_rom_start..chr_rom_start + chr_rom_size].to_vec()
        } else {
            // Some games use CHR RAM instead of CHR ROM
            vec![0; 8 * 1024] // 8KB CHR RAM
        };
        
        // PRG RAM size (usually 8KB if not specified)
        let prg_ram_size = if rom_data[8] == 0 { 8 * 1024 } else { rom_data[8] as usize * 8 * 1024 };
        
        Ok(Rom {
            prg_rom,
            chr_rom,
            mirror_mode,
            mapper,
            prg_ram_size,
        })
    }
    
    /// Load ROM from file path
    pub fn from_file(path: &str) -> Result<Rom, String> {
        let rom_data = std::fs::read(path)
            .map_err(|e| format!("Failed to read ROM file '{}': {}", path, e))?;
        Self::new(&rom_data)
    }
    
    /// Get PRG ROM data at specified offset
    pub fn read_prg_byte(&self, address: u16) -> u8 {
        let offset = address as usize;
        if offset < self.prg_rom.len() {
            self.prg_rom[offset]
        } else {
            // Handle mirroring for 16KB ROMs (NROM)
            if self.prg_rom.len() == 16 * 1024 && offset >= 16 * 1024 {
                self.prg_rom[offset - 16 * 1024]
            } else {
                0
            }
        }
    }
    
    /// Get CHR ROM data at specified offset
    pub fn read_chr_byte(&self, address: u16) -> u8 {
        let offset = address as usize;
        if offset < self.chr_rom.len() {
            self.chr_rom[offset]
        } else {
            0
        }
    }
    
    /// Write to CHR ROM/RAM (some mappers allow this)
    pub fn write_chr_byte(&mut self, address: u16, value: u8) {
        let offset = address as usize;
        if offset < self.chr_rom.len() {
            self.chr_rom[offset] = value;
        }
    }
}