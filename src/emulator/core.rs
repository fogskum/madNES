use crate::cpu::Cpu;
use crate::emulator::options::EmulatorOptions;
use crate::error::{EmulatorError, IoError, SdlError};
use crate::rom::Rom;
use crate::utils::error_helpers::sdl_error_to_emulator_error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump, VideoSubsystem};
use std::time::{Duration, Instant};

const NES_WIDTH: u32 = 256;
const NES_HEIGHT: u32 = 240;

pub struct Emulator {
    #[allow(dead_code)]
    options: EmulatorOptions,
    cpu: Cpu,
    main_canvas: Canvas<Window>,
    debug_canvas: Canvas<Window>,
    event_pump: EventPump,
    disassembly_lines: Vec<String>,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
    // Store the font path for lazy loading
    font_path: String,
    font_size: u16,
}

impl Emulator {
    pub fn new(options: EmulatorOptions) -> Result<Self, EmulatorError> {
        let mut cpu = Cpu::new();

        let sdl_context =
            sdl2::init().map_err(|e| sdl_error_to_emulator_error(e.to_string(), "init"))?;
        let video_subsystem = sdl_context
            .video()
            .map_err(|e| sdl_error_to_emulator_error(e.to_string(), "init"))?;

        // Initialize TTF
        let ttf_context =
            sdl2::ttf::init().map_err(|e| sdl_error_to_emulator_error(e.to_string(), "init"))?;

        // Create main emulator window
        let main_canvas = Emulator::create_main_canvas(&video_subsystem)?;

        // Create debug window for disassembly
        let mut debug_canvas = Emulator::create_debug_canvas(&video_subsystem)?;

        // Create texture creator for text rendering
        let texture_creator = debug_canvas.texture_creator();

        debug_canvas.set_draw_color(Color::RGB(0, 0, 100));
        debug_canvas.clear();
        debug_canvas.present();

        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| sdl_error_to_emulator_error(e.to_string(), "init"))?;

        // Load ROM if provided
        if !options.rom.is_empty() {
            let rom_data = std::fs::read(&options.rom).map_err(|e| {
                EmulatorError::Io(IoError::ReadError(format!(
                    "Failed to read ROM file '{}': {}",
                    options.rom, e
                )))
            })?;

            let rom = Rom::new(&rom_data).map_err(EmulatorError::Rom)?;

            println!(
                "Loaded ROM: {} PRG ROM, {} CHR ROM, Mapper: {}",
                rom.prg_rom.len(),
                rom.chr_rom.len(),
                rom.mapper
            );

            // Initialize CPU log file
            Cpu::init_log()?;

            // Load ROM into CPU memory
            cpu.load_rom(rom.clone());
            cpu.reset();

            // Disassemble ROM for debugging
            let start_address = crate::cpu::core::PROGRAM_ADDRESS;
            let end_address = start_address + std::cmp::min(rom.prg_rom.len(), 0x100) as u16;
            if options.debug {
                cpu.disassemble(start_address, end_address);
            }
        }

