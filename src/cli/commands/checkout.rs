use crate::git::repo::{self, is_git_repo};

use owo_colors::OwoColorize;

pub fn run(target: String) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    println!("{} {}", "Checking out".cyan().bold(), target.blue());

    repo::checkout(&target).expect("cannot checkout");

    println!("{}", "Checkout successful!".bright_green().bold());
}
