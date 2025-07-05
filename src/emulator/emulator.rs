extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
use crate::cpu::cpu::Cpu;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;
use glutin_window::GlutinWindow as Window;

pub struct Emulator {
    cpu: Cpu,
    gl: GlGraphics,
    rotation: f64,
    opengl: OpenGL,
    window: Window,
}

impl Emulator {
    pub fn new() -> Self {
        let cpu = Cpu::new();

        // Change this to OpenGL::V2_1 if not working.
        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new("madNES", [800, 600])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Emulator {
            cpu,
            gl: GlGraphics::new(OpenGL::V3_2),
            rotation: 0.0,
            opengl,
            window,
        }
    }

    pub fn run(&mut self, game_code: Vec<u8>) {
        self.cpu.load_program(game_code, 0x0600);

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(args) = e.update_args() {
                self.update(&args);
            }
        }
    }

    fn render(&mut self, args: &RenderArgs) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}