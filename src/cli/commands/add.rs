use std::process::ExitCode;

use crate::{check_for_git_repo, git::repo::add_paths, handle_error};

use owo_colors::OwoColorize;

pub fn run(paths: Option<Vec<String>>) -> ExitCode {
    check_for_git_repo!();

    let paths = paths.unwrap_or_else(|| vec![".".to_string()]);

    handle_error!(add_paths(&paths));

    println!("{}", "Files staged successfully!".green().bold());
    println!("{}", "Ready for commit.".bright_black());

    ExitCode::SUCCESS
}
