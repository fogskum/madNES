pub mod mapper;
pub mod mapper000;
pub mod mapper001;

pub use mapper::Mapper;
pub use mapper000::Mapper000;
pub use mapper001::Mapper001;

/// Create a mapper instance based on the mapper number
pub fn create_mapper(mapper_number: u8, prg_banks: u8, chr_banks: u8) -> Box<dyn Mapper> {
    match mapper_number {
        0 => Box::new(Mapper000::new(prg_banks, chr_banks)),
        1 => Box::new(Mapper001::new(prg_banks, chr_banks)),
        _ => {
            eprintln!("Warning: Unsupported mapper {}, using NROM (0)", mapper_number);
            Box::new(Mapper000::new(prg_banks, chr_banks))
        }
    }
}
