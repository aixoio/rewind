use owo_colors::OwoColorize;

use crate::git::branch::current_branch;

pub fn run() {
    println!("{}", "Branches".cyan());

    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".bright_red().bold());
        return;
    };

    println!("On branch {}", current_branch.bold());
}
