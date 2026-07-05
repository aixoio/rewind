use std::process::ExitCode;

use crate::check_for_git_repo;

pub fn run() -> ExitCode {
    check_for_git_repo!();

    println!("Stash");

    ExitCode::SUCCESS
}
