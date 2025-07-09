use std::fmt;

/// Comprehensive error types for the madNES emulator
#[derive(Debug, Clone)]
pub enum EmulatorError {
    /// CPU-related errors
    Cpu(CpuError),
    /// Memory-related errors
    Memory(MemoryError),
    /// ROM loading and parsing errors
    Rom(RomError),
    /// Input/Output errors
    Io(IoError),
    /// SDL2-related errors
    Sdl(SdlError),
    /// Configuration errors
    Config(ConfigError),
}

#[derive(Debug, Clone)]
pub enum CpuError {
    /// Unknown or unimplemented opcode
    UnknownOpcode { opcode: u8, pc: u16 },
    /// Invalid program counter
    InvalidProgramCounter(u16),
    /// Stack overflow
    StackOverflow,
    /// Stack underflow
    StackUnderflow,
    /// Invalid instruction at address
    InvalidInstruction { address: u16, reason: String },
    /// Program execution halted
    ExecutionHalted(String),
}

#[derive(Debug, Clone)]
pub enum MemoryError {
    /// Memory address out of bounds
    OutOfBounds { address: u16, size: usize },
    /// Invalid memory region access
    InvalidRegion(u16),
    /// ROM not loaded
    RomNotLoaded,
    /// Memory initialization failed
    InitializationFailed(String),
}

#[derive(Debug, Clone)]
pub enum RomError {
    /// Invalid NES ROM header
    InvalidHeader(String),
    /// File too small to be a valid ROM
    FileTooSmall { expected: usize, actual: usize },
    /// Unsupported mapper
    UnsupportedMapper(u8),
    /// ROM data corrupted or invalid
    CorruptedData(String),
    /// Missing required ROM data
    MissingData(String),
    /// File I/O error
    Io(String),
}

#[derive(Debug, Clone)]
pub enum IoError {
    /// File not found
    FileNotFound(String),
    /// Permission denied
    PermissionDenied(String),
    /// Read error
    ReadError(String),
    /// Write error
    WriteError(String),
    /// General I/O error
    Other(String),
}

#[derive(Debug, Clone)]
pub enum SdlError {
    /// SDL2 initialization failed
    InitializationFailed(String),
    /// Window creation failed
    WindowCreationFailed(String),
    /// Renderer creation failed
    RendererCreationFailed(String),
    /// Texture creation failed
    TextureCreationFailed(String),
    /// Font loading failed
    FontLoadingFailed(String),
    /// Audio initialization failed
    AudioInitializationFailed(String),
    /// Other SDL error
    Other(String),
}

#[derive(Debug, Clone)]
pub enum ConfigError {
    /// Invalid ROM path
    InvalidRomPath(String),
    /// Invalid configuration value
    InvalidValue { key: String, value: String },
    /// Missing required configuration
    MissingRequired(String),
}

// Implement Display for all error types
impl fmt::Display for EmulatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmulatorError::Cpu(e) => write!(f, "CPU Error: {}", e),
            EmulatorError::Memory(e) => write!(f, "Memory Error: {}", e),
            EmulatorError::Rom(e) => write!(f, "ROM Error: {}", e),
            EmulatorError::Io(e) => write!(f, "I/O Error: {}", e),
            EmulatorError::Sdl(e) => write!(f, "SDL Error: {}", e),
            EmulatorError::Config(e) => write!(f, "Configuration Error: {}", e),
        }
    }
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CpuError::UnknownOpcode { opcode, pc } => {
                write!(f, "Unknown opcode 0x{:02X} at PC 0x{:04X}", opcode, pc)
            }
            CpuError::InvalidProgramCounter(pc) => {
                write!(f, "Invalid program counter: 0x{:04X}", pc)
            }
            CpuError::StackOverflow => write!(f, "Stack overflow"),
            CpuError::StackUnderflow => write!(f, "Stack underflow"),
            CpuError::InvalidInstruction { address, reason } => {
                write!(f, "Invalid instruction at 0x{:04X}: {}", address, reason)
            }
            CpuError::ExecutionHalted(reason) => {
                write!(f, "CPU execution halted: {}", reason)
            }
        }
    }
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::OutOfBounds { address, size } => {
                write!(f, "Memory address 0x{:04X} out of bounds (size: {})", address, size)
            }
            MemoryError::InvalidRegion(address) => {
                write!(f, "Invalid memory region access at 0x{:04X}", address)
            }
            MemoryError::RomNotLoaded => write!(f, "ROM not loaded"),
            MemoryError::InitializationFailed(reason) => {
                write!(f, "Memory initialization failed: {}", reason)
            }
        }
    }
}

