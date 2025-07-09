/// Helper functions for common error handling patterns
use crate::error::{EmulatorError, MemoryError, MemoryResult};

/// Helper function to validate memory address bounds
pub fn validate_address_bounds(address: u16, max_size: usize) -> MemoryResult<()> {
    if address as usize >= max_size {
        Err(MemoryError::OutOfBounds {
            address,
            size: max_size,
        })
    } else {
        Ok(())
    }
}

/// Helper function to validate address ranges
pub fn validate_address_range(address: u16, start: u16, end: u16) -> MemoryResult<()> {
    if address < start || address > end {
        Err(MemoryError::InvalidRegion(address))
    } else {
        Ok(())
    }
}

/// Helper function for mirroring addresses
pub fn mirror_address(address: u16, mask: u16) -> u16 {
    address & mask
}

/// Helper function for bank switching calculations
pub fn calculate_bank_address(address: u16, base: u16, bank: u8, bank_size: u16) -> u32 {
    ((address - base) + (bank as u16 * bank_size)) as u32
}

/// Helper function for ROM mirroring (common in 16KB ROMs)
pub fn mirror_rom_16k(address: u16, rom_size: usize) -> u32 {
    let offset = address - 0x8000;
    if rom_size <= 0x4000 {
        // 16KB ROM - mirror it
        (offset & 0x3FFF) as u32
    } else {
        // 32KB or larger ROM
        offset as u32
    }
}

/// Helper to convert SDL errors to EmulatorError
pub fn sdl_error_to_emulator_error(error: String, context: &str) -> EmulatorError {
    use crate::error::SdlError;
    match context {
        "init" => EmulatorError::Sdl(SdlError::InitializationFailed(error)),
        "window" => EmulatorError::Sdl(SdlError::WindowCreationFailed(error)),
        "renderer" => EmulatorError::Sdl(SdlError::RendererCreationFailed(error)),
        "audio" => EmulatorError::Sdl(SdlError::AudioInitializationFailed(error)),
        "texture" => EmulatorError::Sdl(SdlError::TextureCreationFailed(error)),
        _ => EmulatorError::Sdl(SdlError::Other(error)),
    }
}
