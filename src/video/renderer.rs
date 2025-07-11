use crate::error::{EmulatorError, SdlError};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Video renderer for NES graphics
pub struct Renderer {
    canvas: Canvas<Window>,
    width: u32,
    height: u32,
    scale: u32,
}

impl Renderer {
    pub fn new(canvas: Canvas<Window>, width: u32, height: u32, scale: u32) -> Self {
        Self {
            canvas,
            width,
            height,
            scale,
        }
    }

    pub fn clear(&mut self, color: Color) -> Result<(), EmulatorError> {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
        Ok(())
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), EmulatorError> {
        self.canvas.set_draw_color(color);

        let scaled_x = x * self.scale;
        let scaled_y = y * self.scale;

        if self.scale == 1 {
            self.canvas
                .draw_point((scaled_x as i32, scaled_y as i32))
                .map_err(|e| EmulatorError::Sdl(SdlError::RendererCreationFailed(e)))?;
        } else {
            let rect = Rect::new(scaled_x as i32, scaled_y as i32, self.scale, self.scale);
            self.canvas
                .fill_rect(rect)
                .map_err(|e| EmulatorError::Sdl(SdlError::RendererCreationFailed(e)))?;
        }

        Ok(())
    }

    pub fn draw_frame(&mut self, frame_buffer: &[u8]) -> Result<(), EmulatorError> {
        // Draw 256x240 NES frame
        for y in 0..240 {
            for x in 0..256 {
                let pixel_index = (y * 256 + x) * 3;
                if pixel_index + 2 < frame_buffer.len() {
                    let r = frame_buffer[pixel_index];
                    let g = frame_buffer[pixel_index + 1];
                    let b = frame_buffer[pixel_index + 2];

                    self.draw_pixel(x as u32, y as u32, Color::RGB(r, g, b))?;
                }
            }
        }

        Ok(())
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
