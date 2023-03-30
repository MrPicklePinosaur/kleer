use std::{
    io::{BufWriter, Stdout, Write},
    thread,
    time::Duration,
};

use clap::ValueEnum;
use crossterm::{
    self,
    cursor::{self, MoveTo},
    style::Print,
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};

use crate::ClearMode;

/// Clear entire screen like standard `clear` command
pub fn basic(out: &mut BufWriter<Stdout>) -> anyhow::Result<()> {
    out.queue(cursor::MoveTo(0, 0))?;
    out.queue(Clear(ClearType::All))?;
    out.flush()?;
    Ok(())
}

/// The direction the wipe animation will play in
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum WipeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for WipeDirection {
    fn default() -> Self {
        Self::Down
    }
}

/// Wipe animation
pub fn wipe(
    out: &mut BufWriter<Stdout>,
    dir: WipeDirection,
    speed_scale: f32,
) -> anyhow::Result<()> {
    let (cols, rows) = terminal::size()?;

    let speed = (10 as f32 * 1.0 / speed_scale).round() as u64;

    match dir {
        WipeDirection::Up => {
            for i in 0..=rows {
                out.queue(cursor::MoveTo(0, rows - i))?;
                out.queue(Clear(ClearType::CurrentLine))?;
                out.flush()?;
                thread::sleep(Duration::from_millis(speed));
            }
        },
        WipeDirection::Down => {
            for i in 0..=rows {
                out.queue(cursor::MoveTo(0, i))?;
                out.queue(Clear(ClearType::CurrentLine))?;
                out.flush()?;
                thread::sleep(Duration::from_millis(speed));
            }
        },
        WipeDirection::Left => {
            for i in 0..=cols {
                for j in 0..=rows {
                    out.queue(MoveTo(cols - i, j))?;
                    out.queue(Print(" "))?;
                }
                out.flush()?;
                thread::sleep(Duration::from_millis(speed));
            }
        },
        WipeDirection::Right => {
            for i in 0..=cols {
                for j in 0..=rows {
                    out.queue(MoveTo(i, j))?;
                    out.queue(Print(" "))?;
                }
                out.flush()?;
                thread::sleep(Duration::from_millis(speed));
            }
        },
    }

    // reset cursor back to top
    out.queue(cursor::MoveTo(0, 0))?;
    out.flush()?;

    Ok(())
}
