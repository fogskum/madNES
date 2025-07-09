use crate::cpu::cpu::Cpu;
use crate::cpu::memory::Memory;
use crate::rom::rom::Rom;
use crate::emulator::options::EmulatorOptions;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump};
use sdl2::ttf::{Sdl2TtfContext};
use std::time::{Duration, Instant};

pub struct Emulator {
    #[allow(dead_code)]
    options: EmulatorOptions,
    cpu: Cpu,
    main_canvas: Canvas<Window>,
    debug_canvas: Canvas<Window>,
    event_pump: EventPump,
    rotation: f64,
    disassembly_lines: Vec<String>,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
    // Store the font path for lazy loading
    font_path: String,
    font_size: u16,
}

impl Emulator {
    pub fn new(options: EmulatorOptions) -> Result<Self, String> {
        let mut cpu = Cpu::new();

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        // Initialize TTF
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        
        // Create main emulator window
        let main_window = video_subsystem
            .window("madNES - Main", options.width, options.height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut main_canvas = main_window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        main_canvas.set_draw_color(Color::RGB(0, 255, 0));
        main_canvas.clear();
        main_canvas.present();

        // Create debug window for disassembly
        let debug_window = video_subsystem
            .window("madNES - Debug", 600, 800)
            .position(50, 50)
            .build()
            .map_err(|e| e.to_string())?;

        let mut debug_canvas = debug_window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        // Create texture creator for text rendering
        let texture_creator = debug_canvas.texture_creator();

        debug_canvas.set_draw_color(Color::RGB(0, 0, 100));
        debug_canvas.clear();
        debug_canvas.present();

        let event_pump = sdl_context.event_pump()?;

        // Load ROM if provided
        if !options.rom.is_empty() {
            let rom_data = std::fs::read(&options.rom)
                .map_err(|e| format!("Failed to read ROM file '{}': {}", options.rom, e))?;
            
            let rom = Rom::new(&rom_data)
                .map_err(|e| format!("Failed to parse ROM file '{}': {}", options.rom, e))?;
            
            println!("Loaded ROM: {} PRG ROM, {} CHR ROM, Mapper: {}", 
                     rom.prg_rom.len(), rom.chr_rom.len(), rom.mapper);
            
            // Initialize CPU log file
            Cpu::init_log().map_err(|e| format!("Failed to initialize CPU log: {}", e))?;
            
            // Load ROM into CPU memory
            cpu.load_rom(rom.clone());
            
            // Set reset vector to start of PRG ROM
            cpu.write_word(0xFFFC, 0x8000);
            cpu.reset();
            
            // Disassemble ROM for debugging
            if options.debug {
                cpu.disassemble(0x8000, 0x8000 + std::cmp::min(rom.prg_rom.len(), 0x100) as u16);
            }
        }

        Ok(Emulator {
            options,
            cpu,
            main_canvas,
            debug_canvas,
            event_pump,
            rotation: 0.0,
            disassembly_lines: Vec::new(),
            texture_creator,
            ttf_context,
            font_path: "assets/font.ttf".to_string(),
            font_size: 12
        })
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.disassembly_lines.clear();
    }

    pub fn run(&mut self) -> Result<(), String> {

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
                    },
                    Event::KeyDown { 
                        keycode: Some(Keycode::R),
                        ..
                    } => {
                        self.reset();
                        println!("Emulator reset");
                    },
                    Event::KeyDown {
                        keycode: Some(Keycode::I),
                        ..
                    } => {
                        // interrupt CPU
                        self.cpu.irq();
                        println!("CPU interrupted");
                    },
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        // Toggle between auto and manual mode
                        auto_mode = !auto_mode;
                        println!("Switched to {} mode", if auto_mode { "AUTO" } else { "MANUAL" });
                    },
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
                    cpu_running = self.cpu.step();
                    last_cpu_step = now;
                    step_requested = false; // Reset the step request
                }
            }
            
            // Update disassembly for current area around PC
            let pc = self.cpu.get_pc();
            self.disassembly_lines = self.cpu.disassemble_to_string(pc.saturating_sub(10), pc + 20);
            
            self.render(auto_mode)?;

            // Cap framerate to ~60 FPS
            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }

    fn render(&mut self, auto_mode: bool) -> Result<(), String> {
        // Render main window with the rotating square
        self.main_canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.main_canvas.clear();

        // Draw a red rotating square in the center
        let window_size = self.main_canvas.window().size();
        let (center_x, center_y) = (window_size.0 as i32 / 2, window_size.1 as i32 / 2);
        
        // Simple rotation approximation - just move the square in a circle
        let radius = 100.0;
        let x = center_x + (radius * self.rotation.cos()) as i32 - 25;
        let y = center_y + (radius * self.rotation.sin()) as i32 - 25;
        
        self.main_canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.main_canvas.fill_rect(Rect::new(x, y, 50, 50))?;
        self.main_canvas.present();

        // Render debug window with CPU status and disassembly info
        self.debug_canvas.set_draw_color(Color::RGB(0, 0, 100));
        self.debug_canvas.clear();

        // OPTIMIZATION: Batch all text rendering to load font only once
        let mut text_batch = Vec::new();
        
        // Prepare all text items for batch rendering
        text_batch.push((format!("CPU REGISTERS"), 10, 10, Color::RGB(255, 255, 255)));
        
        let mode_text = if auto_mode { "MODE: AUTO (SPACE=manual, N=step)" } else { "MODE: MANUAL (SPACE=auto, N=step)" };
        text_batch.push((mode_text.to_string(), 200, 10, Color::RGB(255, 200, 100)));
        
        text_batch.push((format!("PC: ${:04X}", self.cpu.get_pc()), 10, 30, Color::RGB(0, 255, 255)));
        text_batch.push((format!("A:  ${:02X}  ({:3})", self.cpu.get_a(), self.cpu.get_a()), 10, 50, Color::RGB(0, 255, 255)));
        text_batch.push((format!("X:  ${:02X}  ({:3})", self.cpu.get_x(), self.cpu.get_x()), 10, 70, Color::RGB(0, 255, 255)));
        text_batch.push((format!("Y:  ${:02X}  ({:3})", self.cpu.get_y(), self.cpu.get_y()), 10, 90, Color::RGB(0, 255, 255)));
        text_batch.push((format!("SP: ${:02X}  ({:3})", self.cpu.get_sp(), self.cpu.get_sp()), 10, 110, Color::RGB(0, 255, 255)));
        
        // Status flags
        let status_byte = self.cpu.get_status();
        text_batch.push((format!("P:  ${:02X}  ({:08b})", status_byte, status_byte), 10, 130, Color::RGB(0, 255, 255)));
        text_batch.push((format!("FLAGS: N V - B D I Z C"), 10, 150, Color::RGB(200, 200, 200)));
        
        let flags_status = format!("       {} {} {} {} {} {} {} {}", 
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Negative) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Overflow) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Unused) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Break) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Decimal) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::InterruptDisable) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Zero) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::flags::StatusFlag::Carry) { "1" } else { "0" }
        );
        text_batch.push((flags_status, 10, 170, Color::RGB(200, 200, 200)));
        
        // Show next instruction to be executed
        let next_instruction = self.cpu.disassemble_current_instruction();
        text_batch.push((format!("NEXT: {}", next_instruction), 300, 150, Color::RGB(255, 255, 255)));
        
        // Show instruction count and cycles
        text_batch.push((format!("INSTRUCTIONS: {}", self.cpu.get_instruction_count()), 300, 170, Color::RGB(255, 200, 255)));
        text_batch.push((format!("CYCLES: {}", self.cpu.get_cycles()), 450, 170, Color::RGB(255, 200, 255)));
        
        // Disassembly title
        text_batch.push((format!("DISASSEMBLY"), 10, 200, Color::RGB(255, 255, 255)));
        
        // Prepare disassembly lines
        let lines_to_render: Vec<String> = self.disassembly_lines
            .iter()
            .take(30)
            .cloned()
            .collect();
        
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
                self.debug_canvas.fill_rect(Rect::new(5, y - 2, 590, 16))?;
                text_batch.push((line.clone(), 10, y, Color::RGB(255, 255, 0)));
            } else {
                text_batch.push((line.clone(), 10, y, Color::RGB(0, 255, 0)));
            }
        }
        
        // Add separator
        self.debug_canvas.set_draw_color(Color::RGB(100, 100, 100));
        self.debug_canvas.fill_rect(Rect::new(5, 190, 590, 2))?;
        
        self.render_text_batch(&text_batch)?;
        
        self.debug_canvas.present();
        Ok(())
    }

    // Optimized batch text rendering - renders multiple lines with a single font load
    fn render_text_batch(&mut self, text_items: &[(String, i32, i32, Color)]) -> Result<(), String> {
        if text_items.is_empty() {
            return Ok(());
        }

        // Load font once for the entire batch
        let font = self.ttf_context.load_font(&self.font_path, self.font_size).map_err(|e| e.to_string())?;
        
        for (text, x, y, color) in text_items {
            // Create a surface from the text
            let surface = font
                .render(text)
                .blended(*color)
                .map_err(|e| e.to_string())?;
            
            // Create a texture from the surface
            let texture = self.texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;
            
            // Get the text dimensions
            let text_width = surface.width();
            let text_height = surface.height();
            
            // Render the texture to the canvas
            let dst_rect = Rect::new(*x, *y, text_width, text_height);
            self.debug_canvas.copy(&texture, None, dst_rect)?;
        }
        
        Ok(())
    }

    fn update(&mut self, dt: f64) {
        // Rotate 2 radians per second
        self.rotation += 2.0 * dt;
    }
}