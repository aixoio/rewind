use crate::git::branch::branch_exists;

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

    println!("both branchs exist");
}
