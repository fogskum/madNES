use crate::error::MemoryResult;

/// NES controller state
#[derive(Debug, Clone)]
pub struct Controller {
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    
    // Internal state
    strobe: bool,
    index: u8,
    buttons: u8,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            a: false,
            b: false,
            select: false,
            start: false,
            up: false,
            down: false,
            left: false,
            right: false,
            strobe: false,
            index: 0,
            buttons: 0,
        }
    }
    
    pub fn set_button(&mut self, button: Button, pressed: bool) {
        match button {
            Button::A => self.a = pressed,
            Button::B => self.b = pressed,
            Button::Select => self.select = pressed,
            Button::Start => self.start = pressed,
            Button::Up => self.up = pressed,
            Button::Down => self.down = pressed,
            Button::Left => self.left = pressed,
            Button::Right => self.right = pressed,
        }
        
        self.update_buttons();
    }
    
    pub fn write(&mut self, value: u8) -> MemoryResult<()> {
        let new_strobe = (value & 0x01) != 0;
        
        if self.strobe && !new_strobe {
            // Strobe went from high to low - latch current state
            self.index = 0;
        }
        
        self.strobe = new_strobe;
        
        if self.strobe {
            self.index = 0;
        }
        
        Ok(())
    }
    
    pub fn read(&mut self) -> MemoryResult<u8> {
        let result = if self.index < 8 {
            let result = (self.buttons >> self.index) & 0x01;
            if !self.strobe {
                self.index += 1;
            }
            result
        } else {
            1 // Open bus
        };
        
        Ok(result)
    }
    
    fn update_buttons(&mut self) {
        self.buttons = 0;
        
        if self.a { self.buttons |= 0x01; }
        if self.b { self.buttons |= 0x02; }
        if self.select { self.buttons |= 0x04; }
        if self.start { self.buttons |= 0x08; }
        if self.up { self.buttons |= 0x10; }
        if self.down { self.buttons |= 0x20; }
        if self.left { self.buttons |= 0x40; }
        if self.right { self.buttons |= 0x80; }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
