use std::process::ExitCode;

use crate::{check_for_git_repo, git::repo, handle_error};

pub fn run() -> ExitCode {
    check_for_git_repo!();

    handle_error!(repo::diff());

    ExitCode::SUCCESS
}
