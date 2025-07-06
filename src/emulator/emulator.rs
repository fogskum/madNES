use crate::cpu::cpu::Cpu;
use crate::emulator::options::EmulatorOptions;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::{EventPump};
use std::time::{Duration, Instant};

pub struct Emulator {
    cpu: Cpu,
    main_canvas: Canvas<Window>,
    debug_canvas: Canvas<Window>,
    event_pump: EventPump,
    rotation: f64,
    disassembly_lines: Vec<String>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Emulator {
    pub fn new(options: &EmulatorOptions) -> Result<Self, String> {
        let cpu = Cpu::new();

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        // Initialize TTF
        let _ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        
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

        debug_canvas.set_draw_color(Color::RGB(0, 0, 0));
        debug_canvas.clear();
        debug_canvas.present();

        let event_pump = sdl_context.event_pump()?;

        Ok(Emulator {
            cpu,
            main_canvas,
            debug_canvas,
            event_pump,
            rotation: 0.0,
            disassembly_lines: Vec::new(),
            texture_creator,
        })
    }

    pub fn run(&mut self, game_code: Vec<u8>) -> Result<(), String> {
        self.cpu.load_program(game_code, 0x0600);

        let mut last_update = Instant::now();
        let mut last_cpu_step = Instant::now();
        let mut cpu_running = true;
        let mut auto_mode = false; // Start in manual stepping mode
        let mut step_requested = false;
        
        'running: loop {
            // Handle events
            for event in self.event_pump.poll_iter() {
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
        self.debug_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.debug_canvas.clear();
        
        // Render CPU status registers at the top
        self.debug_canvas.set_draw_color(Color::RGB(255, 255, 255));
        let cpu_status = format!("CPU REGISTERS");
        self.render_text_simple(&cpu_status, 10, 10)?;
        
        // Show execution mode
        self.debug_canvas.set_draw_color(Color::RGB(255, 200, 100));
        let mode_text = if auto_mode { "MODE: AUTO (SPACE=manual, N=step)" } else { "MODE: MANUAL (SPACE=auto, N=step)" };
        self.render_text_simple(&mode_text, 200, 10)?;
        
        self.debug_canvas.set_draw_color(Color::RGB(0, 255, 255));
        let pc_status = format!("PC: ${:04X}", self.cpu.get_pc());
        self.render_text_simple(&pc_status, 10, 30)?;
        
        let a_status = format!("A:  ${:02X}  ({:3})", self.cpu.get_a(), self.cpu.get_a());
        self.render_text_simple(&a_status, 10, 50)?;
        
        let x_status = format!("X:  ${:02X}  ({:3})", self.cpu.get_x(), self.cpu.get_x());
        self.render_text_simple(&x_status, 10, 70)?;
        
        let y_status = format!("Y:  ${:02X}  ({:3})", self.cpu.get_y(), self.cpu.get_y());
        self.render_text_simple(&y_status, 10, 90)?;
        
        let sp_status = format!("SP: ${:02X}  ({:3})", self.cpu.get_sp(), self.cpu.get_sp());
        self.render_text_simple(&sp_status, 10, 110)?;
        
        // Status flags
        let status_byte = self.cpu.get_status();
        let status_status = format!("P:  ${:02X}  ({:08b})", status_byte, status_byte);
        self.render_text_simple(&status_status, 10, 130)?;
        
        self.debug_canvas.set_draw_color(Color::RGB(200, 200, 200));
        let flags_header = format!("FLAGS: N V - B D I Z C");
        self.render_text_simple(&flags_header, 10, 150)?;
        
        let flags_status = format!("       {} {} {} {} {} {} {} {}", 
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::Negative) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::Overflow) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::Unused) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::Break) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::Decimal) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::InterruptDisable) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::Zero) { "1" } else { "0" },
            if self.cpu.get_flag(crate::cpu::cpu::StatusFlag::Carry) { "1" } else { "0" }
        );
        self.render_text_simple(&flags_status, 10, 170)?;
        
        // Add separator
        self.debug_canvas.set_draw_color(Color::RGB(100, 100, 100));
        self.debug_canvas.fill_rect(Rect::new(5, 190, 590, 2))?;
        
        // Render disassembly title
        self.debug_canvas.set_draw_color(Color::RGB(255, 255, 255));
        let disasm_title = format!("DISASSEMBLY");
        self.render_text_simple(&disasm_title, 10, 200)?;
        
        // Render actual disassembly text using simple bitmap-style rendering
        let lines_to_render: Vec<String> = self.disassembly_lines
            .iter()
            .take(30)
            .cloned()
            .collect();
        
        for (i, line) in lines_to_render.iter().enumerate() {
            let y = i as i32 * 18 + 220;
            
            // Highlight current instruction (middle line, around line 10)
            if i == 10 {
                self.debug_canvas.set_draw_color(Color::RGB(40, 40, 0));
                self.debug_canvas.fill_rect(Rect::new(5, y - 2, 590, 16))?;
                self.debug_canvas.set_draw_color(Color::RGB(255, 255, 0));
            } else {
                self.debug_canvas.set_draw_color(Color::RGB(0, 255, 0));
            }
            
            // Render text as simple rectangles representing characters
            self.render_text_simple(&line, 10, y)?;
        }
        
        self.debug_canvas.present();
        Ok(())
    }

    fn render_text_simple(&mut self, text: &str, start_x: i32, y: i32) -> Result<(), String> {
        // Initialize TTF context for this call
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font("assets/font.ttf", 12).map_err(|e| e.to_string())?;
        
        // Create a surface from the text
        let surface = font
            .render(text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        
        // Create a texture from the surface
        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        
        // Get the text dimensions
        let text_width = surface.width();
        let text_height = surface.height();
        
        // Render the texture to the canvas
        let dst_rect = Rect::new(start_x, y, text_width, text_height);
        self.debug_canvas.copy(&texture, None, dst_rect)?;
        
        Ok(())
    }

    fn update(&mut self, dt: f64) {
        // Rotate 2 radians per second
        self.rotation += 2.0 * dt;
    }
}