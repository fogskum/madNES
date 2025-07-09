use crate::input::controller::{Controller, Button};
use crate::error::{MemoryError, MemoryResult};
use sdl2::keyboard::Keycode;

/// Manages input from keyboard and maps to NES controller
pub struct InputManager {
    pub controller1: Controller,
    pub controller2: Controller,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            controller1: Controller::new(),
            controller2: Controller::new(),
        }
    }
    
    pub fn handle_key_down(&mut self, keycode: Keycode) {
        match keycode {
            // Controller 1
            Keycode::Z => self.controller1.set_button(Button::A, true),
            Keycode::X => self.controller1.set_button(Button::B, true),
            Keycode::RShift => self.controller1.set_button(Button::Select, true),
            Keycode::Return => self.controller1.set_button(Button::Start, true),
            Keycode::Up => self.controller1.set_button(Button::Up, true),
            Keycode::Down => self.controller1.set_button(Button::Down, true),
            Keycode::Left => self.controller1.set_button(Button::Left, true),
            Keycode::Right => self.controller1.set_button(Button::Right, true),
            
            // Controller 2 (number pad)
            Keycode::Kp1 => self.controller2.set_button(Button::A, true),
            Keycode::Kp2 => self.controller2.set_button(Button::B, true),
            Keycode::Kp3 => self.controller2.set_button(Button::Select, true),
            Keycode::Kp0 => self.controller2.set_button(Button::Start, true),
            Keycode::Kp8 => self.controller2.set_button(Button::Up, true),
            Keycode::Kp5 => self.controller2.set_button(Button::Down, true),
            Keycode::Kp4 => self.controller2.set_button(Button::Left, true),
            Keycode::Kp6 => self.controller2.set_button(Button::Right, true),
            
            _ => {}
        }
    }
    
    pub fn handle_key_up(&mut self, keycode: Keycode) {
        match keycode {
            // Controller 1
            Keycode::Z => self.controller1.set_button(Button::A, false),
            Keycode::X => self.controller1.set_button(Button::B, false),
            Keycode::RShift => self.controller1.set_button(Button::Select, false),
            Keycode::Return => self.controller1.set_button(Button::Start, false),
            Keycode::Up => self.controller1.set_button(Button::Up, false),
            Keycode::Down => self.controller1.set_button(Button::Down, false),
            Keycode::Left => self.controller1.set_button(Button::Left, false),
            Keycode::Right => self.controller1.set_button(Button::Right, false),
            
            // Controller 2 (number pad)
            Keycode::Kp1 => self.controller2.set_button(Button::A, false),
            Keycode::Kp2 => self.controller2.set_button(Button::B, false),
            Keycode::Kp3 => self.controller2.set_button(Button::Select, false),
            Keycode::Kp0 => self.controller2.set_button(Button::Start, false),
            Keycode::Kp8 => self.controller2.set_button(Button::Up, false),
            Keycode::Kp5 => self.controller2.set_button(Button::Down, false),
            Keycode::Kp4 => self.controller2.set_button(Button::Left, false),
            Keycode::Kp6 => self.controller2.set_button(Button::Right, false),
            
            _ => {}
        }
    }
    
    pub fn read_controller(&mut self, controller: u8) -> MemoryResult<u8> {
        match controller {
            0 => self.controller1.read(),
            1 => self.controller2.read(),
            _ => Err(MemoryError::InvalidRegion(controller as u16)),
        }
    }
    
    pub fn write_controller(&mut self, controller: u8, value: u8) -> MemoryResult<()> {
        match controller {
            0 => self.controller1.write(value),
            1 => self.controller2.write(value),
            _ => Err(MemoryError::InvalidRegion(controller as u16)),
        }
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}
