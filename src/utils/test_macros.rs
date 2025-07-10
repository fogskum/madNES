/// Macros for common test setup patterns
#[cfg(test)]
pub mod test_utils {
    /// Macro to create a test ROM with specified data
    #[allow(unused_macros)]
    macro_rules! create_test_rom {
        ($prg_data:expr, $chr_data:expr) => {
            crate::rom::Rom {
                prg_rom: $prg_data,
                chr_rom: $chr_data,
                mirror_mode: crate::rom::MirrorMode::Horizontal,
                mapper: 0,
                prg_ram_size: 8192,
            }
        };
        ($prg_data:expr) => {
            create_test_rom!($prg_data, vec![0; 8192])
        };
    }
    
    /// Macro to create and initialize a test CPU
    #[allow(unused_macros)]
    macro_rules! create_test_cpu {
        () => {{
            let mut cpu = crate::cpu::Cpu::new();
            cpu
        }};
        ($program:expr) => {{
            let mut cpu = crate::cpu::Cpu::new();
            cpu.load_program($program, 0x8000).unwrap();
            cpu.pc = 0x8000;
            cpu
        }};
        ($program:expr, $address:expr) => {{
            let mut cpu = crate::cpu::Cpu::new();
            cpu.load_program($program, $address).unwrap();
            cpu.pc = $address;
            cpu
        }};
    }
    
    /// Macro to assert CPU flags
    #[allow(unused_macros)]
    macro_rules! assert_cpu_flags {
        ($cpu:expr, $(($flag:expr, $expected:expr)),*) => {
            $(
                assert_eq!($cpu.get_flag($flag), $expected, 
                    "Flag {:?} expected {}, got {}", $flag, $expected, $cpu.get_flag($flag));
            )*
        };
    }
    
    /// Macro for common test assertions
    #[allow(unused_macros)]
    macro_rules! assert_memory_eq {
        ($cpu:expr, $address:expr, $expected:expr) => {
            {
                use crate::cpu::Memory;
                assert_eq!($cpu.read_byte($address), $expected, 
                    "Memory at 0x{:04X} expected 0x{:02X}, got 0x{:02X}", 
                    $address, $expected, $cpu.read_byte($address));
            }
        };
    }
    
    #[allow(unused_imports)]
    pub(crate) use create_test_rom;
    #[allow(unused_imports)]
    pub(crate) use create_test_cpu;
    #[allow(unused_imports)]
    pub(crate) use assert_cpu_flags;
    #[allow(unused_imports)]
    pub(crate) use assert_memory_eq;
}