impl fmt::Display for RomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RomError::InvalidHeader(reason) => {
                write!(f, "Invalid ROM header: {}", reason)
            }
            RomError::FileTooSmall { expected, actual } => {
                write!(f, "ROM file too small: expected at least {} bytes, got {}", expected, actual)
            }
            RomError::UnsupportedMapper(mapper) => {
                write!(f, "Unsupported mapper: {}", mapper)
            }
            RomError::CorruptedData(reason) => {
                write!(f, "ROM data corrupted: {}", reason)
            }
            RomError::MissingData(data) => {
                write!(f, "Missing ROM data: {}", data)
            }
            RomError::Io(reason) => {
                write!(f, "ROM I/O error: {}", reason)
            }
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IoError::FileNotFound(path) => write!(f, "File not found: {}", path),
            IoError::PermissionDenied(path) => write!(f, "Permission denied: {}", path),
            IoError::ReadError(reason) => write!(f, "Read error: {}", reason),
            IoError::WriteError(reason) => write!(f, "Write error: {}", reason),
            IoError::Other(reason) => write!(f, "I/O error: {}", reason),
        }
    }
}

impl fmt::Display for SdlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdlError::InitializationFailed(reason) => {
                write!(f, "SDL initialization failed: {}", reason)
            }
            SdlError::WindowCreationFailed(reason) => {
                write!(f, "Window creation failed: {}", reason)
            }
            SdlError::RendererCreationFailed(reason) => {
                write!(f, "Renderer creation failed: {}", reason)
            }
            SdlError::TextureCreationFailed(reason) => {
                write!(f, "Texture creation failed: {}", reason)
            }
            SdlError::FontLoadingFailed(reason) => {
                write!(f, "Font loading failed: {}", reason)
            }
            SdlError::AudioInitializationFailed(reason) => {
                write!(f, "Audio initialization failed: {}", reason)
            }
            SdlError::Other(reason) => {
                write!(f, "SDL error: {}", reason)
            }
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::InvalidRomPath(path) => {
                write!(f, "Invalid ROM path: {}", path)
            }
            ConfigError::InvalidValue { key, value } => {
                write!(f, "Invalid configuration value for '{}': {}", key, value)
            }
            ConfigError::MissingRequired(key) => {
                write!(f, "Missing required configuration: {}", key)
            }
        }
    }
}

// Implement std::error::Error for all error types
impl std::error::Error for EmulatorError {}
impl std::error::Error for CpuError {}
impl std::error::Error for MemoryError {}
impl std::error::Error for RomError {}
impl std::error::Error for IoError {}
impl std::error::Error for SdlError {}
impl std::error::Error for ConfigError {}

// Convenience conversions
impl From<CpuError> for EmulatorError {
    fn from(err: CpuError) -> Self {
        EmulatorError::Cpu(err)
    }
}

impl From<MemoryError> for EmulatorError {
    fn from(err: MemoryError) -> Self {
        EmulatorError::Memory(err)
    }
}

impl From<RomError> for EmulatorError {
    fn from(err: RomError) -> Self {
        EmulatorError::Rom(err)
    }
}

impl From<IoError> for EmulatorError {
    fn from(err: IoError) -> Self {
        EmulatorError::Io(err)
    }
}

impl From<SdlError> for EmulatorError {
    fn from(err: SdlError) -> Self {
        EmulatorError::Sdl(err)
    }
}

impl From<ConfigError> for EmulatorError {
    fn from(err: ConfigError) -> Self {
        EmulatorError::Config(err)
    }
}

// Convert from std::io::Error
impl From<std::io::Error> for IoError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => IoError::FileNotFound(err.to_string()),
            std::io::ErrorKind::PermissionDenied => IoError::PermissionDenied(err.to_string()),
            _ => IoError::Other(err.to_string()),
        }
    }
}

impl From<std::io::Error> for EmulatorError {
    fn from(err: std::io::Error) -> Self {
        EmulatorError::Io(IoError::from(err))
    }
}

// Type aliases for convenience
pub type EmulatorResult<T> = Result<T, EmulatorError>;
pub type CpuResult<T> = Result<T, CpuError>;
pub type MemoryResult<T> = Result<T, MemoryError>;
pub type RomResult<T> = Result<T, RomError>;
