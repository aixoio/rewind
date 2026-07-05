use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::cli::commands::{
    add,
    branch::{self, BranchCommands},
    checkout, commit, diff, init, log, merge, pull, push, reset, revert,
    stash::{self, StashCommands},
    status,
    tag::{self, TagCommand},
};

mod commands;

#[derive(Parser)]
#[command(version, name = "rewind")]
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
    Diff {
        #[arg(short, long, conflicts_with = "args")]
        stat: bool,
        #[arg(trailing_var_arg = true, conflicts_with = "stat")]
        args: Option<Vec<String>>,
    },
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
    #[command(alias = "co")]
    Checkout {
        target: String,
    },
    Reset,
    #[command(alias = "t")]
    Tag {
        #[command(subcommand)]
        command: TagCommand,
    },
    Revert {
        hash: String,
    },
    Stash {
        #[command(subcommand)]
        sub_command: Option<StashCommands>,
    },
}

impl Commands {
    pub fn run(self) -> ExitCode {
        match self {
            Commands::Status => status::run(),
            Commands::Add { paths } => add::run(paths),
            Commands::Commit { message } => commit::run(message),
            Commands::Init => init::run(),
            Commands::Push => push::run(),
            Commands::Pull => pull::run(),
            Commands::Log { limit, show_all } => log::run(limit, show_all),
            Commands::Diff { stat, args } => diff::run(stat, args),
            Commands::Branch { name, sub_command } => branch::run(name, sub_command),
            Commands::Merge { source, target } => merge::run(source, target),
            Commands::Checkout { target } => checkout::run(target),
            Commands::Reset => reset::run(),
            Commands::Tag { command } => tag::run(command),
            Commands::Revert { hash } => revert::run(hash),
            Commands::Stash { sub_command } => stash::run(sub_command),
        }
    }
}
