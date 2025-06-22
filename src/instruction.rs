use crate::cpu::AddressingMode;

pub struct Instruction 
{
    // fixed set of strings for the instructions that are stored in the binary
    pub mnemonic: &'static str,
    pub opcode: u8,
    pub addressing_mode: AddressingMode,
    pub cycles: u8,
    pub bytes: u8,
}

impl Instruction 
{
    pub fn new(mnemonic: &'static str, opcode: u8, addressing_mode: AddressingMode, cycles: u8, bytes: u8) -> Self 
    {
        Instruction 
        {
            mnemonic,
            opcode,
            addressing_mode,
            cycles,
            bytes,
        }
    }
}