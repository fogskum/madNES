use crate::cpu::flags::StatusFlag;
use crate::cpu::instructions::{get_instruction, Instruction};
use crate::cpu::memory::{AddressingMode, Memory, NesMemory};
use crate::error::{CpuError, CpuResult, EmulatorError, IoError};
use crate::rom::Rom;
use crate::utils::bit_utils;
use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
const PROGRAM_ADDRESS: u16 = 0x8000;

/// Represents the 6502 CPU, including registers, program counter,
/// stack pointer, status flags, memory, and cycle count.
/// Provides methods for executing instructions, managing memory,
/// and simulating CPU behavior.
pub struct Cpu {
    // Accumulator
    a: u8,

    // X and Y registers
    x: u8,
    y: u8,

    // Program counter
    // Stores the address of the next byte for the CPU to read.
    // Increases with each clock or can be directly set in a branch to jump to
    // different parts of the program, like an if-statement.
    pc: u16,

    // Stack pointer
    // Points to an address somewhere in the memory (bus)
    // Incremented/decremented as we pull things from the stack
    sp: u8,

    // Status register
    p: StatusFlag,
    memory: NesMemory,
    cycles: u64,
    instruction_count: u64,
}

impl Memory for Cpu {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory.read_byte(address)
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory.write_byte(address, value)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFD,
            p: StatusFlag::empty(),
            memory: NesMemory::new(),
            cycles: 0,
            instruction_count: 0,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn irq(&mut self) {
        // IRQ can be masked by the interrupt disable flag
        if self.get_flag(StatusFlag::InterruptDisable) {
            return;
        }

        // Push PC high byte first, then low byte
        let pc_high = (self.pc >> 8) as u8;
        let pc_low = (self.pc & 0xFF) as u8;
        self.push_stack(pc_high);
        self.push_stack(pc_low);

        // Push status register with break flag clear (hardware interrupt)
        let mut status = self.p.bits();
        status &= !StatusFlag::Break.bits(); // Clear break flag
        status |= StatusFlag::Unused.bits(); // Set unused flag
        self.push_stack(status);

        // Set the interrupt disable flag
        self.set_flag(StatusFlag::InterruptDisable, true);

        // Set PC to the address stored at 0xFFFE (IRQ vector)
        self.pc = self.read_word(0xFFFE);
    }

    pub fn nmi(&mut self) {
        // NMI cannot be masked

        // Push PC high byte first, then low byte
        let pc_high = (self.pc >> 8) as u8;
        let pc_low = (self.pc & 0xFF) as u8;
        self.push_stack(pc_high);
        self.push_stack(pc_low);

        // Push status register with break flag clear (hardware interrupt)
        let mut status = self.p.bits();
        status &= !StatusFlag::Break.bits(); // Clear break flag
        status |= StatusFlag::Unused.bits(); // Set unused flag
        self.push_stack(status);

        // Set the interrupt disable flag
        self.set_flag(StatusFlag::InterruptDisable, true);

        // Set PC to the address stored at 0xFFFA (NMI vector)
        self.pc = self.read_word(0xFFFA);
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

    fn update_nz_flags(&mut self, value: u8) {
        self.set_flag(StatusFlag::Zero, value == 0);
        self.set_flag(StatusFlag::Negative, value & 0x80 != 0);
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

    pub fn get_cycles(&self) -> u64 {
        self.cycles
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_instruction_count(&self) -> u64 {
        self.instruction_count
    }

    /// Get CHR ROM data for graphics rendering
    pub fn get_chr_rom(&self) -> Option<&[u8]> {
        self.memory.get_chr_rom()
    }

    /// Get PRG ROM data
    pub fn get_prg_rom(&self) -> Option<&[u8]> {
        self.memory.get_prg_rom()
    }

    // Loads the given program to PRG ROM memory range (0x8000-0xFFFF)
    pub fn load_program(&mut self, program: Vec<u8>, address: u16) -> Result<(), String> {
        // For NES, programs are typically loaded to PRG ROM area (0x8000+)
        if address < PROGRAM_ADDRESS {
            return Err(format!(
                "Programs should be loaded to PRG ROM area (0x8000+), got address {:#X}",
                address
            ));
        }

        // Create a fake ROM from the program data for testing
        let rom = Rom {
            prg_rom: program.clone(),
            chr_rom: vec![0; 8192], // 8KB CHR ROM
            mirror_mode: crate::rom::MirrorMode::Horizontal,
            mapper: 0,          // NROM
            prg_ram_size: 8192, // 8KB PRG RAM
        };

        // Load ROM into memory
        self.memory.load_prg_rom(rom);

        // Set reset vector to point to the program start
        self.write_word(0xFFFC, address);
        self.reset();

        // Disassemble for debugging
        self.disassemble(address, address + program.len() as u16);
        Ok(())
    }

    pub fn load_rom(&mut self, rom: Rom) {
        self.memory.load_prg_rom(rom);
    }

    pub fn run(&mut self, show_disassembly: bool) {
        loop {
            if show_disassembly {
                println!("PC: {:#X}", self.pc);
                let upper_address = self.pc + 4;
                self.disassemble(self.pc, upper_address);
            }

            match self.step() {
                Ok(should_continue) => {
                    if !should_continue {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("CPU error: {}", e);
                    break;
                }
            }
        }
    }

    pub fn step(&mut self) -> CpuResult<bool> {
        // Execute one instruction and return true if should continue
        let opcode = self.read_byte(self.pc);

        // get instruction metadata for opcode
        let instruction = get_instruction(opcode).ok_or(CpuError::UnknownOpcode {
            opcode,
            pc: self.pc,
        })?;

        // Collect instruction bytes for logging
        let mut instruction_bytes = vec![opcode];
        for i in 1..instruction.bytes {
            instruction_bytes.push(self.read_byte(self.pc + i as u16));
        }

        // Get disassembly for logging (now handled in log_cpu_state)
        // let disassembly = format!("{} ({})", instruction.mnemonic, format!("{:?}", instruction.addressing_mode));

        // Log CPU state before instruction execution
        self.log_cpu_state(&instruction_bytes, opcode);

        self.pc += 1;

        // get operand address for instruction
        let operand_address = self.get_operand_address(instruction)?;

        match instruction.opcode {
            0x00 => {
                self.brk();
                return Ok(false); // Stop execution on BRK
            }
            // LDA instructions
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                self.lda(operand_address);
            }

            // ORA instructions
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                self.ora(operand_address);
            }

            // CMP instructions
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                self.cmp(operand_address);
            }

            // CPX instructions
            0xE0 | 0xE4 | 0xEC => {
                self.cpx(operand_address);
            }

            // CPY instructions
            0xC0 | 0xC4 | 0xCC => {
                self.cpy(operand_address);
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
                self.ldx(operand_address);
            }

            // LDY instructions
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                self.ldy(operand_address);
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
            0x50 => {
                self.bvc();
            }
            0x70 => {
                self.bvs();
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
                self.bit(operand_address);
            }

            // ADC instructions
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                self.adc(operand_address);
            }

            // SBC instructions
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                self.sbc(operand_address);
            }

            // LSR instructions
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                self.lsr(operand_address, &instruction.addressing_mode);
            }

