/// Macros to reduce boilerplate in mapper implementations
/// Macro to generate common mapper methods that don't need interrupts
macro_rules! impl_no_irq_mapper {
    ($mapper_type:ty) => {
        impl $mapper_type {
            /// Check if mapper generates interrupts
            pub fn irq_state(&self) -> bool {
                false
            }
            
            /// Clear IRQ flag
            pub fn irq_clear(&mut self) {
                // This mapper doesn't generate IRQs
            }
            
            /// Scanline callback
            pub fn scanline(&mut self) {
                // This mapper doesn't use scanline counter
            }
        }
    };
}

/// Macro to generate CHR ROM/RAM access patterns
macro_rules! impl_chr_access {
    ($mapper_type:ty, $chr_banks_field:ident) => {
        impl $mapper_type {
            /// Common CHR write logic for ROM vs RAM
            pub fn chr_write_common(&mut self, _address: u16, mapped_addr: u32) -> crate::error::MemoryResult<Option<u32>> {
                if self.$chr_banks_field == 0 {
                    // CHR RAM - allow writes
                    Ok(Some(mapped_addr))
                } else {
                    // CHR ROM - read-only
                    Ok(None)
                }
            }
        }
    };
}

/// Macro to generate PRG ROM mirroring for 16KB ROMs
macro_rules! impl_prg_mirror {
    () => {
        /// Mirror 16KB PRG ROM to fill 32KB space
        pub fn mirror_prg_16k(address: u16) -> u32 {
            let addr = address - 0x8000;
            (addr & 0x3FFF) as u32
        }
    };
}

pub(crate) use impl_no_irq_mapper;
pub(crate) use impl_chr_access;
pub(crate) use impl_prg_mirror;
