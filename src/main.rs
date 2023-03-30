mod clear_screen;
mod validation;

use std::io::{stdout, BufWriter, Write};

use clap::{Parser, Subcommand, ValueEnum};
use clear_screen::WipeDirection;
use crossterm::{
    cursor,
    terminal::{disable_raw_mode, enable_raw_mode},
    QueueableCommand,
};

#[derive(Subcommand, Copy, Clone)]
pub enum ClearMode {
    Basic,
    Wipe {
        #[arg(short, long)]
        dir: Option<WipeDirection>,
        #[arg(short, long, value_parser = validation::greater_than_zero)]
        speed_scale: Option<f32>,
    },
    Dissolve,
}

impl Default for ClearMode {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Parser)]
#[command(name = "kuuhaku")]
#[command(bin_name = "kuuhaku")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    clear_mode: ClearMode,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Ensure we clean up in case of unexpected exits
    struct CleanUp;
    impl Drop for CleanUp {
        fn drop(&mut self) {
            let _ = disable_raw_mode();
        }
    }
    let _cleanup = CleanUp;

    enable_raw_mode()?;

    let mut out = BufWriter::new(stdout());

    out.queue(cursor::Hide)?;

    match cli.clear_mode {
        ClearMode::Basic => clear_screen::basic(&mut out)?,
        ClearMode::Wipe { dir, speed_scale } => clear_screen::wipe(
            &mut out,
            dir.unwrap_or_default(),
            speed_scale.unwrap_or(1.0),
        )?,
        ClearMode::Dissolve => clear_screen::dissolve(&mut out)?,
    };

    out.queue(cursor::Show)?;

    Ok(())
}
