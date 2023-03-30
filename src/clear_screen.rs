use std::{
    io::{BufWriter, Stdout, Write},
    ops::{Div, Range, RangeInclusive},
    thread,
    time::Duration,
};

use clap::ValueEnum;
use crossterm::{
    self,
    cursor::{self, MoveTo, MoveToColumn, MoveToRow},
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

/// Randomly delete characters in terminal
pub fn dissolve(out: &mut BufWriter<Stdout>) -> anyhow::Result<()> {
    let mut pos = vec![];

    let (cols, rows) = terminal::size()?;

    // TODO there should be more elegant way to do this
    for i in 0..=cols {
        for j in 0..=rows {
            pos.push((i, j));
        }
    }

    // TODO shuffle pos list

    for (i, j) in pos.iter() {
        out.queue(MoveTo(*i, *j))?;
        out.queue(Print(" "))?;
        out.flush()?;

        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

pub fn spiral(out: &mut BufWriter<Stdout>) -> anyhow::Result<()> {
    let (cols, rows) = terminal::size()?;

    let mut h_step = 0;
    let mut v_step = 0;

    let delay = 1;

    while h_step < cols.div(2) && v_step < rows.div(2) {
        // TODO need reverse range
        clear_col(out, h_step, v_step..=rows - v_step, delay)?;
        clear_row(out, v_step, h_step..=cols - h_step, delay)?;
        clear_col(out, cols - h_step, rows - v_step..=v_step, delay)?;
        clear_row(out, rows - v_step, cols - h_step..=h_step, delay)?;

        h_step += 1;
        v_step += 1;
    }

    Ok(())
}

fn clear_col(
    out: &mut BufWriter<Stdout>,
    col: u16,
    range: RangeInclusive<u16>,
    delay: u64,
) -> anyhow::Result<()> {
    for row in range {
        out.queue(MoveTo(col, row))?;
        out.queue(Print(" "))?;
        out.flush()?;
        thread::sleep(Duration::from_millis(delay));
    }

    Ok(())
}

fn clear_row(
    out: &mut BufWriter<Stdout>,
    row: u16,
    range: RangeInclusive<u16>,
    delay: u64,
) -> anyhow::Result<()> {
    for col in range {
        out.queue(MoveTo(col, row))?;
        out.queue(Print(" "))?;
        out.flush()?;
        thread::sleep(Duration::from_millis(delay));
    }

    Ok(())
}
