use crate::{check_for_git_repo, git::remote, handle_error};

use owo_colors::OwoColorize;
use std::process::ExitCode;

pub fn run() -> ExitCode {
    check_for_git_repo!();

    match remote::upstream() {
        Some(_) => {
            println!("{}", "Pushing changes to remote...".blue());

            handle_error!(remote::push());

            println!("{}", "Push completed!".green().bold());
        }
        None => {
            println!("{}", "No upstream branch configured".italic());
            println!("{}", "Setting upstream to origin/HEAD".blue());

            handle_error!(remote::push_set_upstream());

            println!("{}", "Push completed!".green().bold());
            println!("{}", "Upstream set and changes pushed".bright_black());
        }
    };

    ExitCode::SUCCESS
}
