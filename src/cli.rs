use clap::{Parser, Subcommand};

use crate::cli::commands::{
    add,
    branch::{self, BranchCommands},
    commit, diff, init, log, merge, pull, push, status,
};

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
    #[command(alias = "pu")]
    Pull,
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
    #[command(alias = "l")]
    Log {
        #[arg(short, long)]
        limit: Option<usize>,
        #[arg(long)]
        show_all: bool,
    },
    #[command(alias = "d")]
    Diff,
    #[command(alias = "b")]
    Branch {
        name: Option<String>,
        #[command(subcommand)]
        sub_command: Option<BranchCommands>,
    },
    #[command(alias = "m")]
    Merge {
        source: String,
        target: String,
    },
}

impl Commands {
    pub fn run(self) {
        match self {
            Commands::Status => status::run(),
            Commands::Add { paths } => add::run(paths),
            Commands::Commit { message } => commit::run(message),
            Commands::Init => init::run(),
            Commands::Push => push::run(),
            Commands::Pull => pull::run(),
            Commands::Log { limit, show_all } => log::run(limit, show_all),
            Commands::Diff => diff::run(),
            Commands::Branch { name, sub_command } => branch::run(name, sub_command),
            Commands::Merge { source, target } => merge::run(source, target),
        }
    }
}
