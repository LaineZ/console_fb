use console_fb::FrameBuffer;
use std::io::stdout;
use crossterm::{ExecutableCommand, terminal};

fn main() {
    let mut stdout = stdout();
    let mut fb = FrameBuffer::create(80, 25);

    stdout.execute(terminal::Clear(terminal::ClearType::All));

    loop {
        for y in 0..24 {
            for x in 0..80 {
                if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
                    fb.set("+", x, y);
                }
            }
        }

        fb.push_fb(&mut stdout);
    }
}
