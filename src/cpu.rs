use lazy_static::lazy_static;
use std::collections::HashMap;
use bitflags::bitflags;
use crate::instruction::Instruction;

trait Memory {
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
        map
    };
}

#[allow(dead_code)]
const PROGRAM_ADDRESS: u16 = 0x8000;

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
            sp: 0,
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
        self.p = StatusFlag::empty();
        self.cycles = 0;

        self.set_flag(StatusFlag::InterruptDisable, true);
    }

    fn set_flag(&mut self, flag: StatusFlag, value: bool) {
        if value {
            self.p |= flag;
        } else {
            self.p &= !flag;
        }
    }

    fn get_flag(&self, flag: StatusFlag) -> bool {
        self.p & flag != StatusFlag::empty()
    }

    /// Loads the given program to PRG ROM memory range (0x8000-0xFFFF)
    pub fn load_program(&mut self, program: Vec<u8>, address: u16) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[i] = *byte;
        }
        self.write_word(0xFFFC, address);
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_byte(self.pc);
            let instruction = INSTRUCTIONS
                .get(&opcode)
                .unwrap();

            let mut operand_address: u16 = 0;
            match instruction.addressing_mode {
                AddressingMode::Immediate => {
                    operand_address = self.pc;
                },
                _ => (),
            }

            self.pc += 1;

            match opcode {
                0x00 => self.brk(operand_address),
                0xA9 => self.lda(operand_address),
                _ => panic!("Opcode: {:#04X} not implemented!", opcode),
            }
        }
    }

    fn lda(&mut self, address: u16) {
        self.a = self.read_byte(address);
        self.pc += 1;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    fn brk(&mut self, _address: u16) {

    }
}

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
    //use bitflags::Flags;

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
        assert_eq!(cpu.p, StatusFlag::InterruptDisable);
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
        assert_eq!(cpu.memory[0], 0x42);
        assert_eq!(cpu.memory[1], 0x42);
    }

    //#[test]
    fn test_run_program_with_5_instructions() {

        // assambly:
        // LDA #$C0     /* load A with 0xC0 */
        // TAX          /* copy A to X */
        // INX          /* increment X */
        // BRK          */ break */

        // arrange
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00], PROGRAM_ADDRESS);
        cpu.write_byte(0xC0, 42);
        cpu.reset();

        // act
        cpu.run();

        // assert
        assert_eq!(cpu.x, 0xC1);
    }    
}