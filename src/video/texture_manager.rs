use crate::error::{EmulatorError, SdlError};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;

/// Manages textures for efficient rendering
pub struct TextureManager<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    frame_texture: Option<Texture<'a>>,
    width: u32,
    height: u32,
}

impl<'a> TextureManager<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>, width: u32, height: u32) -> Self {
        Self {
            texture_creator,
            frame_texture: None,
            width,
            height,
        }
    }
    
    pub fn create_frame_texture(&mut self) -> Result<(), EmulatorError> {
        let texture = self.texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, self.width, self.height)
            .map_err(|e| EmulatorError::Sdl(SdlError::TextureCreationFailed(e.to_string())))?;
        
        self.frame_texture = Some(texture);
        Ok(())
    }
    
    pub fn update_frame_texture(&mut self, frame_buffer: &[u8]) -> Result<(), EmulatorError> {
        if let Some(ref mut texture) = self.frame_texture {
            texture.update(None, frame_buffer, self.width as usize * 3)
                .map_err(|e| EmulatorError::Sdl(SdlError::TextureCreationFailed(e.to_string())))?;
        }
        Ok(())
    }
    
    pub fn render_frame(&self, canvas: &mut Canvas<Window>, dst_rect: Option<Rect>) -> Result<(), EmulatorError> {
        if let Some(ref texture) = self.frame_texture {
            canvas.copy(texture, None, dst_rect)
                .map_err(|e| EmulatorError::Sdl(SdlError::RendererCreationFailed(e)))?;
        }
        Ok(())
    }
}
