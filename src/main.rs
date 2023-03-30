mod clear_screen;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ClearMode {
    Basic,
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
    #[arg(long, short)]
    clear_mode: ClearMode,
}

fn main() {
    let cli = Cli::parse();

    match cli.clear_mode {
        ClearMode::Basic => clear_screen::basic(),
    }
}
