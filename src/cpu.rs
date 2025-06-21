use lazy_static::lazy_static;
use std::collections::HashMap;
use bitflags::bitflags;
use crate::instruction::Instruction;
use crate::memory::Memory;

lazy_static! {
    static ref INSTRUCTIONS: HashMap<u8, Instruction> = {
        let mut map = HashMap::new();
        // interrupts
        map.insert(0x00, Instruction::new("BRK", 0x00, AddressingMode::Implied, 7, 1));
        map.insert(0x40, Instruction::new("RTI", 0x40, AddressingMode::Implied, 6, 1));
        // compare
        map.insert(0x09, Instruction::new("ORA", 0x09, AddressingMode::Immediate, 2, 2));
        map.insert(0x05, Instruction::new("ORA", 0x05, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x15, Instruction::new("ORA", 0x15, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0x0D, Instruction::new("ORA", 0x0D, AddressingMode::Absolute, 4, 3));        
        map.insert(0x1D, Instruction::new("ORA", 0x1D, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0x19, Instruction::new("ORA", 0x19, AddressingMode::AbsoluteY, 4, 3));
        map.insert(0x01, Instruction::new("ORA", 0x01, AddressingMode::IndirectX, 6, 2));
        map.insert(0x11, Instruction::new("ORA", 0x11, AddressingMode::IndirectY, 5, 2));
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
        map.insert(0xAA, Instruction::new("TAX", 0xAA, AddressingMode::Implied, 2, 1));
        map.insert(0xE8, Instruction::new("INX", 0xE8, AddressingMode::Implied, 2, 1));

        // arithmetic
        map.insert(0x69, Instruction::new("ADC", 0x69, AddressingMode::Immediate, 2, 2));
        map.insert(0xE9, Instruction::new("SBC", 0xE9, AddressingMode::Immediate, 2, 2));
        // jump
        map.insert(0xB0, Instruction::new("BCS", 0xB0, AddressingMode::Implied, 2, 2));
        map.insert(0x90, Instruction::new("BCC", 0x90, AddressingMode::Implied, 2, 2));
        map.insert(0xF0, Instruction::new("BEQ", 0xF0, AddressingMode::Implied, 2, 2));
        map.insert(0xD0, Instruction::new("BNE", 0xD0, AddressingMode::Implied, 2, 2));
        map.insert(0x20, Instruction::new("JSR", 0x20, AddressingMode::Absolute, 6, 3));
        map.insert(0x30, Instruction::new("BIM", 0x30, AddressingMode::Implied, 2, 2));
        // stack
        map.insert(0x48, Instruction::new("PHA", 0x48, AddressingMode::Implied, 1, 3));
        map.insert(0x08, Instruction::new("PHP", 0x08, AddressingMode::Implied, 1, 3));
        map.insert(0x68, Instruction::new("PLA", 0x68, AddressingMode::Implied, 1, 4));
        // --- ADDED INSTRUCTIONS ---
        // NOP
        map.insert(0xEA, Instruction::new("NOP", 0xEA, AddressingMode::Implied, 2, 1));
        // JMP
        map.insert(0x4C, Instruction::new("JMP", 0x4C, AddressingMode::Absolute, 3, 3));
        map.insert(0x6C, Instruction::new("JMP", 0x6C, AddressingMode::IndirectX, 5, 3));
        // STA (Absolute, AbsoluteX, AbsoluteY, ZeroPageX, ZeroPageY, IndirectX, IndirectY)
        map.insert(0x8D, Instruction::new("STA", 0x8D, AddressingMode::Absolute, 4, 3));
        map.insert(0x9D, Instruction::new("STA", 0x9D, AddressingMode::AbsoluteX, 5, 3));
        map.insert(0x99, Instruction::new("STA", 0x99, AddressingMode::AbsoluteY, 5, 3));
        map.insert(0x81, Instruction::new("STA", 0x81, AddressingMode::IndirectX, 6, 2));
        map.insert(0x91, Instruction::new("STA", 0x91, AddressingMode::IndirectY, 6, 2));
        map.insert(0x95, Instruction::new("STA", 0x95, AddressingMode::ZeroPageX, 4, 2));
        // STX, STY
        map.insert(0x86, Instruction::new("STX", 0x86, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x96, Instruction::new("STX", 0x96, AddressingMode::ZeroPageY, 4, 2));
        map.insert(0x8E, Instruction::new("STX", 0x8E, AddressingMode::Absolute, 4, 3));
        map.insert(0x84, Instruction::new("STY", 0x84, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x94, Instruction::new("STY", 0x94, AddressingMode::ZeroPageX, 4, 2));
        map.insert(0x8C, Instruction::new("STY", 0x8C, AddressingMode::Absolute, 4, 3));
        // Register transfer
        map.insert(0x8A, Instruction::new("TXA", 0x8A, AddressingMode::Implied, 2, 1));
        map.insert(0x98, Instruction::new("TYA", 0x98, AddressingMode::Implied, 2, 1));
        map.insert(0x9A, Instruction::new("TXS", 0x9A, AddressingMode::Implied, 2, 1));
        map.insert(0xBA, Instruction::new("TSX", 0xBA, AddressingMode::Implied, 2, 1));
        map.insert(0xA8, Instruction::new("TAY", 0xA8, AddressingMode::Implied, 2, 1));
        // INC, INY, DEC, DEX, DEY
        map.insert(0xE6, Instruction::new("INC", 0xE6, AddressingMode::ZeroPage, 5, 2));
        map.insert(0xF6, Instruction::new("INC", 0xF6, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0xEE, Instruction::new("INC", 0xEE, AddressingMode::Absolute, 6, 3));
        map.insert(0xFE, Instruction::new("INC", 0xFE, AddressingMode::AbsoluteX, 7, 3));
        map.insert(0xC8, Instruction::new("INY", 0xC8, AddressingMode::Implied, 2, 1));
        map.insert(0xCA, Instruction::new("DEX", 0xCA, AddressingMode::Implied, 2, 1));
        map.insert(0x88, Instruction::new("DEY", 0x88, AddressingMode::Implied, 2, 1));
        map.insert(0x29, Instruction::new("AND", 0x29, AddressingMode::Immediate, 2, 2));
        map.insert(0x49, Instruction::new("EOR", 0x49, AddressingMode::Immediate, 2, 2));
        map.insert(0xC9, Instruction::new("CMP", 0xC9, AddressingMode::Immediate, 2, 2));
        map.insert(0xE0, Instruction::new("CPX", 0xE0, AddressingMode::Immediate, 2, 2));
        map.insert(0xC0, Instruction::new("CPY", 0xC0, AddressingMode::Immediate, 2, 2));
        map.insert(0x0A, Instruction::new("ASL", 0x0A, AddressingMode::Implied, 2, 1));
        map.insert(0x4A, Instruction::new("LSR", 0x4A, AddressingMode::Implied, 2, 1));
        map.insert(0x2A, Instruction::new("ROL", 0x2A, AddressingMode::Implied, 2, 1));
        map.insert(0x6A, Instruction::new("ROR", 0x6A, AddressingMode::Implied, 2, 1));
        map.insert(0x18, Instruction::new("CLC", 0x18, AddressingMode::Implied, 2, 1));
        map.insert(0x38, Instruction::new("SEC", 0x38, AddressingMode::Implied, 2, 1));
        map.insert(0x58, Instruction::new("CLI", 0x58, AddressingMode::Implied, 2, 1));
        map.insert(0x78, Instruction::new("SEI", 0x78, AddressingMode::Implied, 2, 1));
        map.insert(0xB8, Instruction::new("CLV", 0xB8, AddressingMode::Implied, 2, 1));
        map.insert(0xD8, Instruction::new("CLD", 0xD8, AddressingMode::Implied, 2, 1));
        map.insert(0xF8, Instruction::new("SED", 0xF8, AddressingMode::Implied, 2, 1));
        map
    };
}

#[allow(dead_code)]
const PROGRAM_ADDRESS: u16 = 0x8000;

/// Represents the 6502 CPU, including registers, program counter, 
/// stack pointer, status flags, memory, and cycle count.
/// Provides methods for executing instructions, managing memory, 
/// and simulating CPU behavior.
pub struct Cpu {
    // Accumulator
    pub a: u8,

    // X and Y registers
    pub x: u8,
    pub y: u8,

    // Program counter
    // Stores the address of the next byte for the CPU to read.
    // Increases with each clock or can be directly set in a branch to jump to
    // different parts of the program, like an if-statement.
    pub pc: u16,

    // Stack pointer
    // Points to an address somewhere in the memory (bus)
    // Incremented/decremented as we pull things from the stack
    pub sp: u8,
    
    // Status register
    pub p: StatusFlag,
    pub memory: [u8; 0xFFFF],
    pub cycles: u8,
}

impl Memory for Cpu {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFD,
            p: StatusFlag::empty(),
            memory: [0; 0xFFFF],
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;

        // set PC to the address stored at 0xFFFC
        self.pc = self.read_word(0xFFFC);
        self.sp = 0xFD;
        self.p = StatusFlag::InterruptDisable | StatusFlag::Unused;
        self.cycles = 0;
    }

    pub fn set_flag(&mut self, flag: StatusFlag, value: bool) {
        if value {
            self.p |= flag;
        } else {
            self.p &= !flag;
        }
    }

    pub fn get_flag(&self, flag: StatusFlag) -> bool {
        self.p & flag != StatusFlag::empty()
    }

    // Loads the given program to PRG ROM memory range (0x8000-0xFFFF)
    pub fn load_program(&mut self, program: Vec<u8>, address: u16) {
        let start = address as usize;
        let end = start + program.len();
        if end > self.memory.len() {
            panic!("Program does not fit in memory: end address {:#X} exceeds memory size {:#X}", end, self.memory.len());
        }
        self.memory[start..end].copy_from_slice(&program);
        self.write_word(0xFFFC, address);
    }

    pub fn run(&mut self) {
        loop {
            // get opcode at program counter
            let opcode = self.read_byte(self.pc);
            self.pc += 1;

            // get instruction metadata for opcode
            let instruction = INSTRUCTIONS
                .get(&opcode)
                .unwrap();
            
            // get operand address for instruction
            let operand_address = self.get_operand_address(instruction);
            
            match instruction.opcode {
                0x00 => { self.brk(operand_address, &instruction.addressing_mode); break; },
                0xA9 => { self.lda(operand_address, &instruction.addressing_mode); },
                0xAA => { self.tax(); },
                0xE8 => { self.inx(); },
                0xEA => { self.nop(); },
                0x4C => { self.jmp(operand_address); },
                0x8D => { self.sta(operand_address); },
                0x8A => { self.txa(); },
                0x98 => { self.tya(); },
                0xA8 => { self.tay(); },
                0xC8 => { self.iny(); },
                0xCA => { self.dex(); },
                0x88 => { self.dey(); },
                // ...add more as you implement...
                _ => panic!("Instruction {} not implemented!", instruction.mnemonic),
            };
        }
    }

    fn get_operand_address(&self, instruction: &Instruction) -> u16 {
        match instruction.addressing_mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.read_byte(self.pc) as u16,
            AddressingMode::ZeroPageX => {
                let addr = self.read_byte(self.pc);
                addr.wrapping_add(self.x) as u16
            }
            AddressingMode::ZeroPageY => {
                let addr = self.read_byte(self.pc);
                addr.wrapping_add(self.y) as u16
            }
            AddressingMode::Absolute => self.read_word(self.pc),
            AddressingMode::AbsoluteX => self.read_word(self.pc).wrapping_add(self.x as u16),
            AddressingMode::AbsoluteY => self.read_word(self.pc).wrapping_add(self.y as u16),
            AddressingMode::IndirectX => {
                let base = self.read_byte(self.pc);
                let ptr = base.wrapping_add(self.x);
                let lo = self.read_byte(ptr as u16) as u16;
                let hi = self.read_byte(ptr.wrapping_add(1) as u16) as u16;
                (hi << 8) | lo
            }
            AddressingMode::IndirectY => {
                let base = self.read_byte(self.pc);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                ((hi << 8) | lo).wrapping_add(self.y as u16)
            }
            AddressingMode::Implied => 0, // Implied has no operand address
            AddressingMode::None => panic!("Addressing mode {} not supported!", instruction.addressing_mode),
        }
    }

    fn brk(&mut self, _address: u16, _addressing_mode: &AddressingMode) -> u8 {
        0
    }

    fn tax(&mut self) {
        self.x = self.a;
        self.set_flag(StatusFlag::Zero, self.x == 0);
        self.set_flag(StatusFlag::Negative, self.x & 0x80 != 0);
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.set_flag(StatusFlag::Zero, self.x == 0);
        self.set_flag(StatusFlag::Negative, self.x & 0x80 != 0);
    }

    fn nop(&mut self) {
        // No operation
    }

    fn jmp(&mut self, address: u16) {
        self.pc = address;
    }

    fn sta(&mut self, address: u16) {
        self.write_byte(address, self.a);
    }

    fn txa(&mut self) {
        self.a = self.x;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    fn tya(&mut self) {
        self.a = self.y;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    fn tay(&mut self) {
        self.y = self.a;
        self.set_flag(StatusFlag::Zero, self.y == 0);
        self.set_flag(StatusFlag::Negative, self.y & 0x80 != 0);
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.set_flag(StatusFlag::Zero, self.y == 0);
        self.set_flag(StatusFlag::Negative, self.y & 0x80 != 0);
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.set_flag(StatusFlag::Zero, self.x == 0);
        self.set_flag(StatusFlag::Negative, self.x & 0x80 != 0);
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.set_flag(StatusFlag::Zero, self.y == 0);
        self.set_flag(StatusFlag::Negative, self.y & 0x80 != 0);
    }

    fn lda(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };
        self.a = value;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }
}

use std::fmt;

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

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StatusFlag: u8 {
        const Carry = 1 << 0;
        const Zero = 1 << 1;
        const InterruptDisable = 1 << 2;
        const Decimal = 1 << 3;
        const Break = 1 << 4;
        const Unused = 1 << 5;
        const Overflow = 1 << 6;
        const Negative = 1 << 7;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write_byte() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x0000, 0x42);
        assert_eq!(cpu.read_byte(0x0000), 0x42);
    }

    #[test]
    fn test_read_write_word() {
        let mut cpu = Cpu::new();
        cpu.write_word(0x0000, 0x4242);
        assert_eq!(cpu.read_word(0x0000), 0x4242);
    }

    #[test]
    fn test_reset() {
        let mut cpu = Cpu::new();
        cpu.write_word(0xFFFC, 0x4242);
        cpu.reset();
        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.pc, 0x4242);
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.p, StatusFlag::InterruptDisable | StatusFlag::Unused);
        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn test_get_set_flag() {
        let mut cpu = Cpu::new();
        let flag = StatusFlag::Carry;
        cpu.set_flag(flag, true);
        assert_eq!(cpu.get_flag(flag), true);
        cpu.set_flag(flag, false);
        assert_eq!(cpu.get_flag(flag), false);
    }

    #[test]
    fn test_load_program() {
        let mut cpu = Cpu::new();
        let program = vec![0x42, 0x42];
        cpu.load_program(program, PROGRAM_ADDRESS);
        assert_eq!(cpu.read_byte(PROGRAM_ADDRESS+0), 0x42);
        assert_eq!(cpu.read_byte(PROGRAM_ADDRESS+1), 0x42);
    }

    //#[test]
    fn test_run_program_with_5_instructions() {
        // assembly:
        // LDA #$C0     /* load A with 0xC0 */
        // TAX          /* copy A to X */
        // INX          /* increment X */
        // BRK          /* break */

        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00], PROGRAM_ADDRESS);
        cpu.reset();
        cpu.run();
        assert_eq!(cpu.x, 0xC1);
    }    
}