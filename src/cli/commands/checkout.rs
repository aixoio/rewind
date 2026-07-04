use std::process::ExitCode;

use crate::{check_for_git_repo, git::repo, handle_error};

use owo_colors::OwoColorize;

pub fn run(target: String) -> ExitCode {
    check_for_git_repo!();

    println!("{} {}", "Checking out".cyan().bold(), target.blue());

    handle_error!(repo::checkout(&target));

    println!("{}", "Checkout successful!".bright_green().bold());

    ExitCode::SUCCESS
}
