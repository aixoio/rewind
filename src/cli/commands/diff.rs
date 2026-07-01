use crate::git::repo::{self, is_git_repo};

use owo_colors::OwoColorize;

pub fn run() {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    repo::diff().expect("failed to diff");
}
