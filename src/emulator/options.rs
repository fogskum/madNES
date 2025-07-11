use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct EmulatorOptions {
    #[arg(short, long, default_value = "", help = "Path to the .nes ROM file")]
    pub rom: String,
    
    #[arg(short, long, help = "Enable debug mode with nestest.nes")]
    pub debug: bool,
}