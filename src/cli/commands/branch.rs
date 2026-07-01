use owo_colors::OwoColorize;

use crate::git::{
    branch::{all_branches, branch_exists, current_branch},
    repo::{self, is_git_repo},
};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum BranchCommands {
    Delete { name: String },
}

pub fn run(name: Option<String>, sub_command: Option<BranchCommands>) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    if name.is_none() && sub_command.is_none() {
        branch_list();
        return;
    }

    if let Some(name) = name {
        branch_create_or_switch(name);
        return;
    }

    let Some(sub_command) = sub_command else {
        eprintln!("{}", "invalid args".bold().bright_red());
        return;
    };

    match sub_command {
        BranchCommands::Delete { name } => println!("deleteing branch {name}"),
    }
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
            println!("   * {}", branch.cyan().bold());
            continue;
        }

        println!("     {}", branch);
    }
}

fn branch_create_or_switch(name: String) {
    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".bright_red().bold());
        return;
    };

    println!("On branch {}", current_branch.bold());
    println!();

    if !branch_exists(name.trim()) {
        println!("creating {name}");
    }

    repo::checkout(name.trim()).expect("cannnot checkout target");

    println!("{} {}", "Switched to branch:".green(), name.trim().bold());
}
