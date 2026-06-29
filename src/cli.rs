use clap::{Parser, Subcommand};

use crate::cli::commands::{add, commit, init, push, status};

mod commands;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(alias = "p")]
    Push,
    #[command(alias = "s")]
    Status,
    #[command(alias = "ad")]
    Add {
        paths: Option<Vec<String>>,
    },
    #[command(alias = "c")]
    Commit {
        #[arg(short, long)]
        message: Option<String>,
    },
    Init,
}

impl Commands {
    pub fn run(self) {
        match self {
            Commands::Status => status::run(),
            Commands::Add { paths } => add::run(paths),
            Commands::Commit { message } => commit::run(message),
            Commands::Init => init::run(),
            Commands::Push => push::run(),
        }
    }
}
