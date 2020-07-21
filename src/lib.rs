use crossterm::{
    cursor,
    style::Print,
    ExecutableCommand,
};

use std::mem;

#[derive(Clone)]
pub struct FrameBuffer {
    frame: Vec<char>,
    width: u16,
    height: u16,
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

impl FrameBuffer {
    /// Creates a framebuffer frame
    pub fn create(width: u16, height: u16) -> Self {
        let mut frame = Vec::with_capacity((width * height).into());
        for _ in 0..(width * height) {
            frame.push(' '); // empty symbol...
        }
        FrameBuffer {
            frame,
            width,
            height,
        }
    }

    /// Draws a frame in console. Recommended disable cursor. Because it will interfere with enjoyment of the drawing
    pub fn push_fb(&mut self, mut frame: Vec<char>, stdout: &mut std::io::Stdout) {
        if !do_vecs_match(&frame, &self.frame) {
            mem::swap(&mut frame, &mut self.frame);

            for x in 0..self.width {
                for y in 0..self.height {
                    if self.frame[(y * self.width + x) as usize] != frame[(y * self.width + x) as usize]
                    {
                        &stdout.execute(cursor::MoveTo(x, y));
                        &stdout.execute(Print(self.frame[(y * self.width + x) as usize]));
                    }
                }
            }
        }
    }

    /// Get symbol in framebuffer
    pub fn get(&mut self, x: u16, y: u16) -> char {
        self.frame[(y * self.width + x) as usize]
    }

    /// Set string in framebuffer
    pub fn set(&mut self, text: &str, x: u16, y: u16) {
        for (i, v) in text.chars().enumerate() {
            if (y * self.width + (x + i as u16)) < self.frame.len() as u16 {
                self.frame[(y * self.width + (x + i as u16)) as usize] = v;
            }
        }
    }
}