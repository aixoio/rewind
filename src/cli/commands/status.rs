use std::process;

use crate::git::{repo::is_git_repo, status::fetch_status};
use owo_colors::OwoColorize;

pub fn run() {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        process::exit(1);
    }

    let status = fetch_status().expect("failed to fetch git status");

    println!("{status:#?}");
}
