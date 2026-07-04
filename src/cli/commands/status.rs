use crate::{
    check_for_git_repo,
    git::{
        branch::current_branch,
        status::{fetch_status, parse_status},
    },
    return_error,
};
use owo_colors::OwoColorize;
use std::process::ExitCode;

pub fn run() -> ExitCode {
    check_for_git_repo!();

    let branch = match current_branch() {
        Ok(branch) => branch,
        Err(err) => {
            return_error!(err);
        }
    };

    println!("On branch {}", branch.bold());
    println!();

    let status = match fetch_status() {
        Ok(status) => status,
        Err(err) => {
            return_error!(err);
        }
    };
    let status = parse_status(&status);

    if status.total_files() == 0 {
        println!("{}", "No changes to commit.".bright_black());
        return ExitCode::SUCCESS;
    }

    if !status.staged().is_empty() {
        println!("{}:", "Staged".green().bold());

        for file in status.staged() {
            println!("     {}", file);
        }
    }

    if !status.unstaged().is_empty() {
        println!("{}:", "Unstaged".blue().bold());

        for file in status.unstaged() {
            println!("     {}", file);
        }
    }

    if !status.untracked().is_empty() {
        println!("{}:", "Untracked".cyan().bold());

        for file in status.untracked() {
            println!("     {}", file);
        }
    }

    ExitCode::SUCCESS
}
