use crate::error::MemoryResult;
use crate::rom::MirrorMode;

/// Trait for memory mappers
pub trait Mapper {
    /// Map CPU address to PRG ROM/RAM
    fn map_prg_read(&self, address: u16) -> MemoryResult<Option<u32>>;

    /// Map CPU address to PRG ROM/RAM for writing
    fn map_prg_write(&mut self, address: u16, value: u8) -> MemoryResult<Option<u32>>;

    /// Map PPU address to CHR ROM/RAM
    fn map_chr_read(&self, address: u16) -> MemoryResult<Option<u32>>;

    /// Map PPU address to CHR ROM/RAM for writing
    fn map_chr_write(&mut self, address: u16, value: u8) -> MemoryResult<Option<u32>>;

    /// Get current mirroring mode
    fn get_mirror_mode(&self) -> MirrorMode;

    /// Reset the mapper
    fn reset(&mut self);

    /// Check if mapper generates interrupts
    fn irq_state(&self) -> bool;

    /// Clear IRQ flag
    fn irq_clear(&mut self);

    /// Scanline counter for mappers that need it
    fn scanline(&mut self);
}
