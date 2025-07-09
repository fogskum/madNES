use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct EmulatorOptions {
    #[arg(short, long, default_value = "", help = "Path to the .nes ROM file")]
    pub rom: String,
    
    #[arg(short, long, default_value_t = 800)]
    pub width: u32,
    
    #[arg(short = 'H', long, default_value_t = 600)]
    pub height: u32,
    
    #[arg(short, long, help = "Enable debug mode with nestest.nes")]
    pub debug: bool,
}