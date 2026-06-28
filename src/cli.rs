use clap::{Parser, Subcommand};

use crate::cli::commands::{add, status};

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
    #[command(alias = "ad")]
    Add { paths: Option<Vec<String>> },
}

impl Commands {
    pub fn run(self) {
        match self {
            Commands::Status => status::run(),
            Commands::Add { paths } => add::run(paths),
        }
    }
}
