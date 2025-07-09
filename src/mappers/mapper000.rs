use crate::mappers::mapper::Mapper;
use crate::rom::MirrorMode;
use crate::error::MemoryResult;
use crate::mappers::macros::{impl_no_irq_mapper, impl_chr_access, impl_prg_mirror};

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
    
    impl_prg_mirror!();
}

impl_no_irq_mapper!(Mapper000);
impl_chr_access!(Mapper000, chr_banks);

impl Mapper for Mapper000 {
    fn map_prg_read(&self, address: u16) -> MemoryResult<Option<u32>> {
        match address {
            0x8000..=0xFFFF => {
                if self.prg_banks == 1 {
                    // 16KB PRG ROM - mirror it
                    Ok(Some(Self::mirror_prg_16k(address)))
                } else {
                    // 32KB PRG ROM
                    Ok(Some((address - 0x8000) as u32))
                }
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
            0x0000..=0x1FFF => self.chr_write_common(address, address as u32),
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
        self.irq_state()
    }

    fn irq_clear(&mut self) {
        self.irq_clear()
    }

    fn scanline(&mut self) {
        self.scanline()
    }
}
