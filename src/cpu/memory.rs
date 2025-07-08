use crate::rom::rom::Rom;

pub trait Memory {
    fn read_byte(&self, address: u16) -> u8;

    fn write_byte(&mut self, address: u16, value: u8);
    
    fn read_word(&self, address: u16) -> u16 {
        let lo = self.read_byte(address) as u16;
        let hi = self.read_byte(address + 1) as u16;
        (hi << 8) | lo
    }
    
    fn write_word(&mut self, address: u16, value: u16) {
        let lo = value as u8;
        let hi = (value >> 8) as u8;
        self.write_byte(address, lo);
        self.write_byte(address + 1, hi);
    }
}

/// NES Memory Map:
/// $0000-$07FF: Internal RAM (2KB, mirrored 4 times up to $1FFF)
/// $2000-$2007: PPU registers (mirrored up to $3FFF)
/// $4000-$4017: APU and I/O registers
/// $4018-$401F: APU and I/O test mode registers
/// $4020-$FFFF: Cartridge space (PRG ROM, PRG RAM, mapper registers)
pub struct NesMemory {
    // Internal RAM - 2KB, mirrored 4 times
    internal_ram: [u8; 0x800], // $0000-$07FF
    
    // PPU registers - 8 bytes, mirrored throughout $2000-$3FFF
    ppu_registers: [u8; 8], // $2000-$2007
    
    // APU and I/O registers
    apu_io_registers: [u8; 0x20], // $4000-$401F
    
    // Cartridge space
    rom: Option<Rom>, // PRG ROM data
    prg_ram: [u8; 0x2000], // PRG RAM (8KB) at $6000-$7FFF
}

impl NesMemory {
    pub fn new() -> Self {
        Self {
            internal_ram: [0; 0x800],
            ppu_registers: [0; 8],
            apu_io_registers: [0; 0x20],
            rom: None, // No ROM loaded initially
            prg_ram: [0; 0x2000],
        }
    }
    
    pub fn load_prg_rom(&mut self, rom: Rom) {
        self.rom = Some(rom);
    }
    
    /// Get the mirrored address for internal RAM
    fn mirror_internal_ram(&self, address: u16) -> usize {
        (address & 0x07FF) as usize
    }
    
    /// Get the mirrored address for PPU registers
    fn mirror_ppu_registers(&self, address: u16) -> usize {
        (address & 0x0007) as usize
    }
}

impl Memory for NesMemory {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            // Internal RAM (2KB mirrored 4 times)
            0x0000..=0x1FFF => {
                let mirrored_addr = self.mirror_internal_ram(address);
                self.internal_ram[mirrored_addr]
            }
            
            // PPU registers (8 bytes mirrored)
            0x2000..=0x3FFF => {
                let mirrored_addr = self.mirror_ppu_registers(address);
                self.ppu_registers[mirrored_addr]
            }
            
            // APU and I/O registers
            0x4000..=0x4017 => {
                let offset = (address - 0x4000) as usize;
                self.apu_io_registers[offset]
            }
            
            // APU test mode registers
            0x4018..=0x401F => {
                let offset = (address - 0x4018) as usize;
                if offset < self.apu_io_registers.len() {
                    self.apu_io_registers[offset]
                } else {
                    0
                }
            }
            
            // PRG RAM
            0x6000..=0x7FFF => {
                let offset = (address - 0x6000) as usize;
                self.prg_ram[offset]
            }
            
            // PRG ROM
            0x8000..=0xFFFF => {
                let offset = (address - 0x8000) as usize;
                if let Some(rom) = &self.rom {
                    if offset < rom.prg_rom.len() {
                        rom.prg_rom[offset]
                    } else {
                        // Handle case where ROM is smaller than address space
                        // Mirror the ROM if it's 16KB (typical for small games)
                        if rom.prg_rom.len() == 0x4000 && offset >= 0x4000 {
                            rom.prg_rom[offset - 0x4000]
                        } else {
                            0
                        }
                    }
                }
                else {
                    // If no ROM is loaded, return 0 (for testing)
                    0
                }
            }
            
            // Unmapped addresses
            _ => 0
        }
    }
    
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // Internal RAM (2KB mirrored 4 times)
            0x0000..=0x1FFF => {
                let mirrored_addr = self.mirror_internal_ram(address);
                self.internal_ram[mirrored_addr] = value;
            }
            
            // PPU registers (8 bytes mirrored)
            0x2000..=0x3FFF => {
                let mirrored_addr = self.mirror_ppu_registers(address);
                self.ppu_registers[mirrored_addr] = value;
                // TODO: Handle PPU register side effects
            }
            
            // APU and I/O registers
            0x4000..=0x4017 => {
                let offset = (address - 0x4000) as usize;
                self.apu_io_registers[offset] = value;
                // TODO: Handle APU/IO register side effects
            }
            
            // APU test mode registers
            0x4018..=0x401F => {
                let offset = (address - 0x4018) as usize;
                if offset < self.apu_io_registers.len() {
                    self.apu_io_registers[offset] = value;
                }
            }
            
            // PRG RAM
            0x6000..=0x7FFF => {
                let offset = (address - 0x6000) as usize;
                self.prg_ram[offset] = value;
            }
            
            // PRG ROM
            0x8000..=0xFFFF => {
                // ROM is read-only, ignore writes
                // In a real NES, mapper registers might be here
            }
            
            // Unmapped addresses - ignore writes
            _ => {}
        }
    }
}

#[derive(Debug)]
pub enum AddressingMode {
    None,
    Immediate,
    Implied, // same as Accumulator
    IndirectX,
    IndirectY,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
}

use std::fmt;

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddressingMode::None => write!(f, "None"),
            AddressingMode::Immediate => write!(f, "Immediate"),
            AddressingMode::Implied => write!(f, "Implied"),
            AddressingMode::IndirectX => write!(f, "IndirectX"),
            AddressingMode::IndirectY => write!(f, "IndirectY"),
            AddressingMode::ZeroPage => write!(f, "ZeroPage"),
            AddressingMode::ZeroPageX => write!(f, "ZeroPageX"),
            AddressingMode::ZeroPageY => write!(f, "ZeroPageY"),
            AddressingMode::Absolute => write!(f, "Absolute"),
            AddressingMode::AbsoluteX => write!(f, "AbsoluteX"),
            AddressingMode::AbsoluteY => write!(f, "AbsoluteY"),
        }
    }
}