use clap::{Parser, Subcommand};

use crate::cli::commands::status;

mod commands;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "s")]
    Status,
}

impl Commands {
    pub fn run(self) {
        match self {
            Commands::Status => status::run(),
        }
    }
}
