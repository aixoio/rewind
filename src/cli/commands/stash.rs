use std::process::ExitCode;

use crate::check_for_git_repo;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum StashCommands {
    List,
    Pop,
}

pub fn run(sub_command: Option<StashCommands>) -> ExitCode {
    check_for_git_repo!();

    match sub_command {
        None => stash_create(),
        Some(StashCommands::List) => stash_list(),
        Some(StashCommands::Pop) => stash_pop(),
    }
}

fn stash_create() -> ExitCode {
    println!("stash create");
    ExitCode::SUCCESS
}

fn stash_list() -> ExitCode {
    println!("stash list");
    ExitCode::SUCCESS
}

fn stash_pop() -> ExitCode {
    println!("stash pop");
    ExitCode::SUCCESS
}
