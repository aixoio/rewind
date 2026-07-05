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

    println!("Stash");
    println!("{sub_command:?}");

    ExitCode::SUCCESS
}
