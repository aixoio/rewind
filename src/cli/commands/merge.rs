use crate::git::{
    branch::{branch_exists, current_branch},
    repo::{checkout, merge},
};

use owo_colors::OwoColorize;

pub fn run(source: String, target: String) {
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
        checkout(target.trim()).expect("cannot switch to target branch");
    }

    println!(
        "{} {} {} {}",
        "Merging".blue(),
        source.cyan(),
        "into".blue(),
        target.cyan()
    );

    merge(source.trim()).expect("cannnot merge branchs");

    println!(
        "{} {} {} {}",
        "Merged".green(),
        source.bright_green().bold(),
        "->".green(),
        target.bright_green().bold()
    );
}
