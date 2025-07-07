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

#[derive(Debug)]
pub enum AddressingMode {
    None,
    Immediate,
    Implied,
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
            //AddressingMode::Indirect => write!(f, "Indirect"),
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