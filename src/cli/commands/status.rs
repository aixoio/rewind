use crate::git::{
    repo::{current_branch, is_git_repo},
    status::fetch_status,
};
use owo_colors::OwoColorize;

pub fn run() {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    let branch = current_branch().expect("cannot get current branch");

    println!("On branch {}", branch.bold());
    println!();

    let status = fetch_status().expect("failed to fetch git status");

    if status.total_files() == 0 {
        println!("{}", "No changes to commit.".bright_black());
        return;
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
}
