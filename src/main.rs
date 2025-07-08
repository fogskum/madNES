use clap::Parser;
use madnes::emulator::emulator::Emulator;
use madnes::emulator::options::EmulatorOptions;

fn main() {
    let args = EmulatorOptions::parse();

    // Check if ROM path is provided
    if args.rom_path.is_empty() {
        eprintln!("Error: No ROM file specified. Use --rom-path <path> to specify a .nes file.");
        std::process::exit(1);
    }

    let mut emulator = Emulator::new(args).expect("Failed to initialize emulator");
    emulator.run().expect("Failed to run emulator");
}
