use crate::git::repo::{self, is_git_repo};

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

    println!("{}", "Staging all files...".green());
    repo::add_paths(&[".".to_string()]).unwrap();

    println!("{}", "Performing hard reset...".green());
    repo::reset().unwrap();

    println!();

    println!("{}", "Repository successfully reset".bright_green().bold());
    println!(
        "{}",
        "All uncommitted changes have been discarded".bright_black()
    );
}