            // DEC instructions
            0xC6 | 0xD6 | 0xCE | 0xDE => {
                self.dec(operand_address);
            }

            // INC instructions
            0xE6 | 0xF6 | 0xEE | 0xFE => {
                self.inc(operand_address);
            }

            // Logic instructions
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                self.and(operand_address);
            }
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                self.eor(operand_address);
            }

            // Shift instructions
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                self.asl(operand_address, &instruction.addressing_mode);
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

        // jump instructions already set the new PC value
        if instruction.mnemonic != "JSR"
            && instruction.mnemonic != "JMP"
            && instruction.mnemonic != "RTS"
        {
            // PC was already incremented by 1 to skip the opcode
            // Now increment by (bytes - 1) to skip the operands
            self.pc += (instruction.bytes - 1) as u16;
        }

        self.instruction_count += 1;
        self.cycles += instruction.cycles as u64;

        Ok(true)
    }

    fn get_operand_address(&self, instruction: &Instruction) -> CpuResult<u16> {
        match instruction.addressing_mode {
            AddressingMode::Immediate => Ok(self.pc),
            AddressingMode::ZeroPage => Ok(self.read_byte(self.pc) as u16),
            AddressingMode::ZeroPageX => {
                let addr = self.read_byte(self.pc);
                Ok(addr.wrapping_add(self.x) as u16)
            }
            AddressingMode::ZeroPageY => {
                let addr = self.read_byte(self.pc);
                Ok(addr.wrapping_add(self.y) as u16)
            }
            AddressingMode::Absolute => Ok(self.read_word(self.pc)),
            AddressingMode::AbsoluteX => Ok(self.read_word(self.pc).wrapping_add(self.x as u16)),
            AddressingMode::AbsoluteY => Ok(self.read_word(self.pc).wrapping_add(self.y as u16)),
            AddressingMode::IndirectX => {
                let base = self.read_byte(self.pc);
                let ptr = base.wrapping_add(self.x);
                let lo = self.read_byte(ptr as u16) as u16;
                let hi = self.read_byte(ptr.wrapping_add(1) as u16) as u16;
                Ok((hi << 8) | lo)
            }
            AddressingMode::IndirectY => {
                let base = self.read_byte(self.pc);
                let lo = self.read_byte(base as u16) as u16;
                let hi = self.read_byte(base.wrapping_add(1) as u16) as u16;
                Ok(((hi << 8) | lo).wrapping_add(self.y as u16))
            }
            AddressingMode::Indirect => {
                // Used for JMP ($xxxx) - indirect jump
                let indirect_addr = self.read_word(self.pc);
                // 6502 bug: if the low byte is 0xFF, high byte is read from same page
                if indirect_addr & 0xFF == 0xFF {
                    let lo = self.read_byte(indirect_addr) as u16;
                    let hi = self.read_byte(indirect_addr & 0xFF00) as u16;
                    Ok((hi << 8) | lo)
                } else {
                    Ok(self.read_word(indirect_addr))
                }
            }
            AddressingMode::Relative => {
                // Used for branch instructions
                let offset = self.read_byte(self.pc) as i8;
                Ok((self.pc + 1).wrapping_add(offset as u16))
            }
            AddressingMode::Implied => Ok(0), // Implied has no operand address
            AddressingMode::None => Err(CpuError::InvalidInstruction {
                address: self.pc,
                reason: format!(
                    "Addressing mode {} not supported!",
                    instruction.addressing_mode
                ),
            }),
        }
    }

    fn brk(&mut self) -> u8 {
        // BRK is a software interrupt
        // Increment PC by 1 to point to the next instruction after BRK
        self.pc += 1;

        // Push PC high byte first, then low byte
        let pc_high = (self.pc >> 8) as u8;
        let pc_low = (self.pc & 0xFF) as u8;
        self.push_stack(pc_high);
        self.push_stack(pc_low);

        // Push status register with break flag set (software interrupt)
        let mut status = self.p.bits();
        status |= StatusFlag::Break.bits(); // Set break flag
        status |= StatusFlag::Unused.bits(); // Set unused flag
        self.push_stack(status);

        // Set the interrupt disable flag
        self.set_flag(StatusFlag::InterruptDisable, true);

        // Set PC to the address stored at 0xFFFE (IRQ vector, same as IRQ)
        self.pc = self.read_word(0xFFFE);

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
        self.update_nz_flags(self.x);
    }

    // increment X register
    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.update_nz_flags(self.x);
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
        self.update_nz_flags(self.a);
    }

    // transfer accumulator to Y register
    fn tya(&mut self) {
        self.a = self.y;
        self.update_nz_flags(self.a);
    }

    // transfer Y register to accumulator
    fn tay(&mut self) {
        self.y = self.a;
        self.update_nz_flags(self.y);
    }

    // increment Y register
    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.update_nz_flags(self.y);
    }

    // decrement X register
    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.update_nz_flags(self.x);
    }

    // decrement Y register
    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.update_nz_flags(self.y);
    }

    // load X register with value at address
    fn ldx(&mut self, address: u16) {
        let value = self.read_byte(address);
        self.x = value;
        self.update_nz_flags(self.x);
    }

    // load Y register with value at address
    fn ldy(&mut self, address: u16) {
        let value = self.read_byte(address);
        self.y = value;
        self.update_nz_flags(self.y);
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

    // fn push_stack_16(&mut self, value: u16) {
    //     let lo = (value & 0xFF) as u8;
    //     let hi = (value >> 8) as u8;
    //     self.push_stack(lo);
    //     self.push_stack(hi);
    // }

    fn pull_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.read_byte(0x0100 + self.sp as u16)
    }

    // load accumulator with value at address
    fn lda(&mut self, address: u16) {
        let value = self.read_byte(address);
        self.a = value;
        self.update_nz_flags(self.a);
    }

    // OR memory with accumulator
    fn ora(&mut self, address: u16) {
        let value = self.read_byte(address);
        self.a |= value;
        self.update_nz_flags(self.a);
    }

    // AND memory with accumulator
    fn and(&mut self, address: u16) {
        let value = self.read_byte(address);
        self.a &= value;
        self.update_nz_flags(self.a);
    }

    // EOR (Exclusive OR) memory with accumulator
    fn eor(&mut self, address: u16) {
        let value = self.read_byte(address);
        self.a ^= value;
        self.update_nz_flags(self.a);
    }

    // add memory to accumulator with carry
    fn adc(&mut self, address: u16) {
        let value = self.read_byte(address);
        let carry = if self.get_flag(StatusFlag::Carry) {
            1
        } else {
            0
        };
        let (sum1, carry1) = self.a.overflowing_add(value);
        let (sum2, carry2) = sum1.overflowing_add(carry);
        self.set_flag(StatusFlag::Carry, carry1 || carry2);
        self.update_nz_flags(sum2);
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
    fn sbc(&mut self, address: u16) {
        let mut value = self.read_byte(address);
        let carry = if self.get_flag(StatusFlag::Carry) {
            1
        } else {
            0
        };

        value ^= 0xFF; // one's complement for subtraction
        let (sum1, carry1) = self.a.overflowing_add(value);
        let (sum2, carry2) = sum1.overflowing_add(carry);
        self.set_flag(StatusFlag::Carry, carry1 || carry2);
        self.update_nz_flags(sum2);
        self.set_flag(
            StatusFlag::Overflow,
            ((self.a ^ sum2) & (value ^ sum2) & 0x80) != 0,
        );
        self.a = sum2;
    }

    // compare memory and index X
    fn cpx(&mut self, address: u16) {
        let value = self.read_byte(address);
        let result = self.x.wrapping_sub(value);
        self.set_flag(StatusFlag::Carry, self.x >= value);
        self.set_flag(StatusFlag::Zero, self.x == value);
        self.set_flag(StatusFlag::Negative, result & 0x80 != 0);
    }

    // compare memory with accumulator
    fn cmp(&mut self, address: u16) {
        let value = self.read_byte(address);
        let result = self.a.wrapping_sub(value);
        self.set_flag(StatusFlag::Carry, self.a >= value);
        self.set_flag(StatusFlag::Zero, self.a == value);
        self.set_flag(StatusFlag::Negative, result & 0x80 != 0);
    }

    // compare memory and index Y
    fn cpy(&mut self, address: u16) {
        let value = self.read_byte(address);
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

    // branch on overflow clear
    fn bvc(&mut self) {
        self.branch(!self.get_flag(StatusFlag::Overflow));
    }

    // branch on overflow set
    fn bvs(&mut self) {
        self.branch(self.get_flag(StatusFlag::Overflow));
    }

    fn branch(&mut self, condition: bool) {
        if condition {
            let old_pc = self.pc;
            let offset = self.read_byte(self.pc) as i8;
            self.pc = self.pc.wrapping_add(offset as u16);

            // add cycles for branch instruction
            self.cycles += 1;

            // Check for page crossing
            if bit_utils::page_crossed(old_pc, self.pc) {
                // Extra cycle for page crossing
                self.cycles += 1;
            }
        }
    }

    // Core disassembly logic - formats a single instruction at given address
    fn disassemble_instruction_at(&self, pc: u16) -> (String, u16) {
        let opcode = self.read_byte(pc);
        if let Some(instr) = get_instruction(opcode) {
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
                "${:04X}: {:8} {} ({:?})",
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
    fn bit(&mut self, address: u16) {
        let value = self.read_byte(address);
        let result = self.a & value;
        self.set_flag(StatusFlag::Zero, result == 0);
        self.set_flag(StatusFlag::Negative, value & 0x80 != 0);
        self.set_flag(StatusFlag::Overflow, value & 0x40 != 0);
    }

    // Arithmetic Shift Left
    fn asl(&mut self, address: u16, addressing_mode: &AddressingMode) {
        match addressing_mode {
            AddressingMode::Implied => {
                let carry = self.a & 0x80 != 0;
                self.a <<= 1;
                self.set_flag(StatusFlag::Carry, carry);
                self.set_flag(StatusFlag::Zero, self.a == 0);
                self.set_flag(StatusFlag::Negative, self.a & 0x80 != 0);
            }
            _ => {
                let mut value = self.read_byte(address);
                let carry = value & 0x80 != 0;
                value <<= 1;
                self.write_byte(address, value);
                self.set_flag(StatusFlag::Carry, carry);
                self.set_flag(StatusFlag::Zero, value == 0);
                self.set_flag(StatusFlag::Negative, value & 0x80 != 0);
            }
        }
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
    fn dec(&mut self, address: u16) {
        let mut value = self.read_byte(address);
        value = value.wrapping_sub(1);
        self.write_byte(address, value);
        self.update_nz_flags(value);
    }

    // Increment memory
    fn inc(&mut self, address: u16) {
        let mut value = self.read_byte(address);
        value = value.wrapping_add(1);
        self.write_byte(address, value);
        self.update_nz_flags(value);
    }

    /// Log CPU state in nestest.log format
    fn log_cpu_state(&self, instruction_bytes: &[u8], opcode: u8) {
        // Get instruction info
        let instruction = get_instruction(opcode).unwrap();

        // Build disassembly string based on addressing mode
        let disassembly = match instruction.addressing_mode {
            AddressingMode::Implied => instruction.mnemonic.to_string(),
            AddressingMode::Immediate => {
                if instruction_bytes.len() > 1 {
                    format!("{} #${:02X}", instruction.mnemonic, instruction_bytes[1])
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::ZeroPage => {
                if instruction_bytes.len() > 1 {
                    format!("{} ${:02X}", instruction.mnemonic, instruction_bytes[1])
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::ZeroPageX => {
                if instruction_bytes.len() > 1 {
                    format!("{} ${:02X},X", instruction.mnemonic, instruction_bytes[1])
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::ZeroPageY => {
                if instruction_bytes.len() > 1 {
                    format!("{} ${:02X},Y", instruction.mnemonic, instruction_bytes[1])
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::Absolute => {
                if instruction_bytes.len() > 2 {
                    let addr = (instruction_bytes[2] as u16) << 8 | instruction_bytes[1] as u16;
                    format!("{} ${:04X}", instruction.mnemonic, addr)
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::AbsoluteX => {
                if instruction_bytes.len() > 2 {
                    let addr = (instruction_bytes[2] as u16) << 8 | instruction_bytes[1] as u16;
                    format!("{} ${:04X},X", instruction.mnemonic, addr)
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::AbsoluteY => {
                if instruction_bytes.len() > 2 {
                    let addr = (instruction_bytes[2] as u16) << 8 | instruction_bytes[1] as u16;
                    format!("{} ${:04X},Y", instruction.mnemonic, addr)
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::IndirectX => {
                if instruction_bytes.len() > 1 {
                    format!("{} (${:02X},X)", instruction.mnemonic, instruction_bytes[1])
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::IndirectY => {
                if instruction_bytes.len() > 1 {
                    format!("{} (${:02X}),Y", instruction.mnemonic, instruction_bytes[1])
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::Indirect => {
                if instruction_bytes.len() > 2 {
                    let addr = (instruction_bytes[2] as u16) << 8 | instruction_bytes[1] as u16;
                    format!("{} (${:04X})", instruction.mnemonic, addr)
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::Relative => {
                if instruction_bytes.len() > 1 {
                    let offset = instruction_bytes[1] as i8;
                    let target = (self.pc + 2).wrapping_add(offset as u16);
                    format!("{} ${:04X}", instruction.mnemonic, target)
                } else {
                    instruction.mnemonic.to_string()
                }
            }
            AddressingMode::None => instruction.mnemonic.to_string(),
        };

        // Format: PC  INSTRUCTION_BYTES  DISASSEMBLY  A:XX X:XX Y:XX P:XX SP:XX CYC:XXX
        let mut log_line = format!("{:04X}  ", self.pc);

        // Add instruction bytes (pad to 8 characters)
        let bytes_str = instruction_bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        log_line.push_str(&format!("{:8} ", bytes_str));

        // Add disassembly (pad to 32 characters)
        log_line.push_str(&format!("{:32} ", disassembly));

        // Add register states
        log_line.push_str(&format!(
            "A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{:3}",
            self.a,
            self.x,
            self.y,
            self.p.bits(),
            self.sp,
            self.cycles
        ));

        // Write to log file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("madnes.log")
        {
            writeln!(file, "{}", log_line).ok();
        }
    }

    // Initialize the log file (clear previous content)
    pub fn init_log() -> Result<(), EmulatorError> {
        std::fs::File::create("madnes.log").map_err(|e| {
            EmulatorError::Io(IoError::WriteError(format!(
                "Failed to create log file: {}",
                e
            )))
        })?;
        Ok(())
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

        // Create a ROM with a reset vector at 0x4242
        let mut rom_data = vec![0; 0x8000]; // 32KB ROM
        rom_data[0x7FFC] = 0x42; // Low byte of reset vector at 0xFFFC
        rom_data[0x7FFD] = 0x42; // High byte of reset vector at 0xFFFD

        let rom = Rom {
            prg_rom: rom_data,
            chr_rom: vec![0; 8192],
            mirror_mode: crate::rom::MirrorMode::Horizontal,
            mapper: 0,
            prg_ram_size: 8192,
        };

        cpu.memory.load_prg_rom(rom);
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
        cpu.load_program(program, PROGRAM_ADDRESS).unwrap();
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
        cpu.load_program(vec![0xA9, 0xC0, 0xAA, 0xE8, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = PROGRAM_ADDRESS;
        cpu.run(false);
        assert_eq!(cpu.x, 0xC1);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0x42, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = PROGRAM_ADDRESS;
        cpu.run(true);
        assert_eq!(cpu.a, 0x42);
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_lda_zero_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0x00, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        cpu.run(false);
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.get_flag(StatusFlag::Zero));
    }

    #[test]
    fn test_lda_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.load_program(vec![0xA9, 0xFF, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        cpu.run(false);
        assert_eq!(cpu.a, 0xFF);
        assert!(cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x10, 0x77); // value at $10
        cpu.load_program(vec![0xA5, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        cpu.run(true);
        assert_eq!(cpu.a, 0x77);
    }

    #[test]
    fn test_bvc_branch_taken() {
        let mut cpu = Cpu::new();
        // BVC $02 - should branch 2 bytes forward if overflow is clear
        cpu.load_program(vec![0x50, 0x02, 0xEA, 0xEA, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.set_flag(StatusFlag::Overflow, false); // Clear overflow flag (though it should already be clear)
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute BVC
                            // PC should be at 0x8000 + 2 (instruction length) + 2 (branch offset) = 0x8004
        assert_eq!(cpu.pc, 0x8004);
    }

    #[test]
    fn test_bvc_branch_not_taken() {
        let mut cpu = Cpu::new();
        // BVC $02 - should not branch if overflow is set
        cpu.load_program(vec![0x50, 0x02, 0xEA, 0xEA, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.set_flag(StatusFlag::Overflow, true); // Set overflow flag AFTER load_program
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute BVC
                            // PC should be at 0x8000 + 2 (instruction length) = 0x8002
        assert_eq!(cpu.pc, 0x8002);
    }

    #[test]
    fn test_bvs_branch_taken() {
        let mut cpu = Cpu::new();
        // BVS $02 - should branch 2 bytes forward if overflow is set
        cpu.load_program(vec![0x70, 0x02, 0xEA, 0xEA, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.set_flag(StatusFlag::Overflow, true); // Set overflow flag
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute BVS
                            // PC should be at 0x8000 + 2 (instruction length) + 2 (branch offset) = 0x8004
        assert_eq!(cpu.pc, 0x8004);
    }

    #[test]
    fn test_bvs_branch_not_taken() {
        let mut cpu = Cpu::new();
        // BVS $02 - should not branch if overflow is clear
        cpu.load_program(vec![0x70, 0x02, 0xEA, 0xEA, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.set_flag(StatusFlag::Overflow, false); // Clear overflow flag (though it should already be clear)
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute BVS
                            // PC should be at 0x8000 + 2 (instruction length) = 0x8002
        assert_eq!(cpu.pc, 0x8002);
    }

    #[test]
    fn test_inc_zeropage() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x10, 0x42); // value at $10
                                    // INC $10 - increment value at zero page address $10
        cpu.load_program(vec![0xE6, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute INC $10
        assert_eq!(cpu.read_byte(0x10), 0x43);
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_inc_zeropage_x() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x15, 0x7E); // value at $10 + X = $15
                                    // INC $10,X - increment value at zero page address $10 + X
        cpu.load_program(vec![0xF6, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.x = 0x05; // Set X AFTER load_program since it calls reset()
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute INC $10,X
        assert_eq!(cpu.read_byte(0x15), 0x7F);
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_inc_absolute() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x3000, 0x80); // value at $3000
                                      // INC $3000 - increment value at absolute address $3000
        cpu.load_program(vec![0xEE, 0x00, 0x30, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute INC $3000
        assert_eq!(cpu.read_byte(0x3000), 0x81);
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(cpu.get_flag(StatusFlag::Negative)); // 0x81 has bit 7 set
    }

    #[test]
    fn test_inc_absolute_x() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x3010, 0xFE); // value at $3000 + X = $3010
                                      // INC $3000,X - increment value at absolute address $3000 + X
        cpu.load_program(vec![0xFE, 0x00, 0x30, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.x = 0x10; // Set X AFTER load_program since it calls reset()
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute INC $3000,X
        assert_eq!(cpu.read_byte(0x3010), 0xFF);
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(cpu.get_flag(StatusFlag::Negative)); // 0xFF has bit 7 set
    }

    #[test]
    fn test_inc_wraparound_to_zero() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x10, 0xFF); // value at $10
                                    // INC $10 - increment value at zero page address $10
        cpu.load_program(vec![0xE6, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute INC $10
        assert_eq!(cpu.read_byte(0x10), 0x00); // Should wrap around to 0
        assert!(cpu.get_flag(StatusFlag::Zero)); // Zero flag should be set
        assert!(!cpu.get_flag(StatusFlag::Negative)); // Negative flag should be clear
    }

    #[test]
    fn test_inc_negative_flag() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x10, 0x7F); // value at $10
                                    // INC $10 - increment value at zero page address $10
        cpu.load_program(vec![0xE6, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute INC $10
        assert_eq!(cpu.read_byte(0x10), 0x80); // Should become 0x80
        assert!(!cpu.get_flag(StatusFlag::Zero)); // Zero flag should be clear
        assert!(cpu.get_flag(StatusFlag::Negative)); // Negative flag should be set (bit 7 is 1)
    }

    #[test]
    fn test_inc_debug() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x10, 0x42); // value at $10
        println!("Before INC: memory[0x10] = {}", cpu.read_byte(0x10));

        // INC $10 - increment value at zero page address $10
        cpu.load_program(vec![0xE6, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;

        let opcode = cpu.read_byte(cpu.pc);
        println!("Opcode: 0x{:02X}", opcode);

        let _ = cpu.step(); // Execute INC $10

        println!("After INC: memory[0x10] = {}", cpu.read_byte(0x10));
        assert_eq!(cpu.read_byte(0x10), 0x43);
    }

    #[test]
    fn test_inc_zeropage_x_debug() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x15, 0x7E); // value at $10 + X = $15

        // INC $10,X - increment value at zero page address $10 + X
        cpu.load_program(vec![0xF6, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.x = 0x05; // Set X AFTER load_program since it calls reset()

        println!(
            "Before INC: X = {}, memory[0x15] = {}",
            cpu.x,
            cpu.read_byte(0x15)
        );
        cpu.pc = 0x8000;

        let opcode = cpu.read_byte(cpu.pc);
        let operand = cpu.read_byte(cpu.pc + 1);
        println!("Opcode: 0x{:02X}, Operand: 0x{:02X}", opcode, operand);
        println!(
            "Calculated address should be: 0x{:02X} + 0x{:02X} = 0x{:02X}",
            operand,
            cpu.x,
            operand.wrapping_add(cpu.x)
        );

        let _ = cpu.step(); // Execute INC $10,X

        println!("After INC: memory[0x15] = {}", cpu.read_byte(0x15));
        println!("After INC: memory[0x10] = {}", cpu.read_byte(0x10));
        assert_eq!(cpu.read_byte(0x15), 0x7F);
    }

    #[test]
    fn test_inc_integration() {
        let mut cpu = Cpu::new();
        // Test a simple program that uses INC to count up
        // Initialize counter at address 0x20 with value 0x05
        cpu.write_byte(0x20, 0x05);

        // Program:
        // INC $20    ; increment counter at 0x20
        // INC $20    ; increment counter at 0x20 again
        // LDA $20    ; load counter into accumulator
        // BRK        ; break
        cpu.load_program(
            vec![0xE6, 0x20, 0xE6, 0x20, 0xA5, 0x20, 0x00],
            PROGRAM_ADDRESS,
        )
        .unwrap();
        cpu.pc = 0x8000;

        // Execute the program
        cpu.run(false);

        // Verify the counter was incremented twice (0x05 -> 0x07)
        assert_eq!(cpu.read_byte(0x20), 0x07);
        assert_eq!(cpu.a, 0x07); // Should also be loaded into accumulator
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_eor_immediate() {
        let mut cpu = Cpu::new();
        // EOR #$FF - should flip all bits
        cpu.load_program(vec![0x49, 0xFF, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.a = 0b10101010; // Set A to known pattern AFTER load_program
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute EOR #$FF
        assert_eq!(cpu.a, 0b01010101); // All bits should be flipped
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_eor_zero_flag() {
        let mut cpu = Cpu::new();
        // EOR #$42 - should result in zero
        cpu.load_program(vec![0x49, 0x42, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.a = 0x42; // Set A AFTER load_program
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute EOR #$42
        assert_eq!(cpu.a, 0x00);
        assert!(cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative));
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = Cpu::new();
        // ASL A - shift left accumulator
        cpu.load_program(vec![0x0A, 0x00], PROGRAM_ADDRESS).unwrap();
        cpu.a = 0b01000001; // Set A to known pattern AFTER load_program
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute ASL A
        assert_eq!(cpu.a, 0b10000010); // Should be shifted left
        assert!(!cpu.get_flag(StatusFlag::Carry)); // Bit 7 was 0, so no carry
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(cpu.get_flag(StatusFlag::Negative)); // Bit 7 is now 1
    }

    #[test]
    fn test_asl_carry_flag() {
        let mut cpu = Cpu::new();
        // ASL A - shift left accumulator
        cpu.load_program(vec![0x0A, 0x00], PROGRAM_ADDRESS).unwrap();
        cpu.a = 0b10000000; // Set A with bit 7 set AFTER load_program
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute ASL A
        assert_eq!(cpu.a, 0b00000000); // Should be shifted left, result is 0
        assert!(cpu.get_flag(StatusFlag::Carry)); // Bit 7 was 1, so carry set
        assert!(cpu.get_flag(StatusFlag::Zero)); // Result is zero
        assert!(!cpu.get_flag(StatusFlag::Negative)); // Bit 7 is now 0
    }

    #[test]
    fn test_asl_memory() {
        let mut cpu = Cpu::new();
        cpu.write_byte(0x10, 0b00110011); // Set memory value
                                          // ASL $10 - shift left memory at zero page
        cpu.load_program(vec![0x06, 0x10, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.pc = 0x8000;
        let _ = cpu.step(); // Execute ASL $10
        assert_eq!(cpu.read_byte(0x10), 0b01100110); // Should be shifted left
        assert!(!cpu.get_flag(StatusFlag::Carry)); // Bit 7 was 0, so no carry
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(!cpu.get_flag(StatusFlag::Negative)); // Bit 7 is still 0
    }

    #[test]
    fn test_eor_asl_integration() {
        let mut cpu = Cpu::new();
        // Test a program that uses EOR and ASL together
        // Initialize A with a pattern
        // EOR #$55   ; flip some bits
        // ASL A      ; shift left
        // EOR #$AA   ; flip different bits
        cpu.load_program(vec![0x49, 0x55, 0x0A, 0x49, 0xAA, 0x00], PROGRAM_ADDRESS)
            .unwrap();
        cpu.a = 0b11110000; // Set initial pattern AFTER load_program
        cpu.pc = 0x8000;

        // Execute EOR #$55
        let _ = cpu.step();
        assert_eq!(cpu.a, 0b10100101); // 0b11110000 ^ 0b01010101 = 0b10100101

        // Execute ASL A
        let _ = cpu.step();
        assert_eq!(cpu.a, 0b01001010); // 0b10100101 << 1 = 0b01001010
        assert!(cpu.get_flag(StatusFlag::Carry)); // Bit 7 was 1

        // Execute EOR #$AA
        let _ = cpu.step();
        assert_eq!(cpu.a, 0b11100000); // 0b01001010 ^ 0b10101010 = 0b11100000
        assert!(!cpu.get_flag(StatusFlag::Zero));
        assert!(cpu.get_flag(StatusFlag::Negative)); // Bit 7 is 1
    }

    #[test]
    fn test_nes_memory_mirroring() {
        let mut cpu = Cpu::new();

        // Test internal RAM mirroring (0x0000-0x07FF mirrored to 0x0800-0x1FFF)
        cpu.write_byte(0x0200, 0xAB); // Write to base RAM
        assert_eq!(cpu.read_byte(0x0200), 0xAB); // Read from base
        assert_eq!(cpu.read_byte(0x0A00), 0xAB); // Read from mirror 1 (0x0200 + 0x0800)
        assert_eq!(cpu.read_byte(0x1200), 0xAB); // Read from mirror 2 (0x0200 + 0x1000)
        assert_eq!(cpu.read_byte(0x1A00), 0xAB); // Read from mirror 3 (0x0200 + 0x1800)

        // Test writing to mirror affects base
        cpu.write_byte(0x1300, 0xCD); // Write to mirror
        assert_eq!(cpu.read_byte(0x0300), 0xCD); // Should appear in base (0x1300 & 0x07FF = 0x0300)
    }

    #[test]
    fn test_nes_memory_ppu_mirroring() {
        let mut cpu = Cpu::new();

        // Test PPU register mirroring (0x2000-0x2007 mirrored throughout 0x2000-0x3FFF)
        cpu.write_byte(0x2002, 0x55); // Write to PPU status register
        assert_eq!(cpu.read_byte(0x2002), 0x55); // Read from base
        assert_eq!(cpu.read_byte(0x200A), 0x55); // Read from mirror (0x200A & 0x0007 = 0x0002)
        assert_eq!(cpu.read_byte(0x3002), 0x55); // Read from high mirror

        // Test writing to mirror affects base
        cpu.write_byte(0x2409, 0x77); // Write to mirror (0x2409 & 0x0007 = 0x0001)
        assert_eq!(cpu.read_byte(0x2001), 0x77); // Should appear in base register
    }

    #[test]
    fn test_nes_memory_regions() {
        let mut cpu = Cpu::new();

        // Test PRG RAM area (0x6000-0x7FFF)
        cpu.write_byte(0x6000, 0x11);
        cpu.write_byte(0x7FFF, 0x22);
        assert_eq!(cpu.read_byte(0x6000), 0x11);
        assert_eq!(cpu.read_byte(0x7FFF), 0x22);

        // Test APU/IO area (0x4000-0x401F)
        cpu.write_byte(0x4000, 0x33);
        cpu.write_byte(0x4015, 0x44);
        assert_eq!(cpu.read_byte(0x4000), 0x33);
        assert_eq!(cpu.read_byte(0x4015), 0x44);
    }

    #[test]
    fn test_nes_memory_prg_rom() {
        let mut cpu = Cpu::new();

        // Load a small ROM
        let rom_data = Rom {
            prg_rom: vec![0x01, 0x02, 0x03, 0x04],
            chr_rom: vec![0; 8192],
            mirror_mode: crate::rom::MirrorMode::Horizontal,
            mapper: 0,
            prg_ram_size: 8192,
        };
        cpu.memory.load_prg_rom(rom_data);

        // Test reading from PRG ROM
        assert_eq!(cpu.read_byte(0x8000), 0x01);
        assert_eq!(cpu.read_byte(0x8001), 0x02);
        assert_eq!(cpu.read_byte(0x8002), 0x03);
        assert_eq!(cpu.read_byte(0x8003), 0x04);

        // Test 16KB ROM mirroring (if ROM is exactly 16KB, it mirrors in upper 16KB)
        let rom_16kb = Rom {
            prg_rom: vec![0xAA; 0x4000], // 16KB ROM
            chr_rom: vec![0; 8192],
            mirror_mode: crate::rom::MirrorMode::Horizontal,
            mapper: 0,
            prg_ram_size: 8192,
        };
        cpu.memory.load_prg_rom(rom_16kb);
        assert_eq!(cpu.read_byte(0x8000), 0xAA); // First 16KB
        assert_eq!(cpu.read_byte(0xC000), 0xAA); // Mirrored in second 16KB
    }
}
