use crate::{check_for_git_repo, git::repo, handle_error};

use inquire::Confirm;
use owo_colors::OwoColorize;

pub fn run() {
    check_for_git_repo!();

    let prompt = Confirm::new("Reset Repository")
        .with_default(false)
        .with_help_message("This will discard all uncommitted changes. Continue?")
        .prompt()
        .unwrap();
    if !prompt {
        return;
    }

    println!("{}", "Staging all files...".green());

    handle_error!(repo::add_paths(&[".".to_string()]));

    println!("{}", "Performing hard reset...".green());

    handle_error!(repo::reset());

    println!();

    println!("{}", "Repository successfully reset".bright_green().bold());
    println!(
        "{}",
        "All uncommitted changes have been discarded".bright_black()
    );
}
