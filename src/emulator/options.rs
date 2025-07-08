use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct EmulatorOptions {
    #[arg(short, long)]
    pub rom_path: String,
    
    #[arg(short, long, default_value_t = 800)]
    pub width: u32,
    
    #[arg(short = 'H', long, default_value_t = 600)]
    pub height: u32
}