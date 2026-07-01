use owo_colors::OwoColorize;

use crate::git::{
    branch::{all_branches, current_branch},
    repo::is_git_repo,
};

pub fn run() {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    branch_list();
}

fn branch_list() {
    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".bright_red().bold());
        return;
    };

    println!("On branch {}", current_branch.bold());
    println!();

    let Ok(branches) = all_branches() else {
        eprintln!("{}", "cannot get all branches".bright_red().bold());
        return;
    };

    println!("{}", "Branches:".blue().bold());
    for branch in branches {
        if branch == current_branch {
            println!("     {}", branch.cyan().bold());
            continue;
        }

        println!("     {}", branch);
    }
}
