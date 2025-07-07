use crate::cpu::memory::AddressingMode;
use crate::cpu::memory::Memory;

use bitflags::bitflags;
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
        map.insert(0x89, Instruction::new("BIT", 0x89, AddressingMode::Immediate, 2, 3));
        map.insert(0x3C, Instruction::new("BIT", 0x3C, AddressingMode::AbsoluteX, 4, 3));
        map.insert(0x24, Instruction::new("BIT", 0x24, AddressingMode::ZeroPage, 3, 2));
        map.insert(0x34, Instruction::new("BIT", 0x34, AddressingMode::ZeroPageX, 4, 2));
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
        map.insert(0xC6, Instruction::new("DEC", 0xC6, AddressingMode::ZeroPage, 5, 2));
        map.insert(0xD6, Instruction::new("DEC", 0xD6, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0xCE, Instruction::new("DEC", 0xCE, AddressingMode::Absolute, 6, 3));
        map.insert(0xDE, Instruction::new("DEC", 0xDE, AddressingMode::AbsoluteX, 7, 3));
        // jump
        map.insert(0xB0, Instruction::new("BCS", 0xB0, AddressingMode::Implied, 2, 2));
        map.insert(0x90, Instruction::new("BCC", 0x90, AddressingMode::Implied, 2, 2));
        map.insert(0xF0, Instruction::new("BEQ", 0xF0, AddressingMode::Implied, 2, 2));
        map.insert(0xD0, Instruction::new("BNE", 0xD0, AddressingMode::Implied, 2, 2));
        map.insert(0x10, Instruction::new("BPL", 0x10, AddressingMode::Implied, 2, 2));
        map.insert(0x20, Instruction::new("JSR", 0x20, AddressingMode::Absolute, 6, 3));
        map.insert(0x30, Instruction::new("BIM", 0x30, AddressingMode::Implied, 2, 2));
        map.insert(0x60, Instruction::new("RTS", 0x60, AddressingMode::Implied, 6, 1));
        // stack
        map.insert(0x48, Instruction::new("PHA", 0x48, AddressingMode::Implied, 1, 3));
        map.insert(0x08, Instruction::new("PHP", 0x08, AddressingMode::Implied, 1, 3));
        map.insert(0x68, Instruction::new("PLA", 0x68, AddressingMode::Implied, 1, 4));
        map.insert(0xEA, Instruction::new("NOP", 0xEA, AddressingMode::Implied, 2, 1));
        map.insert(0x4C, Instruction::new("JMP", 0x4C, AddressingMode::Absolute, 3, 3));
        map.insert(0x6C, Instruction::new("JMP", 0x6C, AddressingMode::IndirectX, 5, 3));
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
        // Register transfer
        map.insert(0x8A, Instruction::new("TXA", 0x8A, AddressingMode::Implied, 2, 1));
        map.insert(0x98, Instruction::new("TYA", 0x98, AddressingMode::Implied, 2, 1));
        map.insert(0x9A, Instruction::new("TXS", 0x9A, AddressingMode::Implied, 2, 1));
        map.insert(0xBA, Instruction::new("TSX", 0xBA, AddressingMode::Implied, 2, 1));
        map.insert(0xA8, Instruction::new("TAY", 0xA8, AddressingMode::Implied, 2, 1));
        map.insert(0xE6, Instruction::new("INC", 0xE6, AddressingMode::ZeroPage, 5, 2));
        map.insert(0xF6, Instruction::new("INC", 0xF6, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0xEE, Instruction::new("INC", 0xEE, AddressingMode::Absolute, 6, 3));
        map.insert(0xFE, Instruction::new("INC", 0xFE, AddressingMode::AbsoluteX, 7, 3));
        map.insert(0xC8, Instruction::new("INY", 0xC8, AddressingMode::Implied, 2, 1));
        map.insert(0xCA, Instruction::new("DEX", 0xCA, AddressingMode::Implied, 2, 1));
        map.insert(0x88, Instruction::new("DEY", 0x88, AddressingMode::Implied, 2, 1));
        map.insert(0x29, Instruction::new("AND", 0x29, AddressingMode::Immediate, 2, 2));
        map.insert(0x49, Instruction::new("EOR", 0x49, AddressingMode::Immediate, 2, 2));
        map.insert(0x0A, Instruction::new("ASL", 0x0A, AddressingMode::Implied, 2, 1));
        // shift
        map.insert(0x4A, Instruction::new("LSR", 0x4A, AddressingMode::Implied, 2, 1));
        map.insert(0x46, Instruction::new("LSR", 0x46, AddressingMode::ZeroPage, 5, 2));
        map.insert(0x56, Instruction::new("LSR", 0x56, AddressingMode::ZeroPageX, 6, 2));
        map.insert(0x4E, Instruction::new("LSR", 0x4E, AddressingMode::Absolute, 6, 3));
        map.insert(0x5E, Instruction::new("LSR", 0x5E, AddressingMode::AbsoluteX, 7, 3));

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
    pub instruction_count: u64,
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
            instruction_count: 0,
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
        self.instruction_count = 0;
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

    // Getter methods for CPU registers
    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn get_sp(&self) -> u8 {
        self.sp
    }

    pub fn get_status(&self) -> u8 {
        self.p.bits()
    }

    pub fn get_cycles(&self) -> u8 {
        self.cycles
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_instruction_count(&self) -> u64 {
        self.instruction_count
    }

    // Loads the given program to PRG ROM memory range (0x8000-0xFFFF)
    pub fn load_program(&mut self, program: Vec<u8>, address: u16) {
        let start = address as usize;
        let end = start + program.len();
        if end > self.memory.len() {
            panic!(
                "Program does not fit in memory: end address {:#X} exceeds memory size {:#X}",
                end,
                self.memory.len()
            );
        }
        self.memory[start..end].copy_from_slice(&program);
        self.write_word(0xFFFC, address);
        self.reset();
        self.disassemble(start as u16, end as u16);
    }

    pub fn run(&mut self, show_disassembly: bool) {
        loop {
            if show_disassembly {
                println!("PC: {:#X}", self.pc);
                let upper_address = self.pc + 4;
                self.disassemble(self.pc, upper_address);
            }

            if !self.step() {
                break;
            }
        }
    }

    pub fn step(&mut self) -> bool {
        // Execute one instruction and return true if should continue
        let opcode = self.read_byte(self.pc);

        // get instruction metadata for opcode
        if INSTRUCTIONS.get(&opcode).is_none() {
            panic!("Unknown opcode: {:#X} at PC: {:#X}", opcode, self.pc);
        }

        let instruction = INSTRUCTIONS.get(&opcode).unwrap();

        self.pc += 1;

        // get operand address for instruction
        let operand_address = self.get_operand_address(instruction);

        match instruction.opcode {
            0x00 => {
                self.brk(operand_address, &instruction.addressing_mode);
                return false; // Stop execution on BRK
            }
            // LDA instructions
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                self.lda(operand_address, &instruction.addressing_mode);
            }

            // ORA instructions
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                self.ora(operand_address, &instruction.addressing_mode);
            }

            // CMP instructions
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                self.cmp(operand_address, &instruction.addressing_mode);
            }

            // CPX instructions
            0xE0 | 0xE4 | 0xEC => {
                self.cpx(operand_address, &instruction.addressing_mode);
            }

            // CPY instructions
            0xC0 | 0xC4 | 0xCC => {
                self.cpy(operand_address, &instruction.addressing_mode);
            }

            // Transfer instructions
            0xAA => {
                self.tax();
            }
            0xA8 => {
                self.tay();
            }
            0x8A => {
                self.txa();
            }
            0x98 => {
                self.tya();
            }
            0xE8 => {
                self.inx();
            }
            0xC8 => {
                self.iny();
            }
            0xCA => {
                self.dex();
            }
            0x88 => {
                self.dey();
            }

            // STA instructions
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                self.sta(operand_address);
            }

            // LDX instructions
            0xA2 | 0xA6 | 0xAE | 0xB6 | 0xBE => {
                self.ldx(operand_address, &instruction.addressing_mode);
            }

            // LDY instructions
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                self.ldy(operand_address, &instruction.addressing_mode);
            }

            // Jump and subroutine instructions
            0x20 => {
                self.jsr(operand_address);
            }
            0x60 => {
                self.rts();
            }
            0x4C => {
                self.jmp(operand_address);
            }

            // Branch instructions
            0x10 => {
                self.bpl();
            }
            0x30 => {
                self.bmi();
            }
            0x90 => {
                self.bcc();
            }
            0xB0 => {
                self.bcs();
            }
            0xD0 => {
                self.bne();
            }
            0xF0 => {
                self.beq();
            }

            // BIT instructions
            0x24 | 0x2C => {
                self.bit(operand_address, &instruction.addressing_mode);
            }

            // ADC instructions
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                self.adc(operand_address, &instruction.addressing_mode);
            }

            // SBC instructions
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                self.sbc(operand_address, &instruction.addressing_mode);
            }

            // LSR instructions
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                self.lsr(operand_address, &instruction.addressing_mode);
            }

            // DEC instructions
            0xC6 | 0xD6 | 0xCE | 0xDE => {
                self.dec(operand_address, &instruction.addressing_mode);
            }

            // Logic instructions
            0x29 => {
                self.and(operand_address, &instruction.addressing_mode);
            }

            // Flag instructions
            0x18 => {
                self.set_flag(StatusFlag::Carry, false);
            } // CLC
            0x38 => {
                self.set_flag(StatusFlag::Carry, true);
            } // SEC

            // interrupts
            0x40 => {
                self.rti();
            }
            0xEA => {
                self.nop();
            }
            _ => {
                println!("Unknown opcode: {:#X}", instruction.opcode);
            }
        }

        if instruction.mnemonic != "JSR"
            && instruction.mnemonic != "JMP"
            && instruction.mnemonic != "RTS"
        {
            // PC was already incremented by 1 to skip the opcode
            // Now increment by (bytes - 1) to skip the operands
            self.pc += (instruction.bytes - 1) as u16;
        }

        self.instruction_count += 1;

        true
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
            AddressingMode::None => panic!(
                "Addressing mode {} not supported!",
                instruction.addressing_mode
            ),
        }
    }

    fn brk(&mut self, _address: u16, _addressing_mode: &AddressingMode) -> u8 {
        0
    }

    // branch on carry set
    fn bcs(&mut self) {
        self.branch(self.get_flag(StatusFlag::Carry));
    }

    // branch on carry clear
    fn bcc(&mut self) {
        self.branch(!self.get_flag(StatusFlag::Carry));
    }

    // branch on equal
    fn tax(&mut self) {
        self.x = self.a;
        self.set_flag(StatusFlag::Zero, self.x == 0);
        self.set_flag(StatusFlag::Negative, self.x & 0x80 != 0);
    }

    // increment X register
    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.set_flag(StatusFlag::Zero, self.x == 0);
        self.set_flag(StatusFlag::Negative, self.x & 0x80 != 0);
    }

    // No operation
    fn nop(&mut self) {}

    // jump to address
    fn jmp(&mut self, address: u16) {
        self.pc = address;
    }

    // store accumulator in memory
    fn sta(&mut self, address: u16) {
        self.write_byte(address, self.a);
    }

    // transfer accumulator to X register
    fn txa(&mut self) {
        self.a = self.x;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    // transfer accumulator to Y register
    fn tya(&mut self) {
        self.a = self.y;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    // transfer Y register to accumulator
    fn tay(&mut self) {
        self.y = self.a;
        self.set_flag(StatusFlag::Zero, self.y == 0);
        self.set_flag(StatusFlag::Negative, self.y & 0x80 != 0);
    }

    // increment Y register
    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.set_flag(StatusFlag::Zero, self.y == 0);
        self.set_flag(StatusFlag::Negative, self.y & 0x80 != 0);
    }

    // decrement X register
    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.set_flag(StatusFlag::Zero, self.x == 0);
        self.set_flag(StatusFlag::Negative, self.x & 0x80 != 0);
    }

    // decrement Y register
    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.set_flag(StatusFlag::Zero, self.y == 0);
        self.set_flag(StatusFlag::Negative, self.y & 0x80 != 0);
    }

    // load X register with value at address
    fn ldx(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        self.x = value;
        self.set_flag(StatusFlag::Zero, self.x == 0);
        self.set_flag(StatusFlag::Negative, self.x & 0x80 != 0);
    }

    // load Y register with value at address
    fn ldy(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        self.y = value;
        self.set_flag(StatusFlag::Zero, self.y == 0);
        self.set_flag(StatusFlag::Negative, self.y & 0x80 != 0);
    }

    // return from interrupt
    fn rti(&mut self) {
        // Pull status from stack (with B flag ignored)
        let status = self.pull_stack();
         // Set unused flag
        self.p = StatusFlag::from_bits_truncate(status & 0b1110_1111 | 0b0010_0000);
        // Pull PC from stack (low then high)
        let pcl = self.pull_stack();
        let pch = self.pull_stack();
        self.pc = (pch as u16) << 8 | (pcl as u16);
    }

    fn push_stack(&mut self, value: u8) {
        self.write_byte(0x0100 + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pull_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read_byte(0x0100 + self.sp as u16)
    }

    // load accumulator with value at address
    fn lda(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        self.a = value;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    // OR memory with accumulator
    fn ora(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        self.a |= value;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    // AND memory with accumulator
    fn and(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        self.a &= value;
        self.set_flag(StatusFlag::Zero, self.a == 0);
        self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
    }

    // add memory to accumulator with carry
    fn adc(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        let carry = if self.get_flag(StatusFlag::Carry) {
            1
        } else {
            0
        };
        let (sum1, carry1) = self.a.overflowing_add(value);
        let (sum2, carry2) = sum1.overflowing_add(carry);
        self.set_flag(StatusFlag::Carry, carry1 || carry2);
        self.set_flag(StatusFlag::Zero, sum2 == 0);
        self.set_flag(StatusFlag::Negative, sum2 & 0x80 != 0);
        self.set_flag(
            StatusFlag::Overflow,
            ((self.a ^ sum2) & (value ^ sum2) & 0x80) != 0,
        );
        self.a = sum2;
    }

    // subtract memory from accumulator with carry
    // Note: 6502 uses two's complement for subtraction
    // so we need to invert the bits of the value and add 1
    // to perform the subtraction.
    fn sbc(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        let carry = if self.get_flag(StatusFlag::Carry) {
            1
        } else {
            0
        };
        let value = value ^ 0xFF; // one's complement for subtraction
        let (sum1, carry1) = self.a.overflowing_add(value);
        let (sum2, carry2) = sum1.overflowing_add(carry);
        self.set_flag(StatusFlag::Carry, carry1 || carry2);
        self.set_flag(StatusFlag::Zero, sum2 == 0);
        self.set_flag(StatusFlag::Negative, sum2 & 0x80 != 0);
        self.set_flag(
            StatusFlag::Overflow,
            ((self.a ^ sum2) & (value ^ sum2) & 0x80) != 0,
        );
        self.a = sum2;
    }

    // compare memory and index X
    fn cpx(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        let result = self.x.wrapping_sub(value);
        self.set_flag(StatusFlag::Carry, self.x >= value);
        self.set_flag(StatusFlag::Zero, self.x == value);
        self.set_flag(StatusFlag::Negative, result & 0x80 != 0);
    }

    // compare memory with accumulator
    fn cmp(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        let result = self.a.wrapping_sub(value);
        self.set_flag(StatusFlag::Carry, self.a >= value);
        self.set_flag(StatusFlag::Zero, self.a == value);
        self.set_flag(StatusFlag::Negative, result & 0x80 != 0);
    }

    // compare memory and index Y
    fn cpy(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        let result = self.y.wrapping_sub(value);
        self.set_flag(StatusFlag::Carry, self.y >= value);
        self.set_flag(StatusFlag::Zero, self.y == value);
        self.set_flag(StatusFlag::Negative, result & 0x80 != 0);
    }

    // branch on equal (zero flag set)
    fn beq(&mut self) {
        self.branch(self.get_flag(StatusFlag::Zero));
    }

    // branch on zero flag = 0
    fn bne(&mut self) {
        self.branch(!self.get_flag(StatusFlag::Zero));
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let offset = self.read_byte(self.pc) as i8;
            self.pc = self.pc.wrapping_add(offset as u16);
        }
    }

    // Core disassembly logic - formats a single instruction at given address
    fn disassemble_instruction_at(&self, pc: u16) -> (String, u16) {
        let opcode = self.read_byte(pc);
        if let Some(instr) = INSTRUCTIONS.get(&opcode) {
            let mut bytes = vec![opcode];
            for i in 1..instr.bytes {
                bytes.push(self.read_byte(pc + i as u16));
            }

            let byte_str = bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");

            let formatted = format!(
                "${:04X}: {:8} {} {:?}",
                pc, byte_str, instr.mnemonic, instr.addressing_mode
            );

            (formatted, instr.bytes as u16)
        } else {
            (format!("${:04X}: {:02X}    ???", pc, opcode), 1)
        }
    }

    pub fn disassemble(&self, start: u16, end: u16) {
        let mut pc = start;
        while pc < end {
            let (formatted, bytes_consumed) = self.disassemble_instruction_at(pc);
            println!("{}", formatted);
            pc += bytes_consumed;
        }
    }

    pub fn disassemble_to_string(&self, start: u16, end: u16) -> Vec<String> {
        let mut result = Vec::new();
        let mut pc = start;
        while pc < end {
            let (formatted, bytes_consumed) = self.disassemble_instruction_at(pc);
            result.push(formatted);
            pc += bytes_consumed;
        }
        result
    }

    pub fn disassemble_current_instruction(&self) -> String {
        let (formatted, _) = self.disassemble_instruction_at(self.pc);
        formatted
    }

    // Jump to SubRoutine
    fn jsr(&mut self, address: u16) {
        let return_addr = self.pc + 1;
        self.push_stack((return_addr >> 8) as u8); // push high byte
        self.push_stack((return_addr & 0xFF) as u8); // push low byte
        self.pc = address;
    }

    // Return from Subroutine
    fn rts(&mut self) {
        let pcl = self.pull_stack();
        let pch = self.pull_stack();
        self.pc = ((pch as u16) << 8 | (pcl as u16)).wrapping_add(1);
    }

    // branch on minus (negative flag set)
    fn bmi(&mut self) {
        self.branch(self.get_flag(StatusFlag::Negative));
    }

    // branch on plus (negative flag clear)
    fn bpl(&mut self) {
        self.branch(!self.get_flag(StatusFlag::Negative));
    }

    // test bits in memory with accumulator
    fn bit(&mut self, address: u16, addressing_mode: &AddressingMode) {
        let value = match addressing_mode {
            AddressingMode::Immediate => self.read_byte(address),
            _ => self.read_byte(address),
        };

        let result = self.a & value;
        self.set_flag(StatusFlag::Zero, result == 0);
        self.set_flag(StatusFlag::Negative, value & 0x80 != 0);
        self.set_flag(StatusFlag::Overflow, value & 0x40 != 0);
    }

    // Logical Shift Right
    fn lsr(&mut self, address: u16, addressing_mode: &AddressingMode) {
        match addressing_mode {
            AddressingMode::Implied => {
                let carry = self.a & 0x01 != 0;
                self.a >>= 1;
                self.set_flag(StatusFlag::Carry, carry);
                self.set_flag(StatusFlag::Zero, self.a == 0);
                self.set_flag(StatusFlag::Negative, false);
            }
            _ => {
                let mut value = self.read_byte(address);
                let carry = value & 0x01 != 0;
                value >>= 1;
                self.write_byte(address, value);
                self.set_flag(StatusFlag::Carry, carry);
                self.set_flag(StatusFlag::Zero, value == 0);
                self.set_flag(StatusFlag::Negative, false);
            }
        }
    }

    // Decrement memory
    fn dec(&mut self, address: u16, _addressing_mode: &AddressingMode) {
        let mut value = self.read_byte(address);
        value = value.wrapping_sub(1);
        self.write_byte(address, value);
        self.set_flag(StatusFlag::Zero, value == 0);
        self.set_flag(StatusFlag::Negative, value & 0x80 != 0);
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
        assert_eq!(cpu.read_byte(PROGRAM_ADDRESS + 0), 0x42);
        assert_eq!(cpu.read_byte(PROGRAM_ADDRESS + 1), 0x42);
    }

    #[test]
    fn test_run_program_with_4_instructions() {
        // assembly:
        // LDA #$C0     /* load A with 0xC0 */
        // TAX          /* copy A to X */
        // INX          /* increment X */
        // BRK          /* break */
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00], PROGRAM_ADDRESS);
        cpu.pc = PROGRAM_ADDRESS;
        cpu.run(false);
        assert_eq!(cpu.x, 0xC1);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0x42, 0x00], PROGRAM_ADDRESS);
        cpu.pc = PROGRAM_ADDRESS;
        cpu.run(true);
        assert_eq!(cpu.a, 0x42);
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0x00, 0x00], PROGRAM_ADDRESS);
        cpu.pc = 0x8000;
        cpu.run(false);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.get_flag(StatusFlag::Zero));
    }

    #[test]
    fn test_lda_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0xFF, 0x00], PROGRAM_ADDRESS);
        cpu.pc = 0x8000;
        cpu.run(false);
        assert_eq!(cpu.a, 0xFF);
        assert!(cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x10, 0x77); // value at $10
        cpu.load_program(vec![0xA5, 0x10, 0x00], PROGRAM_ADDRESS);
        cpu.pc = 0x8000;
        cpu.run(true);
        assert_eq!(cpu.a, 0x77);
    }
}
