/// Utility functions for bit manipulation
pub fn get_bit(value: u8, bit: u8) -> bool {
    (value & (1 << bit)) != 0
}

pub fn set_bit(value: u8, bit: u8) -> u8 {
    value | (1 << bit)
}

pub fn clear_bit(value: u8, bit: u8) -> u8 {
    value & !(1 << bit)
}

pub fn toggle_bit(value: u8, bit: u8) -> u8 {
    value ^ (1 << bit)
}

pub fn get_bits(value: u8, start: u8, count: u8) -> u8 {
    let mask = (1 << count) - 1;
    (value >> start) & mask
}

pub fn set_bits(value: u8, start: u8, count: u8, new_value: u8) -> u8 {
    let mask = (1 << count) - 1;
    let cleared = value & !(mask << start);
    cleared | ((new_value & mask) << start)
}

/// Convert two u8 values to u16 (little endian)
pub fn make_word(low: u8, high: u8) -> u16 {
    (high as u16) << 8 | (low as u16)
}

/// Extract low byte from u16
pub fn low_byte(value: u16) -> u8 {
    (value & 0xFF) as u8
}

/// Extract high byte from u16
pub fn high_byte(value: u16) -> u8 {
    (value >> 8) as u8
}

/// Check if adding two u8 values would overflow
pub fn would_overflow_add(a: u8, b: u8) -> bool {
    a.checked_add(b).is_none()
}

/// Check if subtracting two u8 values would underflow
pub fn would_underflow_sub(a: u8, b: u8) -> bool {
    a < b
}

/// Page cross detection for 6502 addressing modes
pub fn page_crossed(addr1: u16, addr2: u16) -> bool {
    (addr1 & 0xFF00) != (addr2 & 0xFF00)
}

/// Wrap around page boundary (for zero page addressing)
pub fn wrap_page(addr: u16) -> u16 {
    (addr & 0xFF00) | ((addr + 1) & 0xFF)
}
