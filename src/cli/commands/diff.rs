use std::process::ExitCode;

use crate::{check_for_git_repo, git::repo, handle_error};

pub fn run(stat: bool, args: Option<Vec<String>>) -> ExitCode {
    check_for_git_repo!();

    if stat {
        handle_error!(repo::diff_stat());
    } else {
        handle_error!(repo::diff(args.unwrap_or_default()));
    }

    ExitCode::SUCCESS
}
