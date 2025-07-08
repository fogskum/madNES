#!/usr/bin/env python3
"""
Create a simple test NES ROM file for testing logging functionality.
"""

import struct

def create_test_rom():
    # Create a 16KB PRG ROM
    prg_rom = bytearray(16 * 1024)
    
    # Simple test program with various instructions
    program = [
        0xA9, 0x42,  # LDA #$42
        0x8D, 0x00, 0x30,  # STA $3000
        0xA2, 0x10,  # LDX #$10
        0xA0, 0x20,  # LDY #$20
        0x18,        # CLC
        0x69, 0x01,  # ADC #$01
        0xEA,        # NOP
        0x4C, 0x00, 0x80  # JMP $8000 (loop back)
    ]
    
    # Place program at start of ROM
    for i, byte in enumerate(program):
        prg_rom[i] = byte
    
    # Set reset vector to point to start of ROM (0x8000)
    prg_rom[0x3FFC] = 0x00  # Low byte
    prg_rom[0x3FFD] = 0x80  # High byte
    
    # Create 8KB CHR ROM (empty)
    chr_rom = bytearray(8 * 1024)
    
    # Create NES header
    header = bytearray(16)
    header[0:4] = b'NES\x1A'  # NES signature
    header[4] = 1  # 1 x 16KB PRG ROM
    header[5] = 1  # 1 x 8KB CHR ROM
    header[6] = 0  # Mapper 0 (NROM), horizontal mirroring
    header[7] = 0  # Mapper 0 (NROM)
    
    # Write ROM file
    with open('test_log.nes', 'wb') as f:
        f.write(header)
        f.write(prg_rom)
        f.write(chr_rom)
    
    print("Created test_log.nes ROM file")
    print(f"Program: {[hex(b) for b in program]}")

if __name__ == "__main__":
    create_test_rom()
