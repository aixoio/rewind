use std::process::ExitCode;

use crate::check_for_git_repo;

pub fn run(hash: String) -> ExitCode {
    check_for_git_repo!();

    println!("revertting {hash}");

    ExitCode::SUCCESS
}
