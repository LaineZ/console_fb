//! # Console (Crossterm) double-buffering library
//! ![Scheme of working](https://i.imgur.com/K6dZxZy.png)

use crossterm::{
    cursor,
    style::{Color, Colors, Print, SetColors},
    ExecutableCommand,
};
use std::{io::Write, mem};

#[derive(Clone)]
/// The structure of framebuffer
pub struct FrameBuffer {
    current_frame: Vec<char>,
    current_frame_color: Vec<Colors>,
    frame: Vec<char>,
    width: u16,
    height: u16,
}

impl FrameBuffer {
    /// Creates an empty framebuffer with specified dimensions. Usually it is created at the beginning of the program. But needs to recreate if terminal window is resized
    pub fn create(width: u16, height: u16) -> Self {
        let frame = vec![' '; (width * height) as usize];
        let current_frame = vec![' '; (width * height) as usize];
        let current_frame_color =
            vec![Colors::new(Color::Reset, Color::Reset); (width * height) as usize];

        FrameBuffer {
            frame,
            current_frame,
            current_frame_color,
            width,
            height,
        }
    }

    /// Draws a frame in console. Recommended disable cursor. Because it will interfere with enjoyment of the drawing
    pub fn push_fb(&mut self, stdout: &mut std::io::Stdout, use_colors: bool) {
        if &self.frame != &self.current_frame {
            mem::swap(&mut self.frame, &mut self.current_frame);

            let mut last_color = Colors::new(Color::Reset, Color::Reset);

            for x in 0..self.width {
                for y in 0..self.height {
                    if self.frame[(y * self.width + x) as usize]
                        != self.current_frame[(y * self.width + x) as usize]
                    {
                        &stdout.execute(cursor::MoveTo(x, y));

                        let clr = self.current_frame_color[(y * self.width + x) as usize];
                        if last_color != clr && use_colors {
                            &stdout.execute(SetColors(clr));
                            last_color = clr;
                        }

                        &stdout.execute(Print(self.frame[(y * self.width + x) as usize]));
                    }
                }
            }
            &stdout.flush();
        }
    }

    /// Get symbol in framebuffer in specified coordinates
    pub fn get(&mut self, x: u16, y: u16) -> char {
        self.frame[(y * self.width + x) as usize]
    }

    /// Get foreground and background colors in framebuffer at specified coordinates
    pub fn get_color(&mut self, x: u16, y: u16) -> Colors {
        self.current_frame_color[(y * self.width + x) as usize]
    }

    /// Color-only rectangle fill with specified background and foreground colors
    pub fn set_color(&mut self, x: u16, y: u16, w: u16, h: u16, colors: Colors) {
        for xf in x..x + w {
            for yf in y..y + h {
                self.current_frame_color[(yf * self.width + xf) as usize] = colors;
            }
        }
    }

    /// Set string or &str in framebuffer in specified coordinates
    pub fn set<S: Into<String>>(&mut self, text: S, x: u16, y: u16) {
        for (i, v) in text.into().chars().enumerate() {
            if (y * self.width + (x + i as u16)) < self.current_frame.len() as u16 {
                self.current_frame[(y * self.width + (x + i as u16)) as usize] = v;
            }
        }
    }

    /// Set colored (background and foreground) string or &str in framebuffer in specified coordinates
    pub fn set_with_style<S: Into<String>>(&mut self, text: S, x: u16, y: u16, color: Colors) {
        for (i, v) in text.into().chars().enumerate() {
            if (y * self.width + (x + i as u16)) < self.current_frame.len() as u16 {
                self.current_frame_color[(y * self.width + (x + i as u16)) as usize] = color;
                self.current_frame[(y * self.width + (x + i as u16)) as usize] = v;
            }
        }
    }
}
