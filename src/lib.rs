//! # Console (Crossterm) double-buffering library
//! ![Scheme of working](https://i.imgur.com/K6dZxZy.png)

use crossterm::{cursor, style::Print, ExecutableCommand};
use std::mem;

#[derive(Clone)]
/// The structure of framebuffer
pub struct FrameBuffer {
    current_frame: Vec<char>,
    frame: Vec<char>,
    width: u16,
    height: u16,
}

/// Vector match condition
fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

impl FrameBuffer {
    /// Creates an empty framebuffer with specified dimensions. Usually it is created at the beginning of the program. But needs to recreate if terminal window is resized
    pub fn create(width: u16, height: u16) -> Self {
        let frame = vec![' '; (width * height) as usize];
        let current_frame = vec![' '; (width * height) as usize];

        FrameBuffer {
            frame,
            current_frame,
            width,
            height,
        }
    }

    /// Draws a frame in console. Recommended disable cursor. Because it will interfere with enjoyment of the drawing
    pub fn push_fb(&mut self, stdout: &mut std::io::Stdout) {
        if !do_vecs_match(&self.frame, &self.current_frame) {
            mem::swap(&mut self.frame, &mut self.current_frame);

            for x in 0..self.width {
                for y in 0..self.height {
                    if self.frame[(y * self.width + x) as usize]
                        != self.current_frame[(y * self.width + x) as usize]
                    {
                        &stdout.execute(cursor::MoveTo(x, y));
                        &stdout.execute(Print(self.frame[(y * self.width + x) as usize]));
                    }
                }
            }
        }
    }

    /// Get symbol in framebuffer in specified coordinates
    pub fn get(&mut self, x: u16, y: u16) -> char {
        self.frame[(y * self.width + x) as usize]
    }

    /// Set string in framebuffer
    pub fn set(&mut self, text: &str, x: u16, y: u16) {
        for (i, v) in text.chars().enumerate() {
            if (y * self.width + (x + i as u16)) < self.current_frame.len() as u16 {
                self.current_frame[(y * self.width + (x + i as u16)) as usize] = v;
            }
        }
    }
}
