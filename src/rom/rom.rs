pub enum MirrorMode {
    Horizontal,
    Vertical,
    FourScreen,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mirror_mode: MirrorMode,
    pub mapper: u8,
}

impl Rom {
    pub fn new(rom: &Vec<u8>) -> Result<Rom, String> {
        if &rom[0..4] != b"NES\x1A" {
            return Err("Invalid NES ROM header".to_string());
        }

        todo!();
    }
}