        Ok(Emulator {
            options,
            cpu,
            main_canvas,
            debug_canvas,
            event_pump,
            disassembly_lines: Vec::new(),
            texture_creator,
            ttf_context,
            font_path: "assets/font.ttf".to_string(),
            font_size: 12,
        })
    }

    fn create_main_canvas(
        video_subsystem: &VideoSubsystem,
    ) -> Result<Canvas<Window>, EmulatorError> {
        let scale_factor = 2;
        let main_window = video_subsystem
            .window(
                "madNES",
                NES_WIDTH * scale_factor,
                NES_HEIGHT * scale_factor,
            )
            .position_centered()
            .build()
            .map_err(|e| sdl_error_to_emulator_error(e.to_string(), "window"))?;

        let mut main_canvas = main_window
            .into_canvas()
            .build()
            .map_err(|e| sdl_error_to_emulator_error(e.to_string(), "renderer"))?;

        main_canvas.set_draw_color(Color::RGB(0, 255, 0));
        main_canvas.clear();
        main_canvas.present();
        Ok(main_canvas)
    }

    fn create_debug_canvas(
        video_subsystem: &VideoSubsystem,
    ) -> Result<Canvas<Window>, EmulatorError> {
        let debug_window = video_subsystem
            .window("madNES - Debug", 600, 800)
            .position(50, 50)
            .build()
            .map_err(|e| sdl_error_to_emulator_error(e.to_string(), "window"))?;

        let debug_canvas = debug_window
            .into_canvas()
            .build()
            .map_err(|e| sdl_error_to_emulator_error(e.to_string(), "renderer"))?;

        Ok(debug_canvas)
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.disassembly_lines.clear();
    }

    pub fn run(&mut self) -> Result<(), EmulatorError> {
        let mut last_update = Instant::now();
        let mut last_cpu_step = Instant::now();
        let mut cpu_running = true;
        let mut auto_mode = false;
        let mut step_requested = false;

        'running: loop {
            // Collect events first to avoid multiple mutable borrows of self
            let events: Vec<Event> = self.event_pump.poll_iter().collect();

            for event in events {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::KeyDown {
                        keycode: Some(Keycode::N),
                        ..
                    } => {
                        // Step to next instruction
                        step_requested = true;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        self.reset();
                        println!("Emulator reset");
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::I),
                        ..
                    } => {
                        // interrupt CPU
                        self.cpu.irq();
                        println!("CPU interrupted");
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        // Toggle between auto and manual mode
                        auto_mode = !auto_mode;
                        println!(
                            "Switched to {} mode",
                            if auto_mode { "AUTO" } else { "MANUAL" }
                        );
                    }
                    _ => {}
                }
            }

            // Update logic
            let now = Instant::now();
            let dt = now.duration_since(last_update).as_secs_f64();
            last_update = now;

            self.update(dt);

            // Execute CPU instructions
            if cpu_running {
                let should_step = if auto_mode {
                    // Auto mode: step at regular intervals (about 10 Hz for visibility)
                    now.duration_since(last_cpu_step).as_millis() > 100
                } else {
                    // Manual mode: step only when N key is pressed
                    step_requested
                };

                if should_step {
                    match self.cpu.step() {
                        Ok(should_continue) => {
                            cpu_running = should_continue;
                        }
                        Err(e) => {
                            eprintln!("CPU error: {}", e);
                            cpu_running = false;
                        }
                    }
                    last_cpu_step = now;
                    step_requested = false; // Reset the step request
                }
            }

            // Update disassembly for current area around PC
            let pc = self.cpu.get_pc();
            self.disassembly_lines = self
                .cpu
                .disassemble_to_string(pc.saturating_sub(10), pc + 20);

            self.render(auto_mode)?;

            // Cap framerate to ~60 FPS
            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }

    fn render(&mut self, auto_mode: bool) -> Result<(), EmulatorError> {
        // Render main window with NES screen
        self.main_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.main_canvas.clear();

        // Calculate scaling to fit the window
        let window_size = self.main_canvas.window().size();
        let scale_x = window_size.0 as f32 / NES_WIDTH as f32;
        let scale_y = window_size.1 as f32 / NES_HEIGHT as f32;
        let scale = scale_x.min(scale_y) as u32; // Use uniform scaling

        // Center the NES screen in the window
        let scaled_width = NES_WIDTH * scale;
        let scaled_height = NES_HEIGHT * scale;
        let offset_x = (window_size.0 - scaled_width) / 2;
        let offset_y = (window_size.1 - scaled_height) / 2;

        // Render NES screen pixel by pixel
        self.render_nes_screen(offset_x as i32, offset_y as i32, scale)?;

        self.main_canvas.present();

        // Render debug window with CPU status and disassembly info
        self.debug_canvas.set_draw_color(Color::RGB(0, 0, 100));
        self.debug_canvas.clear();

        // Batch all text rendering to load font only once
        let mut text_batch = Vec::new();

        // Prepare all text items for batch rendering
        text_batch.push((
            "CPU REGISTERS".to_string(),
            10,
            10,
            Color::RGB(255, 255, 255),
        ));

        let mode_text = if auto_mode {
            "MODE: AUTO (SPACE=manual, N=step)"
        } else {
            "MODE: MANUAL (SPACE=auto, N=step)"
        };
        text_batch.push((mode_text.to_string(), 200, 10, Color::RGB(255, 200, 100)));

        text_batch.push((
            format!("PC: ${:04X}", self.cpu.get_pc()),
            10,
            30,
            Color::RGB(0, 255, 255),
        ));
        text_batch.push((
            format!("A:  ${:02X}  ({:3})", self.cpu.get_a(), self.cpu.get_a()),
            10,
            50,
            Color::RGB(0, 255, 255),
        ));
        text_batch.push((
            format!("X:  ${:02X}  ({:3})", self.cpu.get_x(), self.cpu.get_x()),
            10,
            70,
            Color::RGB(0, 255, 255),
        ));
        text_batch.push((
            format!("Y:  ${:02X}  ({:3})", self.cpu.get_y(), self.cpu.get_y()),
            10,
            90,
            Color::RGB(0, 255, 255),
        ));
        text_batch.push((
            format!("SP: ${:02X}  ({:3})", self.cpu.get_sp(), self.cpu.get_sp()),
            10,
            110,
            Color::RGB(0, 255, 255),
        ));

        // Status flags
        let status_byte = self.cpu.get_status();
        text_batch.push((
            format!("P:  ${:02X}  ({:08b})", status_byte, status_byte),
            10,
            130,
            Color::RGB(0, 255, 255),
        ));
        text_batch.push((
            "FLAGS: N V - B D I Z C".to_string(),
            10,
            150,
            Color::RGB(200, 200, 200),
        ));

        let flags_status = format!(
            "       {} {} {} {} {} {} {} {}",
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Negative) {
                "1"
            } else {
                "0"
            },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Overflow) {
                "1"
            } else {
                "0"
            },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Unused) {
                "1"
            } else {
                "0"
            },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Break) {
                "1"
            } else {
                "0"
            },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Decimal) {
                "1"
            } else {
                "0"
            },
            if self
                .cpu
                .get_flag(crate::cpu::flags::StatusFlag::InterruptDisable)
            {
                "1"
            } else {
                "0"
            },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Zero) {
                "1"
            } else {
                "0"
            },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Carry) {
                "1"
            } else {
                "0"
            }
        );
        text_batch.push((flags_status, 10, 170, Color::RGB(200, 200, 200)));

        // Show next instruction to be executed
        let next_instruction = self.cpu.disassemble_current_instruction();
        text_batch.push((
            format!("NEXT: {}", next_instruction),
            300,
            150,
            Color::RGB(255, 255, 255),
        ));

        // Show instruction count and cycles
        text_batch.push((
            format!("INSTRUCTIONS: {}", self.cpu.get_instruction_count()),
            300,
            170,
            Color::RGB(255, 200, 255),
        ));
        text_batch.push((
            format!("CYCLES: {}", self.cpu.get_cycles()),
            450,
            170,
            Color::RGB(255, 200, 255),
        ));

        // Disassembly title
        text_batch.push((
            "DISASSEMBLY".to_string(),
            10,
            200,
            Color::RGB(255, 255, 255),
        ));

        // Prepare disassembly lines
        let lines_to_render: Vec<String> =
            self.disassembly_lines.iter().take(30).cloned().collect();

        let current_pc = self.cpu.get_pc();

        // Add disassembly lines to batch
        for (i, line) in lines_to_render.iter().enumerate() {
            let y = i as i32 * 18 + 220;

            // Extract address from the line (format: $XXXX: ...)
            let should_highlight = if let Some(addr_end) = line.find(':') {
                if line.starts_with('$') && addr_end > 1 {
                    if let Ok(line_addr) = u16::from_str_radix(&line[1..addr_end], 16) {
                        line_addr == current_pc
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            // Highlight current instruction (the line with current PC)
            if should_highlight {
                self.debug_canvas.set_draw_color(Color::RGB(40, 40, 0));
                self.debug_canvas
                    .fill_rect(Rect::new(5, y - 2, 590, 16))
                    .map_err(|e| EmulatorError::Sdl(SdlError::RendererCreationFailed(e)))?;
                text_batch.push((line.clone(), 10, y, Color::RGB(255, 255, 0)));
            } else {
                text_batch.push((line.clone(), 10, y, Color::RGB(0, 255, 0)));
            }
        }

        // Add separator
        self.debug_canvas.set_draw_color(Color::RGB(100, 100, 100));
        self.debug_canvas
            .fill_rect(Rect::new(5, 190, 590, 2))
            .map_err(|e| EmulatorError::Sdl(SdlError::RendererCreationFailed(e)))?;

        self.render_text_batch(&text_batch)?;

        self.debug_canvas.present();
        Ok(())
    }

    // renders multiple lines with a single font load
    fn render_text_batch(
        &mut self,
        text_items: &[(String, i32, i32, Color)],
    ) -> Result<(), EmulatorError> {
        if text_items.is_empty() {
            return Ok(());
        }

        // Load font once for the entire batch
        let font = self
            .ttf_context
            .load_font(&self.font_path, self.font_size)
            .map_err(|e| EmulatorError::Sdl(SdlError::FontLoadingFailed(e.to_string())))?;

        for (text, x, y, color) in text_items {
            // Create a surface from the text
            let surface = font
                .render(text)
                .blended(*color)
                .map_err(|e| EmulatorError::Sdl(SdlError::FontLoadingFailed(e.to_string())))?;

            // Create a texture from the surface
            let texture = self
                .texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| EmulatorError::Sdl(SdlError::TextureCreationFailed(e.to_string())))?;

            // Get the text dimensions
            let text_width = surface.width();
            let text_height = surface.height();

            // Render the texture to the canvas
            let dst_rect = Rect::new(*x, *y, text_width, text_height);
            self.debug_canvas
                .copy(&texture, None, dst_rect)
                .map_err(|e| EmulatorError::Sdl(SdlError::RendererCreationFailed(e.to_string())))?;
        }

        Ok(())
    }

    fn update(&mut self, dt: f64) {
        // Update emulator state (currently minimal since we're not implementing full PPU timing)
        // In a full NES emulator, this would handle PPU timing, APU updates, etc.
        let _ = dt; // Acknowledge parameter to avoid unused warning
    }

    /// Render the NES screen by reading CHR ROM data and converting it to pixels
    fn render_nes_screen(
        &mut self,
        offset_x: i32,
        offset_y: i32,
        scale: u32,
    ) -> Result<(), EmulatorError> {
        // Basic NES color palette (simplified NTSC palette)
        let ntsc_palette = [
            Color::RGB(84, 84, 84),    // 0x00 - Dark gray
            Color::RGB(0, 30, 116),    // 0x01 - Dark blue
            Color::RGB(8, 16, 144),    // 0x02 - Purple
            Color::RGB(48, 0, 136),    // 0x03 - Dark purple
            Color::RGB(68, 0, 100),    // 0x04 - Dark red
            Color::RGB(92, 0, 48),     // 0x05 - Brown
            Color::RGB(84, 4, 0),      // 0x06 - Dark brown
            Color::RGB(60, 24, 0),     // 0x07 - Orange brown
            Color::RGB(32, 42, 0),     // 0x08 - Dark green
            Color::RGB(8, 58, 0),      // 0x09 - Green
            Color::RGB(0, 64, 0),      // 0x0A - Light green
            Color::RGB(0, 60, 13),     // 0x0B - Cyan green
            Color::RGB(0, 50, 60),     // 0x0C - Dark cyan
            Color::RGB(0, 0, 0),       // 0x0D - Black
            Color::RGB(0, 0, 0),       // 0x0E - Black
            Color::RGB(0, 0, 0),       // 0x0F - Black
            Color::RGB(152, 150, 152), // 0x10 - Light gray
            Color::RGB(8, 76, 196),    // 0x11 - Blue
            Color::RGB(48, 50, 236),   // 0x12 - Light purple
            Color::RGB(92, 30, 228),   // 0x13 - Pink
            Color::RGB(136, 20, 176),  // 0x14 - Magenta
            Color::RGB(160, 20, 100),  // 0x15 - Red
            Color::RGB(152, 34, 32),   // 0x16 - Orange red
            Color::RGB(120, 60, 0),    // 0x17 - Orange
            Color::RGB(84, 90, 0),     // 0x18 - Yellow green
            Color::RGB(40, 114, 0),    // 0x19 - Light green
            Color::RGB(8, 124, 0),     // 0x1A - Green
            Color::RGB(0, 118, 40),    // 0x1B - Cyan
            Color::RGB(0, 102, 120),   // 0x1C - Light cyan
            Color::RGB(0, 0, 0),       // 0x1D - Black
            Color::RGB(0, 0, 0),       // 0x1E - Black
            Color::RGB(0, 0, 0),       // 0x1F - Black
            Color::RGB(236, 238, 236), // 0x20 - White
            Color::RGB(76, 154, 236),  // 0x21 - Light blue
            Color::RGB(120, 124, 236), // 0x22 - Lavender
            Color::RGB(176, 98, 236),  // 0x23 - Light pink
            Color::RGB(228, 84, 236),  // 0x24 - Pink
            Color::RGB(236, 88, 180),  // 0x25 - Light red
            Color::RGB(236, 106, 100), // 0x26 - Salmon
            Color::RGB(212, 136, 32),  // 0x27 - Light orange
            Color::RGB(160, 170, 0),   // 0x28 - Yellow
            Color::RGB(116, 196, 0),   // 0x29 - Light green
            Color::RGB(76, 208, 32),   // 0x2A - Lime
            Color::RGB(56, 204, 108),  // 0x2B - Light cyan
            Color::RGB(56, 180, 204),  // 0x2C - Cyan
            Color::RGB(60, 60, 60),    // 0x2D - Dark gray
            Color::RGB(0, 0, 0),       // 0x2E - Black
            Color::RGB(0, 0, 0),       // 0x2F - Black
            Color::RGB(236, 238, 236), // 0x30 - White (repeat)
            Color::RGB(168, 204, 236), // 0x31 - Very light blue
            Color::RGB(188, 188, 236), // 0x32 - Very light purple
            Color::RGB(212, 178, 236), // 0x33 - Very light pink
            Color::RGB(236, 174, 236), // 0x34 - Very light magenta
            Color::RGB(236, 174, 212), // 0x35 - Very light red
            Color::RGB(236, 180, 176), // 0x36 - Very light orange
            Color::RGB(228, 196, 144), // 0x37 - Cream
            Color::RGB(204, 210, 120), // 0x38 - Light yellow
            Color::RGB(180, 222, 120), // 0x39 - Very light green
            Color::RGB(168, 226, 144), // 0x3A - Very light lime
            Color::RGB(152, 226, 180), // 0x3B - Very light cyan
            Color::RGB(160, 214, 228), // 0x3C - Very light blue
            Color::RGB(160, 162, 160), // 0x3D - Light gray
            Color::RGB(0, 0, 0),       // 0x3E - Black
            Color::RGB(0, 0, 0),       // 0x3F - Black
        ];

        // Create a simple test pattern if no ROM is loaded or for demonstration
        // This will show a checkerboard pattern using CHR ROM data if available
        for y in 0..NES_HEIGHT {
            for x in 0..NES_WIDTH {
                // Read pattern data from CHR ROM or create test pattern
                let color_index = self.get_pixel_color(x, y);
                let color = ntsc_palette[color_index as usize % ntsc_palette.len()];

                // Draw scaled pixel
                let pixel_rect = Rect::new(
                    offset_x + (x * scale) as i32,
                    offset_y + (y * scale) as i32,
                    scale,
                    scale,
                );

                self.main_canvas.set_draw_color(color);
                self.main_canvas
                    .fill_rect(pixel_rect)
                    .map_err(|e| EmulatorError::Sdl(SdlError::RendererCreationFailed(e)))?;
            }
        }

        Ok(())
    }

    /// Get the color index for a pixel at the given coordinates
    /// This is a simplified implementation that reads from CHR ROM
    fn get_pixel_color(&self, x: u32, y: u32) -> u8 {
        // Try to read from CHR ROM if available
        if let Some(chr_data) = self.get_chr_rom_data() {
            if !chr_data.is_empty() {
                // Calculate which 8x8 tile we're in
                let tile_x = x / 8;
                let tile_y = y / 8;
                let tiles_per_row = 32; // Standard NES nametable width
                let tile_index = (tile_y * tiles_per_row + tile_x) as usize;

                // Get pixel within the tile (0-7)
                let pixel_x = x % 8;
                let pixel_y = y % 8;

                // Each tile is 16 bytes in CHR ROM (8 bytes for low bit plane, 8 for high)
                if tile_index * 16 + 15 < chr_data.len() {
                    let tile_offset = tile_index * 16;

                    // Get the two bit planes for this pixel
                    let low_byte = chr_data[tile_offset + pixel_y as usize];
                    let high_byte = chr_data[tile_offset + 8 + pixel_y as usize];

                    // Extract the specific pixel (bit 7-pixel_x)
                    let bit_pos = 7 - pixel_x;
                    let low_bit = (low_byte >> bit_pos) & 1;
                    let high_bit = (high_byte >> bit_pos) & 1;

                    // Combine bits to get color index (0-3)
                    let color_index = (high_bit << 1) | low_bit;

                    // Map to palette (simplified - using first 4 colors)
                    return match color_index {
                        0 => 0x0F, // Black
                        1 => 0x00, // Dark gray
                        2 => 0x10, // Light gray
                        3 => 0x20, // White
                        _ => 0x0F,
                    };
                }
            }
        }

        // Fallback: Create a simple test pattern
        let tile_x = x / 8;
        let tile_y = y / 8;

        // Checkerboard pattern
        if (tile_x + tile_y) % 2 == 0 {
            // Show some CHR data pattern or gradient
            ((x + y) / 8) as u8 % 64
        } else {
            // Different pattern for alternating tiles
            ((x * 2 + y) / 16) as u8 % 64
        }
    }

    /// Get CHR ROM data from the CPU's memory
    fn get_chr_rom_data(&self) -> Option<&[u8]> {
        self.cpu.get_chr_rom()
    }
}
