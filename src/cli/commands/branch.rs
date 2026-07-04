use std::process::ExitCode;

use inquire::Confirm;
use owo_colors::OwoColorize;

use crate::{
    check_for_git_repo,
    git::{
        branch::{
            all_branches, branch_exists, create_branch, current_branch, delete_branch,
            delete_remote_branch, parse_branch_names,
        },
        repo,
    },
    handle_error, print_error,
};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum BranchCommands {
    Delete {
        name: String,
        #[arg(long)]
        remote: bool,
    },
}

pub fn run(name: Option<String>, sub_command: Option<BranchCommands>) -> ExitCode {
    check_for_git_repo!();

    if name.is_none() && sub_command.is_none() {
        return branch_list();
    }

    if let Some(name) = name {
        return branch_create_or_switch(name);
    }

    let Some(sub_command) = sub_command else {
        print_error!("invalid args");
        return ExitCode::FAILURE;
    };

    match sub_command {
        BranchCommands::Delete { name, remote } => branch_delete(name, remote),
    }
}

fn branch_delete(name: String, remote: bool) -> ExitCode {
    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".bright_red().bold());
        return ExitCode::FAILURE;
    };

    if name.trim() == current_branch.trim() {
        eprintln!("{}", "cannot delete current branch".bright_red().bold());
        return ExitCode::FAILURE;
    }

    if !branch_exists(&name) {
        eprintln!(
            "{}",
            "cannot delete branch as it does not exist"
                .bright_red()
                .bold()
        );
        return ExitCode::FAILURE;
    }

    let message = if remote {
        format!("You are sure you want to delete the remote branch {name}?")
    } else {
        format!("You are sure you want to delete the branch {name}?")
    };
    let check = match Confirm::new(&message).with_default(false).prompt() {
        Ok(check) => check,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red().bold(), err.bold());
            return ExitCode::FAILURE;
        }
    };
    if !check {
        return ExitCode::FAILURE;
    }

    println!("{} {}", "Deleting branch".green(), name);

    if remote {
        handle_error!(delete_remote_branch(&name));
    } else {
        handle_error!(delete_branch(&name));
    }

    println!("{} {}", "Deleted branch:".green(), name.trim().bold());

    ExitCode::SUCCESS
}

fn branch_list() -> ExitCode {
    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".bright_red().bold());
        return ExitCode::FAILURE;
    };

    println!("On branch {}", current_branch.bold());
    println!();

    let Ok(branches) = all_branches() else {
        eprintln!("{}", "cannot get all branches".bright_red().bold());
        return ExitCode::FAILURE;
    };
    let branches = parse_branch_names(&branches);

    println!("{}", "Branches:".blue().bold());
    for branch in branches {
        if branch == current_branch {
            println!("   * {}", branch.cyan().bold());
            continue;
        }

        println!("     {}", branch);
    }

    ExitCode::SUCCESS
}

fn branch_create_or_switch(name: String) -> ExitCode {
    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".bright_red().bold());
        return ExitCode::FAILURE;
    };

    println!("On branch {}", current_branch.bold());
    println!();

    if !branch_exists(name.trim()) {
        handle_error!(create_branch(name.trim()));

        println!("{} {}", "Created new branch:".green(), name.trim().bold());
    }

    handle_error!(repo::checkout(name.trim()));

    println!("{} {}", "Switched to branch:".green(), name.trim().bold());

    ExitCode::SUCCESS
}
