use crate::cpu::memory::AddressingMode;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Instruction {
    // fixed set of strings for the instructions that are stored in the binary
    pub mnemonic: &'static str,
    pub opcode: u8,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
    pub bytes: u8,
}

impl Instruction {
    pub fn new(
        mnemonic: &'static str,
        opcode: u8,
        addressing_mode: AddressingMode,
        cycles: u8,
        bytes: u8,
    ) -> Self {
        Instruction {
            mnemonic,
            opcode,
            addressing_mode,
            cycles,
            bytes,
        }
    }
}

lazy_static! {
    pub static ref INSTRUCTIONS: HashMap<u8, Instruction> = {
        let mut map = HashMap::new();

        // transfer
        map.insert(0xA9, Instruction::new("LDA", 0xA9, AddressingMode::Immediate, 2, 2));
        map.insert(0xA5, Instruction::new("LDA", 0xA5, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xB5, Instruction::new("LDA", 0xB5, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0xAD, Instruction::new("LDA", 0xAD, AddressingMode::Absolute, 4, 3));
        map.insert(0xBD, Instruction::new("LDA", 0xBD, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0xB9, Instruction::new("LDA", 0xB9, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0xA1, Instruction::new("LDA", 0xA1, AddressingMode::IndirectX, 6, 2));
        map.insert(0xB1, Instruction::new("LDA", 0xB1, AddressingMode::IndirectY, 5, 2));
        map.insert(0xA2, Instruction::new("LDX", 0xA2, AddressingMode::Immediate, 2, 2));
        map.insert(0xA6, Instruction::new("LDX", 0xA6, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xB6, Instruction::new("LDX", 0xB6, AddressingMode::ZeroPageY, 4, 2));
        map.insert(0xAE, Instruction::new("LDX", 0xAE, AddressingMode::Absolute, 4, 3));
        map.insert(0xBE, Instruction::new("LDX", 0xBE, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0xA0, Instruction::new("LDY", 0xA0, AddressingMode::Immediate, 2, 2));
        map.insert(0xA4, Instruction::new("LDY", 0xA4, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xB4, Instruction::new("LDY", 0xB4, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0xAC, Instruction::new("LDY", 0xAC, AddressingMode::Absolute, 4, 3));
        map.insert(0xBC, Instruction::new("LDY", 0xBC, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0x85, Instruction::new("STA", 0x85, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x8D, Instruction::new("STA", 0x8D, AddressingMode::Absolute, 4, 3));
        map.insert(0x9D, Instruction::new("STA", 0x9D, AddressingMode::AbsoluteX, 5, 3));
        map.insert(0x99, Instruction::new("STA", 0x99, AddressingMode::AbsoluteY, 5, 3));
        map.insert(0x81, Instruction::new("STA", 0x81, AddressingMode::IndirectX, 6, 2));
        map.insert(0x91, Instruction::new("STA", 0x91, AddressingMode::IndirectY, 6, 2));
        map.insert(0x95, Instruction::new("STA", 0x95, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0x86, Instruction::new("STX", 0x86, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x96, Instruction::new("STX", 0x96, AddressingMode::ZeroPageY, 4, 2));
        map.insert(0x8E, Instruction::new("STX", 0x8E, AddressingMode::Absolute, 4, 3));
        map.insert(0x84, Instruction::new("STY", 0x84, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x94, Instruction::new("STY", 0x94, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0x8C, Instruction::new("STY", 0x8C, AddressingMode::Absolute, 4, 3));
        map.insert(0xAA, Instruction::new("TAX", 0xAA, AddressingMode::Implied, 2, 1));
        map.insert(0xA8, Instruction::new("TAY", 0xA8, AddressingMode::Implied, 2, 1));
        map.insert(0xBA, Instruction::new("TSX", 0xBA, AddressingMode::Implied, 2, 1));
        map.insert(0x8A, Instruction::new("TXA", 0x8A, AddressingMode::Implied, 2, 1));
        map.insert(0x98, Instruction::new("TYA", 0x98, AddressingMode::Implied, 2, 1));
        map.insert(0x9A, Instruction::new("TXS", 0x9A, AddressingMode::Implied, 2, 1));
        
        // stack
        map.insert(0x48, Instruction::new("PHA", 0x48, AddressingMode::Implied, 1, 3));
        map.insert(0x08, Instruction::new("PHP", 0x08, AddressingMode::Implied, 1, 3));
        map.insert(0x68, Instruction::new("PLA", 0x68, AddressingMode::Implied, 1, 4));

        // inc/dec
        map.insert(0xC6, Instruction::new("DEC", 0xC6, AddressingMode::ZeroPage, 5, 2));
        map.insert(0xD6, Instruction::new("DEC", 0xD6, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0xCE, Instruction::new("DEC", 0xCE, AddressingMode::Absolute, 6, 3));
        map.insert(0xDE, Instruction::new("DEC", 0xDE, AddressingMode::AbsoluteX, 7, 3));
        map.insert(0xCA, Instruction::new("DEX", 0xCA, AddressingMode::Implied, 2, 1));
        map.insert(0x88, Instruction::new("DEY", 0x88, AddressingMode::Implied, 2, 1));
        map.insert(0xE6, Instruction::new("INC", 0xE6, AddressingMode::ZeroPage, 5, 2));
        map.insert(0xF6, Instruction::new("INC", 0xF6, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0xEE, Instruction::new("INC", 0xEE, AddressingMode::Absolute, 6, 3));
        map.insert(0xFE, Instruction::new("INC", 0xFE, AddressingMode::AbsoluteX, 7, 3));
        map.insert(0xE8, Instruction::new("INX", 0xE8, AddressingMode::Implied, 2, 1));
        map.insert(0xC8, Instruction::new("INY", 0xC8, AddressingMode::Implied, 2, 1));

        // arithmetic
        map.insert(0x69, Instruction::new("ADC", 0x69, AddressingMode::Immediate, 2, 2));
        map.insert(0x65, Instruction::new("ADC", 0x65, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x75, Instruction::new("ADC", 0x75, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0x6D, Instruction::new("ADC", 0x6D, AddressingMode::Absolute, 4, 3));
        map.insert(0x7D, Instruction::new("ADC", 0x7D, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0x79, Instruction::new("ADC", 0x79, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0x61, Instruction::new("ADC", 0x61, AddressingMode::IndirectX, 6, 2));
        map.insert(0x71, Instruction::new("ADC", 0x71, AddressingMode::IndirectY, 5, 2));
        map.insert(0xE9, Instruction::new("SBC", 0xE9, AddressingMode::Immediate, 2, 2));
        map.insert(0xE5, Instruction::new("SBC", 0xE5, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xF5, Instruction::new("SBC", 0xF5, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0xED, Instruction::new("SBC", 0xED, AddressingMode::Absolute, 4, 3));
        map.insert(0xFD, Instruction::new("SBC", 0xFD, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0xF9, Instruction::new("SBC", 0xF9, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0xE1, Instruction::new("SBC", 0xE1, AddressingMode::IndirectX, 6, 2));
        map.insert(0xF1, Instruction::new("SBC", 0xF1, AddressingMode::IndirectY, 5, 2));

        // logical
        map.insert(0x29, Instruction::new("AND", 0x29, AddressingMode::Immediate, 2, 2));
        map.insert(0x49, Instruction::new("EOR", 0x49, AddressingMode::Immediate, 2, 2));
        map.insert(0x45, Instruction::new("EOR", 0x45, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x55, Instruction::new("EOR", 0x55, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0x4D, Instruction::new("EOR", 0x4D, AddressingMode::Absolute, 4, 3));
        map.insert(0x5D, Instruction::new("EOR", 0x5D, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0x59, Instruction::new("EOR", 0x59, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0x41, Instruction::new("EOR", 0x41, AddressingMode::IndirectX, 6, 2));
        map.insert(0x51, Instruction::new("EOR", 0x51, AddressingMode::IndirectY, 5, 2));
        map.insert(0x09, Instruction::new("ORA", 0x09, AddressingMode::Immediate, 2, 2));
        map.insert(0x05, Instruction::new("ORA", 0x05, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x15, Instruction::new("ORA", 0x15, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0x0D, Instruction::new("ORA", 0x0D, AddressingMode::Absolute, 4, 3));
        map.insert(0x1D, Instruction::new("ORA", 0x1D, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0x19, Instruction::new("ORA", 0x19, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0x01, Instruction::new("ORA", 0x01, AddressingMode::IndirectX, 6, 2));
        map.insert(0x11, Instruction::new("ORA", 0x11, AddressingMode::IndirectY, 5, 2));

        // shift
        map.insert(0x0A, Instruction::new("ASL", 0x0A, AddressingMode::Implied, 2, 1));
        map.insert(0x06, Instruction::new("ASL", 0x06, AddressingMode::ZeroPage, 5, 2));
        map.insert(0x16, Instruction::new("ASL", 0x16, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0x0E, Instruction::new("ASL", 0x0E, AddressingMode::Absolute, 6, 3));
        map.insert(0x1E, Instruction::new("ASL", 0x1E, AddressingMode::AbsoluteX, 7, 3));
        map.insert(0x4A, Instruction::new("LSR", 0x4A, AddressingMode::Implied, 2, 1));
        map.insert(0x46, Instruction::new("LSR", 0x46, AddressingMode::ZeroPage, 5, 2));
        map.insert(0x56, Instruction::new("LSR", 0x56, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0x4E, Instruction::new("LSR", 0x4E, AddressingMode::Absolute, 6, 3));
        map.insert(0x5E, Instruction::new("LSR", 0x5E, AddressingMode::AbsoluteX, 7, 3));
        map.insert(0x2A, Instruction::new("ROL", 0x2A, AddressingMode::Implied, 2, 1));
        map.insert(0x6A, Instruction::new("ROR", 0x6A, AddressingMode::Implied, 2, 1));

        // flags
        map.insert(0x18, Instruction::new("CLC", 0x18, AddressingMode::Implied, 2, 1));
        map.insert(0xD8, Instruction::new("CLD", 0xD8, AddressingMode::Implied, 2, 1));
        map.insert(0x58, Instruction::new("CLI", 0x58, AddressingMode::Implied, 2, 1));
        map.insert(0xB8, Instruction::new("CLV", 0xB8, AddressingMode::Implied, 2, 1));
        map.insert(0x38, Instruction::new("SEC", 0x38, AddressingMode::Implied, 2, 1));
        map.insert(0xF8, Instruction::new("SED", 0xF8, AddressingMode::Implied, 2, 1));
        map.insert(0x78, Instruction::new("SEI", 0x78, AddressingMode::Implied, 2, 1));

        // compare
        map.insert(0xC9, Instruction::new("CMP", 0xC9, AddressingMode::Immediate, 2, 2));
        map.insert(0xC5, Instruction::new("CMP", 0xC5, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xD5, Instruction::new("CMP", 0xD5, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0xCD, Instruction::new("CMP", 0xCD, AddressingMode::Absolute, 4, 3));
        map.insert(0xDD, Instruction::new("CMP", 0xDD, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0xD9, Instruction::new("CMP", 0xD9, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0xC1, Instruction::new("CMP", 0xC1, AddressingMode::IndirectX, 6, 2));
        map.insert(0xD1, Instruction::new("CMP", 0xD1, AddressingMode::IndirectY, 5, 2));
        map.insert(0xE0, Instruction::new("CPX", 0xE0, AddressingMode::Immediate, 2, 2));
        map.insert(0xE4, Instruction::new("CPX", 0xE4, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xEC, Instruction::new("CPX", 0xEC, AddressingMode::Absolute, 4, 3));
        map.insert(0xC0, Instruction::new("CPY", 0xC0, AddressingMode::Immediate, 2, 2));
        map.insert(0xC4, Instruction::new("CPY", 0xC4, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xCC, Instruction::new("CPY", 0xCC, AddressingMode::Absolute, 4, 3));

        // branch
        map.insert(0x90, Instruction::new("BCC", 0x90, AddressingMode::Implied, 2, 2));
        map.insert(0xB0, Instruction::new("BCS", 0xB0, AddressingMode::Implied, 2, 2));
        map.insert(0xF0, Instruction::new("BEQ", 0xF0, AddressingMode::Implied, 2, 2));
        map.insert(0x30, Instruction::new("BMI", 0x30, AddressingMode::Implied, 2, 2));
        map.insert(0xD0, Instruction::new("BNE", 0xD0, AddressingMode::Implied, 2, 2));
        map.insert(0x10, Instruction::new("BPL", 0x10, AddressingMode::Implied, 2, 2));
        map.insert(0x50, Instruction::new("BVC", 0x50, AddressingMode::Implied, 2, 2));
        map.insert(0x70, Instruction::new("BVS", 0x70, AddressingMode::Implied, 2, 2));

        // jump
        map.insert(0x4C, Instruction::new("JMP", 0x4C, AddressingMode::Absolute, 3, 3));
        map.insert(0x6C, Instruction::new("JMP", 0x6C, AddressingMode::IndirectX, 5, 3));
        map.insert(0x20, Instruction::new("JSR", 0x20, AddressingMode::Absolute, 6, 3));
        map.insert(0x60, Instruction::new("RTS", 0x60, AddressingMode::Implied, 6, 1));
        
        // interrupts
        map.insert(0x00, Instruction::new("BRK", 0x00, AddressingMode::Implied, 7, 1));
        map.insert(0x40, Instruction::new("RTI", 0x40, AddressingMode::Implied, 6, 1));

        // other
        map.insert(0x2C, Instruction::new("BIT", 0x2C, AddressingMode::Absolute, 4, 3));
        map.insert(0x24, Instruction::new("BIT", 0x24, AddressingMode::ZeroPage, 3, 2));
        map.insert(0xEA, Instruction::new("NOP", 0xEA, AddressingMode::Implied, 2, 1));

        map
    };
}
