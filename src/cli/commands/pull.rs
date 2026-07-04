use std::process::ExitCode;

use crate::{check_for_git_repo, git::remote, handle_error};

use owo_colors::OwoColorize;

pub fn run() -> ExitCode {
    check_for_git_repo!();

    match remote::upstream() {
        Some(_) => {
            println!("{}", "Pulling changes from remote...".blue());

            handle_error!(remote::pull());

            println!("{}", "Pull completed!".green().bold());
        }
        None => {
            println!("{}", "No upstream branch configured".italic());
            println!("{}", "Setting upstream to origin/HEAD".blue());

            handle_error!(remote::pull_set_upstream());

            println!("{}", "Pull completed!".green().bold());
            println!("{}", "Upstream set and changes pulled".bright_black());
        }
    };

    ExitCode::SUCCESS
}
