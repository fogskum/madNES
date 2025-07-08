#!/usr/bin/env python3
"""
Create a simple test NES ROM file for testing madNES emulator.
This creates a minimal NROM ROM with a basic program.
"""

import struct

def create_test_rom():
    # Create a 16KB PRG ROM
    prg_rom = bytearray(16 * 1024)
    
    # Simple test program: Load 0x42 into A register and loop
    program = [
        0xA9, 0x42,  # LDA #$42
        0x4C, 0x00, 0x80  # JMP $8000 (infinite loop)
    ]
    
    # Place program at start of ROM
    for i, byte in enumerate(program):
        prg_rom[i] = byte
    
    # Set reset vector to point to start of ROM (0x8000)
    # Reset vector is at 0xFFFC-0xFFFD, which is offset 0x3FFC-0x3FFD in a 16KB ROM
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
    with open('test.nes', 'wb') as f:
        f.write(header)
        f.write(prg_rom)
        f.write(chr_rom)
    
    print("Created test.nes ROM file")
    print(f"Header: {header.hex()}")
    print(f"PRG ROM size: {len(prg_rom)} bytes")
    print(f"CHR ROM size: {len(chr_rom)} bytes")
    print(f"Reset vector: 0x{prg_rom[0x3FFC]:02X}{prg_rom[0x3FFD]:02X}")

if __name__ == "__main__":
    create_test_rom()
