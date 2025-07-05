use crate::cpu::cpu::Cpu;
use crate::emulator::options::EmulatorOptions;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump};
use std::time::{Duration, Instant};

pub struct Emulator {
    cpu: Cpu,
    main_canvas: Canvas<Window>,
    debug_canvas: Canvas<Window>,
    event_pump: EventPump,
    rotation: f64,
    disassembly_lines: Vec<String>,
}

impl Emulator {
    pub fn new(options: &EmulatorOptions) -> Result<Self, String> {
        let cpu = Cpu::new();

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

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
        })
    }

    pub fn run(&mut self, game_code: Vec<u8>) -> Result<(), String> {
        self.cpu.load_program(game_code, 0x0600);

        let mut last_update = Instant::now();
        let mut last_cpu_step = Instant::now();
        let mut cpu_running = true;
        
        'running: loop {
            // Handle events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            // Update logic
            let now = Instant::now();
            let dt = now.duration_since(last_update).as_secs_f64();
            last_update = now;
            
            self.update(dt);
            
            // Execute CPU instructions at a slower pace (about 10 Hz for visibility)
            if cpu_running && now.duration_since(last_cpu_step).as_millis() > 100 {
                cpu_running = self.cpu.step();
                last_cpu_step = now;
            }
            
            // Update disassembly for current area around PC
            let pc = self.cpu.get_pc();
            self.disassembly_lines = self.cpu.disassemble_to_string(pc.saturating_sub(10), pc + 20);
            
            self.render()?;

            // Cap framerate to ~60 FPS
            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }

    fn render(&mut self) -> Result<(), String> {
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

        // Render debug window with disassembly info
        self.debug_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.debug_canvas.clear();
        
        // Render actual disassembly text using simple bitmap-style rendering
        let lines_to_render: Vec<String> = self.disassembly_lines.iter().take(40).cloned().collect();
        
        for (i, line) in lines_to_render.iter().enumerate() {
            let y = i as i32 * 18 + 10;
            
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
        let mut x = start_x;
        for ch in text.chars() {
            match ch {
                ' ' => {
                    x += 8; // Space width
                }
                '$' | ':' => {
                    // Draw symbol as thin rectangle
                    self.debug_canvas.fill_rect(Rect::new(x, y + 2, 6, 10))?;
                    x += 8;
                }
                '0'..='9' | 'A'..='F' => {
                    // Draw hex digits as wider rectangles
                    self.debug_canvas.fill_rect(Rect::new(x, y + 2, 7, 10))?;
                    x += 8;
                }
                'a'..='z' | 'A'..='Z' => {
                    // Draw letters as medium rectangles
                    self.debug_canvas.fill_rect(Rect::new(x, y + 2, 6, 10))?;
                    x += 7;
                }
                _ => {
                    // Draw other characters as small rectangles
                    self.debug_canvas.fill_rect(Rect::new(x, y + 4, 4, 6))?;
                    x += 6;
                }
            }
        }
        Ok(())
    }

    fn update(&mut self, dt: f64) {
        // Rotate 2 radians per second
        self.rotation += 2.0 * dt;
    }
}