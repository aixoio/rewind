use clap::Parser;

use crate::cli::commands::Commands;

mod commands;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
