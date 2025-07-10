use crate::error::{RomError, RomResult};
use crate::rom::ines::InesHeader;

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
    /// Load a ROM from NES file format using iNES header
    pub fn new(rom_data: &[u8]) -> RomResult<Rom> {
        let header = InesHeader::parse(rom_data)?;
        
        let prg_rom_size = header.get_prg_rom_size_bytes();
        let chr_rom_size = header.get_chr_rom_size_bytes();
        let trainer_size = header.get_trainer_size();
        
        // Calculate data offsets
        let prg_rom_start = 16 + trainer_size;
        let chr_rom_start = prg_rom_start + prg_rom_size;
        
        // Validate file size
        let expected_size = chr_rom_start + chr_rom_size;
        if rom_data.len() < expected_size {
            return Err(RomError::FileTooSmall {
                expected: expected_size,
                actual: rom_data.len(),
            });
        }
        
        // Validate PRG ROM size
        if prg_rom_size == 0 {
            return Err(RomError::MissingData("PRG ROM".to_string()));
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
        
        Ok(Rom {
            prg_rom,
            chr_rom,
            mirror_mode: header.get_mirror_mode(),
            mapper: header.get_mapper_number(),
            prg_ram_size: header.get_prg_ram_size_bytes(),
        })
    }
    
    /// Load ROM from file path
    pub fn from_file(path: &str) -> RomResult<Rom> {
        let rom_data = std::fs::read(path)
            .map_err(|e| RomError::Io(format!("Failed to read ROM file '{}': {}", path, e)))?;
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