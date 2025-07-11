use std::collections::VecDeque;

/// Circular audio buffer for samples
pub struct AudioBuffer {
    buffer: VecDeque<f32>,
    capacity: usize,
}

impl AudioBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, sample: f32) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample);
    }

    pub fn pop(&mut self) -> Option<f32> {
        self.buffer.pop_front()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
