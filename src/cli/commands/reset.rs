use crate::{check_for_git_repo, git::repo, handle_error, return_error};

use inquire::Confirm;
use owo_colors::OwoColorize;
use std::process::ExitCode;

pub fn run() -> ExitCode {
    check_for_git_repo!();

    let prompt = match Confirm::new("Reset Repository")
        .with_default(false)
        .with_help_message("This will discard all uncommitted changes. Continue?")
        .prompt()
    {
        Ok(prompt) => prompt,
        Err(err) => {
            return_error!(err);
        }
    };
    if !prompt {
        return ExitCode::SUCCESS;
    }

    println!("{}", "Staging all files...".green());

    handle_error!(repo::add_paths(&["."]));

    println!("{}", "Performing hard reset...".green());

    handle_error!(repo::reset());

    println!();

    println!("{}", "Repository successfully reset".bright_green().bold());
    println!(
        "{}",
        "All uncommitted changes have been discarded".bright_black()
    );

    ExitCode::SUCCESS
}
