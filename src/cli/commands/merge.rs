use crate::{
    check_for_git_repo,
    git::{
        branch::{branch_exists, current_branch},
        repo::{checkout, merge},
    },
    handle_error,
};

use owo_colors::OwoColorize;

pub fn run(source: String, target: String) {
    check_for_git_repo!();

    if !branch_exists(&source) {
        eprintln!(
            "{} {} {}",
            "branch".red(),
            source.bright_red().bold(),
            "does not exist".red()
        );
        return;
    }

    if !branch_exists(&target) {
        eprintln!(
            "{} {} {}",
            "branch".red(),
            target.bright_red().bold(),
            "does not exist".red()
        );
        return;
    }

    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".red().bold());
        return;
    };

    if current_branch.trim() != target.trim() {
        println!("{} {}", "Switching to target branch".green(), target.cyan());

        handle_error!(checkout(target.trim()));
    }

    println!(
        "{} {} {} {}",
        "Merging".blue(),
        source.cyan(),
        "into".blue(),
        target.cyan()
    );

    handle_error!(merge(source.trim()));

    println!(
        "{} {} {} {}",
        "Merged".green(),
        source.bright_green().bold(),
        "->".green(),
        target.bright_green().bold()
    );
}
