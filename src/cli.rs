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

/// Rewind is a small fast git-wrapper
///
/// Use this program to manage your Git repository
/// with ease using a modern CLI application built
/// with performance and usability in mind.
#[derive(Parser)]
#[command(version, name = "rewind")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// push changes to origin
    #[command(visible_alias = "p")]
    Push,
    /// pull changes from origin
    #[command(visible_alias = "pu")]
    Pull,
    /// prints git status
    #[command(visible_alias = "s")]
    Status,
    /// stage files to git
    #[command(visible_alias = "ad")]
    Add {
        paths: Option<Vec<String>>,
    },
    /// commit files to git and auto stage
    #[command(visible_alias = "c")]
    Commit {
        #[arg(short, long)]
        message: Option<String>,
    },
    /// run `git init`
    Init,
    /// display commit history
    #[command(visible_alias = "l")]
    Log {
        /// limit results (default=10)
        #[arg(short, long)]
        limit: Option<usize>,
        /// print all commits
        #[arg(long)]
        show_all: bool,
    },
    /// display `git diff`
    #[command(visible_alias = "d")]
    Diff {
        /// display `git diff --stat`, not compatable with args
        #[arg(short, long, conflicts_with = "args")]
        stat: bool,
        /// args passed to `git diff`, not compatable with stat
        #[arg(trailing_var_arg = true, conflicts_with = "stat")]
        args: Option<Vec<String>>,
    },
    /// manage git branches
    #[command(visible_alias = "b")]
    Branch {
        /// create a new branch with the name
        name: Option<String>,
        #[command(subcommand)]
        sub_command: Option<BranchCommands>,
    },
    /// merge 2 branch
    #[command(visible_alias = "m")]
    Merge {
        /// source branch
        source: String,
        /// target branch
        target: String,
    },
    /// checkout a branch or commit
    #[command(visible_alias = "co")]
    Checkout {
        /// target branch or commit
        target: String,
    },
    /// discard all uncommited changes
    Reset,
    /// manage git tags
    #[command(visible_alias = "t")]
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
