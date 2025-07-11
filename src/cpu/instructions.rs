use crate::cpu::memory::AddressingMode;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub mnemonic: &'static str,
    pub opcode: u8,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
    pub bytes: u8,
}

impl Instruction {
    pub const fn new(
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

pub static INSTRUCTION_TABLE: [Option<Instruction>; 256] = {
    let mut table = [None; 256];

    // Helper macro to insert an instruction into the table
    macro_rules! insert_instruction {
        ($table:expr, $opcode:expr, $mnemonic:expr, $addressing_mode:expr, $cycles:expr, $bytes:expr) => {
            $table[$opcode as usize] = Some(Instruction::new(
                $mnemonic,
                $opcode,
                $addressing_mode,
                $cycles,
                $bytes,
            ));
        };
    }

    // Transfer instructions
    insert_instruction!(table, 0xA9, "LDA", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0xA5, "LDA", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0xB5, "LDA", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0xAD, "LDA", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0xBD, "LDA", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0xB9, "LDA", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0xA1, "LDA", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0xB1, "LDA", AddressingMode::IndirectY, 5, 2);
    insert_instruction!(table, 0xA2, "LDX", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0xA6, "LDX", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0xB6, "LDX", AddressingMode::ZeroPageY, 4, 2);
    insert_instruction!(table, 0xAE, "LDX", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0xBE, "LDX", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0xA0, "LDY", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0xA4, "LDY", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0xB4, "LDY", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0xAC, "LDY", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0xBC, "LDY", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0x85, "STA", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0x8D, "STA", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0x9D, "STA", AddressingMode::AbsoluteX, 5, 3);
    insert_instruction!(table, 0x99, "STA", AddressingMode::AbsoluteY, 5, 3);
    insert_instruction!(table, 0x81, "STA", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0x91, "STA", AddressingMode::IndirectY, 6, 2);
    insert_instruction!(table, 0x95, "STA", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0x86, "STX", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0x96, "STX", AddressingMode::ZeroPageY, 4, 2);
    insert_instruction!(table, 0x8E, "STX", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0x84, "STY", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0x94, "STY", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0x8C, "STY", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0xAA, "TAX", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0xA8, "TAY", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0xBA, "TSX", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x8A, "TXA", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x98, "TYA", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x9A, "TXS", AddressingMode::Implied, 2, 1);

    // Stack operations
    insert_instruction!(table, 0x48, "PHA", AddressingMode::Implied, 3, 1);
    insert_instruction!(table, 0x08, "PHP", AddressingMode::Implied, 3, 1);
    insert_instruction!(table, 0x68, "PLA", AddressingMode::Implied, 4, 1);
    insert_instruction!(table, 0x28, "PLP", AddressingMode::Implied, 4, 1);

    // Increment/Decrement
    insert_instruction!(table, 0xC6, "DEC", AddressingMode::ZeroPage, 5, 2);
    insert_instruction!(table, 0xD6, "DEC", AddressingMode::ZeroPageX, 6, 2);
    insert_instruction!(table, 0xCE, "DEC", AddressingMode::Absolute, 6, 3);
    insert_instruction!(table, 0xDE, "DEC", AddressingMode::AbsoluteX, 7, 3);
    insert_instruction!(table, 0xCA, "DEX", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x88, "DEY", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0xE6, "INC", AddressingMode::ZeroPage, 5, 2);
    insert_instruction!(table, 0xF6, "INC", AddressingMode::ZeroPageX, 6, 2);
    insert_instruction!(table, 0xEE, "INC", AddressingMode::Absolute, 6, 3);
    insert_instruction!(table, 0xFE, "INC", AddressingMode::AbsoluteX, 7, 3);
    insert_instruction!(table, 0xE8, "INX", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0xC8, "INY", AddressingMode::Implied, 2, 1);

    // Arithmetic
    insert_instruction!(table, 0x69, "ADC", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0x65, "ADC", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0x75, "ADC", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0x6D, "ADC", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0x7D, "ADC", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0x79, "ADC", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0x61, "ADC", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0x71, "ADC", AddressingMode::IndirectY, 5, 2);
    insert_instruction!(table, 0xE9, "SBC", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0xE5, "SBC", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0xF5, "SBC", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0xED, "SBC", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0xFD, "SBC", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0xF9, "SBC", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0xE1, "SBC", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0xF1, "SBC", AddressingMode::IndirectY, 5, 2);

    // Logical operations
    insert_instruction!(table, 0x29, "AND", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0x25, "AND", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0x35, "AND", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0x2D, "AND", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0x3D, "AND", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0x39, "AND", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0x21, "AND", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0x31, "AND", AddressingMode::IndirectY, 5, 2);
    insert_instruction!(table, 0x49, "EOR", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0x45, "EOR", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0x55, "EOR", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0x4D, "EOR", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0x5D, "EOR", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0x59, "EOR", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0x41, "EOR", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0x51, "EOR", AddressingMode::IndirectY, 5, 2);
    insert_instruction!(table, 0x09, "ORA", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0x05, "ORA", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0x15, "ORA", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0x0D, "ORA", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0x1D, "ORA", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0x19, "ORA", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0x01, "ORA", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0x11, "ORA", AddressingMode::IndirectY, 5, 2);

    // Bit manipulation and shift operations
    insert_instruction!(table, 0x0A, "ASL", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x06, "ASL", AddressingMode::ZeroPage, 5, 2);
    insert_instruction!(table, 0x16, "ASL", AddressingMode::ZeroPageX, 6, 2);
    insert_instruction!(table, 0x0E, "ASL", AddressingMode::Absolute, 6, 3);
    insert_instruction!(table, 0x1E, "ASL", AddressingMode::AbsoluteX, 7, 3);
    insert_instruction!(table, 0x4A, "LSR", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x46, "LSR", AddressingMode::ZeroPage, 5, 2);
    insert_instruction!(table, 0x56, "LSR", AddressingMode::ZeroPageX, 6, 2);
    insert_instruction!(table, 0x4E, "LSR", AddressingMode::Absolute, 6, 3);
    insert_instruction!(table, 0x5E, "LSR", AddressingMode::AbsoluteX, 7, 3);
    insert_instruction!(table, 0x2A, "ROL", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x26, "ROL", AddressingMode::ZeroPage, 5, 2);
    insert_instruction!(table, 0x36, "ROL", AddressingMode::ZeroPageX, 6, 2);
    insert_instruction!(table, 0x2E, "ROL", AddressingMode::Absolute, 6, 3);
    insert_instruction!(table, 0x3E, "ROL", AddressingMode::AbsoluteX, 7, 3);
    insert_instruction!(table, 0x6A, "ROR", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x66, "ROR", AddressingMode::ZeroPage, 5, 2);
    insert_instruction!(table, 0x76, "ROR", AddressingMode::ZeroPageX, 6, 2);
    insert_instruction!(table, 0x6E, "ROR", AddressingMode::Absolute, 6, 3);
    insert_instruction!(table, 0x7E, "ROR", AddressingMode::AbsoluteX, 7, 3);

    // Flag operations
    insert_instruction!(table, 0x18, "CLC", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0xD8, "CLD", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x58, "CLI", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0xB8, "CLV", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x38, "SEC", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0xF8, "SED", AddressingMode::Implied, 2, 1);
    insert_instruction!(table, 0x78, "SEI", AddressingMode::Implied, 2, 1);

    // Compare operations
    insert_instruction!(table, 0xC9, "CMP", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0xC5, "CMP", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0xD5, "CMP", AddressingMode::ZeroPageX, 4, 2);
    insert_instruction!(table, 0xCD, "CMP", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0xDD, "CMP", AddressingMode::AbsoluteX, 4, 3);
    insert_instruction!(table, 0xD9, "CMP", AddressingMode::AbsoluteY, 4, 3);
    insert_instruction!(table, 0xC1, "CMP", AddressingMode::IndirectX, 6, 2);
    insert_instruction!(table, 0xD1, "CMP", AddressingMode::IndirectY, 5, 2);
    insert_instruction!(table, 0xE0, "CPX", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0xE4, "CPX", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0xEC, "CPX", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0xC0, "CPY", AddressingMode::Immediate, 2, 2);
    insert_instruction!(table, 0xC4, "CPY", AddressingMode::ZeroPage, 3, 2);
    insert_instruction!(table, 0xCC, "CPY", AddressingMode::Absolute, 4, 3);

    // Branch instructions
    insert_instruction!(table, 0x90, "BCC", AddressingMode::Relative, 2, 2);
    insert_instruction!(table, 0xB0, "BCS", AddressingMode::Relative, 2, 2);
    insert_instruction!(table, 0xF0, "BEQ", AddressingMode::Relative, 2, 2);
    insert_instruction!(table, 0x30, "BMI", AddressingMode::Relative, 2, 2);
    insert_instruction!(table, 0xD0, "BNE", AddressingMode::Relative, 2, 2);
    insert_instruction!(table, 0x10, "BPL", AddressingMode::Relative, 2, 2);
    insert_instruction!(table, 0x50, "BVC", AddressingMode::Relative, 2, 2);
    insert_instruction!(table, 0x70, "BVS", AddressingMode::Relative, 2, 2);

    // Jump and subroutine instructions
    insert_instruction!(table, 0x4C, "JMP", AddressingMode::Absolute, 3, 3);
    insert_instruction!(table, 0x6C, "JMP", AddressingMode::Indirect, 5, 3);
    insert_instruction!(table, 0x20, "JSR", AddressingMode::Absolute, 6, 3);
    insert_instruction!(table, 0x60, "RTS", AddressingMode::Implied, 6, 1);

    // Interrupt and system instructions
    insert_instruction!(table, 0x00, "BRK", AddressingMode::Implied, 7, 1);
    insert_instruction!(table, 0x40, "RTI", AddressingMode::Implied, 6, 1);

    // Bit test
    insert_instruction!(table, 0x2C, "BIT", AddressingMode::Absolute, 4, 3);
    insert_instruction!(table, 0x24, "BIT", AddressingMode::ZeroPage, 3, 2);

    // No operation
    insert_instruction!(table, 0xEA, "NOP", AddressingMode::Implied, 2, 1);

    table
};

/// Get instruction metadata for a given opcode.
/// Returns None for invalid/unimplemented opcodes.
#[inline]
pub fn get_instruction(opcode: u8) -> Option<&'static Instruction> {
    INSTRUCTION_TABLE[opcode as usize].as_ref()
}

/// Check if an opcode is valid (has an implementation).
#[inline]
pub fn is_valid_opcode(opcode: u8) -> bool {
    INSTRUCTION_TABLE[opcode as usize].is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_lookup() {
        // Test valid opcodes
        assert!(is_valid_opcode(0xA9)); // LDA immediate
        assert!(is_valid_opcode(0xEA)); // NOP
        assert!(is_valid_opcode(0x00)); // BRK

        // Test invalid opcodes
        assert!(!is_valid_opcode(0x02)); // Invalid opcode
        assert!(!is_valid_opcode(0xFF)); // Invalid opcode

        // Test instruction retrieval
        let lda_imm = get_instruction(0xA9).unwrap();
        assert_eq!(lda_imm.mnemonic, "LDA");
        assert_eq!(lda_imm.opcode, 0xA9);
        assert_eq!(lda_imm.cycles, 2);
        assert_eq!(lda_imm.bytes, 2);

        let nop = get_instruction(0xEA).unwrap();
        assert_eq!(nop.mnemonic, "NOP");
        assert_eq!(nop.opcode, 0xEA);
        assert_eq!(nop.cycles, 2);
        assert_eq!(nop.bytes, 1);
    }

    #[test]
    fn test_instruction_table_completeness() {
        // Count valid instructions
        let valid_count = INSTRUCTION_TABLE.iter().filter(|i| i.is_some()).count();
        println!("Valid instructions: {}", valid_count);

        // Should have exactly the instructions we defined
        assert!(valid_count > 100); // We have about 135 instructions

        // Test a few key instructions from each category
        let key_opcodes = [
            0xA9, // LDA immediate
            0x85, // STA zero page
            0x4C, // JMP absolute
            0x20, // JSR
            0x60, // RTS
            0x00, // BRK
            0xEA, // NOP
            0x69, // ADC immediate
            0xE9, // SBC immediate
            0x29, // AND immediate
            0x09, // ORA immediate
            0x49, // EOR immediate
            0x0A, // ASL accumulator
            0x4A, // LSR accumulator
            0x2A, // ROL accumulator
            0x6A, // ROR accumulator
            0x18, // CLC
            0x38, // SEC
            0xC9, // CMP immediate
            0xE0, // CPX immediate
            0xC0, // CPY immediate
            0x90, // BCC
            0xF0, // BEQ
            0xD0, // BNE
            0x10, // BPL
            0xE8, // INX
            0xC8, // INY
            0xCA, // DEX
            0x88, // DEY
            0xAA, // TAX
            0xA8, // TAY
            0x8A, // TXA
            0x98, // TYA
            0x48, // PHA
            0x68, // PLA
        ];

        for &opcode in &key_opcodes {
            assert!(
                is_valid_opcode(opcode),
                "Opcode 0x{:02X} should be valid",
                opcode
            );
        }
    }
}
