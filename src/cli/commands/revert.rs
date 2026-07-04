use std::process::ExitCode;

use crate::{
    check_for_git_repo,
    git::commits::{fetch_commit_info, parse_commit_info},
    return_error,
};
use inquire::Confirm;
use owo_colors::OwoColorize;

pub fn run(hash: String) -> ExitCode {
    check_for_git_repo!();

    let commit_info = match fetch_commit_info(&hash) {
        Ok(commit_info) => commit_info,
        Err(err) => {
            return_error!(err);
        }
    };
    let commit_info = match parse_commit_info(&commit_info) {
        Some(commit_info) => commit_info,
        None => {
            return_error!("cannot parse commit info");
        }
    };

    println!("{}", "Commit to revert:".blue().bold());
    println!(
        "   {} {}",
        "Author:".bright_blue().bold(),
        commit_info.author()
    );
    println!("   {} {}", "Date:".bright_blue().bold(), commit_info.date());
    println!("     {}", commit_info.hash().bright_black());
    println!("     {}", commit_info.message());

    println!();

    let check = match Confirm::new("Confirm revert").with_default(false).prompt() {
        Ok(check) => check,
        Err(err) => {
            return_error!(err);
        }
    };
    if !check {
        return ExitCode::SUCCESS;
    }
    println!();

    ExitCode::SUCCESS
}
