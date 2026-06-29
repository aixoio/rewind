use crate::git::{remote, repo::is_git_repo};

use owo_colors::OwoColorize;

pub fn run() {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    match remote::upstream() {
        Some(_) => {
            println!("{}", "Pulling changes from remote...".blue());

            remote::pull().expect("failed to pull upstream");

            println!("{}", "Pull completed!".green().bold());
        }
        None => {
            println!("{}", "No upstream branch configured".italic());
            println!("{}", "Setting upstream to origin/HEAD".blue());

            remote::pull_set_upstream().expect("failed to set upstream");

            println!("{}", "Pull completed!".green().bold());
            println!("{}", "Upstream set and changes pulled".bright_black());
        }
    };
}
