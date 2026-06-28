use crate::git::{repo::is_git_repo, status::fetch_status};
use owo_colors::OwoColorize;

pub fn run() {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    let status = fetch_status().expect("failed to fetch git status");

    if status.total_files() == 0 {
        println!("{}", "No changes to commit.".bright_black());
        return;
    }

    if status.staged().len() != 0 {
        println!("{}", "Staged:".bold());

        for file in status.staged() {
            println!("     {}", file);
        }
    }

    if status.unstaged().len() != 0 {
        println!("{}", "Unstaged:".bold());

        for file in status.unstaged() {
            println!("     {}", file);
        }
    }

    if status.untracked().len() != 0 {
        println!("{}", "Untracked:".bold());

        for file in status.untracked() {
            println!("     {}", file);
        }
    }
}
