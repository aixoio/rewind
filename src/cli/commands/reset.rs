use crate::git::repo::is_git_repo;

use inquire::Confirm;
use owo_colors::OwoColorize;

pub fn run() {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    let prompt = Confirm::new("Reset Repository")
        .with_default(false)
        .with_help_message("This will discard all uncommitted changes. Continue?")
        .prompt()
        .unwrap();

    if !prompt {
        return;
    }

    println!("resetting...");
}
