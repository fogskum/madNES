use clap::Parser;
use madnes::emulator::core::Emulator;
use madnes::emulator::options::EmulatorOptions;

fn main() {
    display_banner();
    
    let mut args = EmulatorOptions::parse();
    
    // Handle debug mode
    if args.debug {
        args.rom = "assets/nestest.nes".to_string();
        println!("Debug mode enabled - loading nestest.nes");
    }
    
    // Check if ROM path is provided
    if args.rom.is_empty() {
        eprintln!("Error: No ROM file specified. Use --rom-path <path> to specify a .nes file, or use --debug for nestest.nes.");
        std::process::exit(1);
    }
    
    println!("Initializing emulator...");
    let mut emulator = match Emulator::new(args) {
        Ok(emulator) => emulator,
        Err(e) => {
            eprintln!("Failed to initialize emulator: {}", e);
            std::process::exit(1);
        }
    };
    
    if let Err(e) = emulator.run() {
        eprintln!("Failed to run emulator: {}", e);
        std::process::exit(1);
    }
}

fn display_banner() {
    println!("\n");
    println!("                                    _/  _/      _/  _/_/_/_/    _/_/_/   ");
    println!("   _/_/_/  _/_/      _/_/_/    _/_/_/  _/_/    _/  _/        _/          ");
    println!("  _/    _/    _/  _/    _/  _/    _/  _/  _/  _/  _/_/_/      _/_/       ");
    println!(" _/    _/    _/  _/    _/  _/    _/  _/    _/_/  _/              _/      ");
    println!("_/    _/    _/    _/_/_/    _/_/_/  _/      _/  _/_/_/_/  _/_/_/         ");
    println!("\n");
}