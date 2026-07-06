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
    handle_error, match_error, return_error,
};

use clap::Subcommand;

#[derive(Subcommand)]
pub enum BranchCommands {
    /// delete a branch
    Delete {
        /// branch name
        name: String,
        /// if it's remote
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
        return_error!("invalid args");
    };

    match sub_command {
        BranchCommands::Delete { name, remote } => branch_delete(name, remote),
    }
}

fn branch_delete(name: String, remote: bool) -> ExitCode {
    let Ok(current_branch) = current_branch() else {
        return_error!("cannot get current branch");
    };

    if name.trim() == current_branch.trim() {
        return_error!("cannot delete current branch");
    }

    if !branch_exists(&name) {
        return_error!(format!(
            "{}",
            "cannot delete branch as it does not exist"
                .bright_red()
                .bold()
        ));
    }

    let message = if remote {
        format!("You are sure you want to delete the remote branch {name}?")
    } else {
        format!("You are sure you want to delete the branch {name}?")
    };
    let check = match_error!(Confirm::new(&message).with_default(false).prompt());
    if !check {
        return ExitCode::SUCCESS;
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
        return_error!("cannot get current branch");
    };

    println!("On branch {}", current_branch.bold());
    println!();

    let Ok(branches) = all_branches() else {
        return_error!("cannot get all branches");
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
        return_error!("cannot get current branch");
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
