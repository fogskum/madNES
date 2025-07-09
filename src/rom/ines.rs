use crate::error::{RomError, RomResult};
use crate::rom::MirrorMode;

/// iNES header format for NES ROM files
#[derive(Debug, Clone)]
pub struct InesHeader {
    pub prg_rom_size: u8,      // Size of PRG ROM in 16KB units
    pub chr_rom_size: u8,      // Size of CHR ROM in 8KB units
    pub flags6: u8,            // Mapper, mirroring, battery, trainer
    pub flags7: u8,            // Mapper, VS/Playchoice, NES 2.0
    pub prg_ram_size: u8,      // Size of PRG RAM in 8KB units (rare)
    pub flags9: u8,            // TV system (rare)
    pub flags10: u8,           // TV system, PRG RAM presence (unofficial)
    pub padding: [u8; 5],      // Unused padding
}

impl InesHeader {
    pub fn parse(data: &[u8]) -> RomResult<Self> {
        if data.len() < 16 {
            return Err(RomError::FileTooSmall { 
                expected: 16, 
                actual: data.len() 
            });
        }
        
        // Check for iNES signature
        if &data[0..4] != b"NES\x1A" {
            return Err(RomError::InvalidHeader(
                "Missing or invalid iNES header signature".to_string()
            ));
        }
        
        Ok(Self {
            prg_rom_size: data[4],
            chr_rom_size: data[5],
            flags6: data[6],
            flags7: data[7],
            prg_ram_size: data[8],
            flags9: data[9],
            flags10: data[10],
            padding: [data[11], data[12], data[13], data[14], data[15]],
        })
    }
    
    pub fn get_mapper_number(&self) -> u8 {
        (self.flags7 & 0xF0) | (self.flags6 >> 4)
    }
    
    pub fn get_mirror_mode(&self) -> MirrorMode {
        if self.flags6 & 0x08 != 0 {
            MirrorMode::FourScreen
        } else if self.flags6 & 0x01 != 0 {
            MirrorMode::Vertical
        } else {
            MirrorMode::Horizontal
        }
    }
    
    pub fn has_battery(&self) -> bool {
        (self.flags6 & 0x02) != 0
    }
    
    pub fn has_trainer(&self) -> bool {
        (self.flags6 & 0x04) != 0
    }
    
    pub fn is_vs_unisystem(&self) -> bool {
        (self.flags7 & 0x01) != 0
    }
    
    pub fn is_playchoice_10(&self) -> bool {
        (self.flags7 & 0x02) != 0
    }
    
    pub fn is_nes2(&self) -> bool {
        (self.flags7 & 0x0C) == 0x08
    }
    
    pub fn get_prg_rom_size_bytes(&self) -> usize {
        self.prg_rom_size as usize * 16 * 1024
    }
    
    pub fn get_chr_rom_size_bytes(&self) -> usize {
        self.chr_rom_size as usize * 8 * 1024
    }
    
    pub fn get_prg_ram_size_bytes(&self) -> usize {
        if self.prg_ram_size == 0 {
            8 * 1024 // Default 8KB if not specified
        } else {
            self.prg_ram_size as usize * 8 * 1024
        }
    }
    
    pub fn get_trainer_size(&self) -> usize {
        if self.has_trainer() { 512 } else { 0 }
    }
}
