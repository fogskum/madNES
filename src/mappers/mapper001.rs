use crate::mappers::mapper::Mapper;
use crate::rom::MirrorMode;
use crate::error::MemoryResult;

/// Mapper 001 - MMC1 (SxROM)
pub struct Mapper001 {
    prg_banks: u8,
    chr_banks: u8,
    mirror_mode: MirrorMode,
    
    // MMC1 registers
    shift_register: u8,
    shift_count: u8,
    control_register: u8,
    chr_bank_0: u8,
    chr_bank_1: u8,
    prg_bank: u8,
    
    // PRG RAM
    prg_ram_enabled: bool,
}

impl Mapper001 {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Self {
            prg_banks,
            chr_banks,
            mirror_mode: MirrorMode::Horizontal,
            shift_register: 0,
            shift_count: 0,
            control_register: 0x0C, // Default: 16KB PRG mode, 4KB CHR mode
            chr_bank_0: 0,
            chr_bank_1: 0,
            prg_bank: 0,
            prg_ram_enabled: true,
        }
    }
    
    fn write_register(&mut self, address: u16, value: u8) -> MemoryResult<()> {
        if (value & 0x80) != 0 {
            // Reset shift register
            self.shift_register = 0;
            self.shift_count = 0;
            self.control_register |= 0x0C;
            return Ok(());
        }
        
        // Shift in the new bit
        self.shift_register >>= 1;
        self.shift_register |= (value & 0x01) << 4;
        self.shift_count += 1;
        
        if self.shift_count == 5 {
            // Write to the target register
            match address {
                0x8000..=0x9FFF => {
                    // Control register
                    self.control_register = self.shift_register;
                    self.mirror_mode = match self.control_register & 0x03 {
                        0 => MirrorMode::Horizontal, // Single screen lower
                        1 => MirrorMode::Horizontal, // Single screen upper
                        2 => MirrorMode::Vertical,
                        3 => MirrorMode::Horizontal,
                        _ => MirrorMode::Horizontal,
                    };
                }
                0xA000..=0xBFFF => {
                    // CHR bank 0
                    self.chr_bank_0 = self.shift_register;
                }
                0xC000..=0xDFFF => {
                    // CHR bank 1
                    self.chr_bank_1 = self.shift_register;
                }
                0xE000..=0xFFFF => {
                    // PRG bank
                    self.prg_bank = self.shift_register & 0x0F;
                    self.prg_ram_enabled = (self.shift_register & 0x10) == 0;
                }
                _ => {}
            }
            
            // Reset shift register
            self.shift_register = 0;
            self.shift_count = 0;
        }
        
        Ok(())
    }
}

impl Mapper for Mapper001 {
    fn map_prg_read(&self, address: u16) -> MemoryResult<Option<u32>> {
        match address {
            0x6000..=0x7FFF => {
                // PRG RAM
                if self.prg_ram_enabled {
                    Ok(Some((address - 0x6000) as u32))
                } else {
                    Ok(None)
                }
            }
            0x8000..=0xFFFF => {
                let prg_mode = (self.control_register >> 2) & 0x03;
                let bank_size = 0x4000; // 16KB
                
                match prg_mode {
                    0 | 1 => {
                        // 32KB mode
                        let bank = (self.prg_bank & 0x0E) >> 1;
                        let addr = (address - 0x8000) + (bank as u16 * bank_size);
                        Ok(Some(addr as u32))
                    }
                    2 => {
                        // First bank fixed to 0, second bank switchable
                        if address < 0xC000 {
                            let addr = address - 0x8000;
                            Ok(Some(addr as u32))
                        } else {
                            let addr = (address - 0xC000) + (self.prg_bank as u16 * bank_size);
                            Ok(Some(addr as u32))
                        }
                    }
                    3 => {
                        // First bank switchable, last bank fixed
                        if address < 0xC000 {
                            let addr = (address - 0x8000) + (self.prg_bank as u16 * bank_size);
                            Ok(Some(addr as u32))
                        } else {
                            let last_bank = self.prg_banks - 1;
                            let addr = (address - 0xC000) + (last_bank as u16 * bank_size);
                            Ok(Some(addr as u32))
                        }
                    }
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }
    
    fn map_prg_write(&mut self, address: u16, value: u8) -> MemoryResult<Option<u32>> {
        match address {
            0x6000..=0x7FFF => {
                // PRG RAM
                if self.prg_ram_enabled {
                    Ok(Some((address - 0x6000) as u32))
                } else {
                    Ok(None)
                }
            }
            0x8000..=0xFFFF => {
                // Mapper register write
                self.write_register(address, value)?;
                Ok(None)
            }
            _ => Ok(None),
        }
    }
    
    fn map_chr_read(&self, address: u16) -> MemoryResult<Option<u32>> {
        match address {
            0x0000..=0x1FFF => {
                let chr_mode = (self.control_register >> 4) & 0x01;
                
                if chr_mode == 0 {
                    // 8KB mode
                    let bank = (self.chr_bank_0 & 0x1E) >> 1;
                    let addr = address + (bank as u16 * 0x2000);
                    Ok(Some(addr as u32))
                } else {
                    // 4KB mode
                    if address < 0x1000 {
                        let addr = address + (self.chr_bank_0 as u16 * 0x1000);
                        Ok(Some(addr as u32))
                    } else {
                        let addr = (address - 0x1000) + (self.chr_bank_1 as u16 * 0x1000);
                        Ok(Some(addr as u32))
                    }
                }
            }
            _ => Ok(None),
        }
    }
    
    fn map_chr_write(&mut self, address: u16, _value: u8) -> MemoryResult<Option<u32>> {
        match address {
            0x0000..=0x1FFF => {
                if self.chr_banks == 0 {
                    // CHR RAM - allow writes
                    let chr_mode = (self.control_register >> 4) & 0x01;
                    
                    if chr_mode == 0 {
                        // 8KB mode
                        let bank = (self.chr_bank_0 & 0x1E) >> 1;
                        let addr = address + (bank as u16 * 0x2000);
                        Ok(Some(addr as u32))
                    } else {
                        // 4KB mode
                        if address < 0x1000 {
                            let addr = address + (self.chr_bank_0 as u16 * 0x1000);
                            Ok(Some(addr as u32))
                        } else {
                            let addr = (address - 0x1000) + (self.chr_bank_1 as u16 * 0x1000);
                            Ok(Some(addr as u32))
                        }
                    }
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
        self.shift_register = 0;
        self.shift_count = 0;
        self.control_register = 0x0C;
        self.chr_bank_0 = 0;
        self.chr_bank_1 = 0;
        self.prg_bank = 0;
        self.prg_ram_enabled = true;
        self.mirror_mode = MirrorMode::Horizontal;
    }
    
    fn irq_state(&self) -> bool {
        false
    }
    
    fn irq_clear(&mut self) {
        // MMC1 doesn't generate IRQs
    }
    
    fn scanline(&mut self) {
        // MMC1 doesn't use scanline counter
    }
}
