#![warn(unreachable_code)]
/// A basic-benchmark example

use console_fb::FrameBuffer;
use crossterm::{cursor, style::Print, terminal, ErrorKind, ExecutableCommand};
use std::io::{Write, stdout};
use std::time::Instant;

fn main() -> Result<(), ErrorKind> {
    let mut stdout = stdout();
    let mut fb = FrameBuffer::create(80, 25);

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let start_test1 = Instant::now();
    for i in 0..9 {
        &stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        for y in 0..24 {
            for x in 0..80 {
                if x % 2 == 0 {
                    &stdout.execute(cursor::MoveTo(x, y));
                    &stdout.execute(Print(i));
                }
            }
            &stdout.flush();
        }
    }
    let test1_time = start_test1.elapsed();

    let start_test2 = Instant::now();
    for i in 0..9 {
        for y in 0..24 {
            for x in 0..80 {
                if x % 2 == 0 {
                    fb.set(i.to_string(), x, y);
                }
            }
        }
        fb.push_fb(&mut stdout);
    }
    let test2_time = start_test2.elapsed();
    &stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    println!(
        "test1 (no double): {} ms test2 (double): {} ms",
        test1_time.as_millis(),
        test2_time.as_millis()
    );
    Ok(())
}
