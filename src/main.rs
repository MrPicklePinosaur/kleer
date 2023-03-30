mod clear_screen;

use std::io::{stdout, BufWriter, Write};

use clap::{Parser, Subcommand, ValueEnum};
use clear_screen::WipeDirection;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Subcommand, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClearMode {
    Basic,
    Wipe {
        #[arg(short, long)]
        dir: WipeDirection,
    },
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

    match cli.clear_mode {
        ClearMode::Basic => clear_screen::basic(&mut out)?,
        ClearMode::Wipe { dir } => clear_screen::wipe(&mut out, dir)?,
    };

    Ok(())
}
