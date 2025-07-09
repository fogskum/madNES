use clap::Parser;
use madnes::emulator::emulator::Emulator;
use madnes::emulator::options::EmulatorOptions;

fn main() {
    let mut args = EmulatorOptions::parse();

    // Handle debug mode
    if args.debug {
        args.rom_path = "assets/nestest.nes".to_string();
        println!("Debug mode enabled - loading nestest.nes");
    }

    // Check if ROM path is provided
    if args.rom_path.is_empty() {
        eprintln!("Error: No ROM file specified. Use --rom-path <path> to specify a .nes file, or use --debug for nestest.nes.");
        std::process::exit(1);
    }

    let mut emulator = Emulator::new(args).expect("Failed to initialize emulator");
    emulator.run().expect("Failed to run emulator");
}
