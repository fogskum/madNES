use crate::mappers::mapper::Mapper;
use crate::rom::MirrorMode;
use crate::error::MemoryResult;

/// Mapper 000 - NROM (No mapper)
pub struct Mapper000 {
    prg_banks: u8,
    chr_banks: u8,
    mirror_mode: MirrorMode,
}

impl Mapper000 {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Self {
            prg_banks,
            chr_banks,
            mirror_mode: MirrorMode::Horizontal,
        }
    }
    
    pub fn set_mirror_mode(&mut self, mode: MirrorMode) {
        self.mirror_mode = mode;
    }
}

impl Mapper for Mapper000 {
    fn map_prg_read(&self, address: u16) -> MemoryResult<Option<u32>> {
        match address {
            0x8000..=0xFFFF => {
                let mut addr = address - 0x8000;
                if self.prg_banks == 1 {
                    // 16KB PRG ROM - mirror it
                    addr &= 0x3FFF;
                }
                Ok(Some(addr as u32))
            }
            _ => Ok(None),
        }
    }
    
    fn map_prg_write(&mut self, address: u16, _value: u8) -> MemoryResult<Option<u32>> {
        match address {
            0x8000..=0xFFFF => {
                // PRG ROM is read-only in NROM
                Ok(None)
            }
            _ => Ok(None),
        }
    }
    
    fn map_chr_read(&self, address: u16) -> MemoryResult<Option<u32>> {
        match address {
            0x0000..=0x1FFF => Ok(Some(address as u32)),
            _ => Ok(None),
        }
    }
    
    fn map_chr_write(&mut self, address: u16, _value: u8) -> MemoryResult<Option<u32>> {
        match address {
            0x0000..=0x1FFF => {
                if self.chr_banks == 0 {
                    // CHR RAM - allow writes
                    Ok(Some(address as u32))
                } else {
                    // CHR ROM - read-only
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
    
    fn get_mirror_mode(&self) -> MirrorMode {
        self.mirror_mode.clone()
    }
    
    fn reset(&mut self) {
        // NROM has no state to reset
    }
    
    fn irq_state(&self) -> bool {
        false
    }
    
    fn irq_clear(&mut self) {
        // NROM doesn't generate IRQs
    }
    
    fn scanline(&mut self) {
        // NROM doesn't use scanline counter
    }
}
