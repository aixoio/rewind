use std::process::ExitCode;

use crate::{
    check_for_git_repo,
    git::stash::{fetch_stashes, parse_stashes},
    match_error, return_error,
};

use clap::Subcommand;
use inquire::Confirm;
use owo_colors::OwoColorize;

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
    println!("{}", "Stash list:".cyan().bold());

    let stashes = match_error!(fetch_stashes());
    let stashes = parse_stashes(&stashes);

    let mut is_empty = true;

    for stash in stashes {
        is_empty = false;

        println!(
            "     {} {} {}",
            stash.id().bright_black(),
            stash.created().blue(),
            stash.subject().bold()
        );
    }

    if is_empty {
        println!("     {}", "No stashes found".bright_black());
    }

    ExitCode::SUCCESS
}

fn stash_pop() -> ExitCode {
    println!("{}", "Stash pop".cyan().bold());
    println!();

    let stashes = match_error!(fetch_stashes());

    let stash = match parse_stashes(&stashes).next() {
        Some(stash) => stash,
        None => {
            return_error!("no stashes to pop");
        }
    };

    println!("{}", "Latest stash:".bright_blue().bold());
    println!(
        "     {} {} {}",
        stash.id().bright_black(),
        stash.created().blue(),
        stash.subject().bold()
    );
    println!();

    let check = match_error!(
        Confirm::new("Apply this stash?")
            .with_help_message("This may cause conflicts if files have changed")
            .with_default(false)
            .prompt()
    );
    if !check {
        return ExitCode::SUCCESS;
    }

    ExitCode::SUCCESS
}
