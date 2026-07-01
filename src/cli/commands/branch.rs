use inquire::Confirm;
use owo_colors::OwoColorize;

use crate::git::{
    branch::{
        all_branches, branch_exists, create_branch, current_branch, delete_branch,
        delete_remote_branch,
    },
    repo::{self, is_git_repo},
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
        BranchCommands::Delete { name, remote } => branch_delete(name, remote),
    }
}

fn branch_delete(name: String, remote: bool) {
    let Ok(current_branch) = current_branch() else {
        eprintln!("{}", "cannot get current branch".bright_red().bold());
        return;
    };

    if name.trim() == current_branch.trim() {
        eprintln!("{}", "cannot delete current branch".bright_red().bold());
        return;
    }

    if !branch_exists(&name) {
        eprintln!(
            "{}",
            "cannot delete branch as it does not exist"
                .bright_red()
                .bold()
        );
        return;
    }

    let message = if remote {
        format!("You are sure you want to delete the remote branch {name}?")
    } else {
        format!("You are sure you want to delete the branch {name}?")
    };
    let check = Confirm::new(&message).with_default(false).prompt().unwrap();
    if !check {
        return;
    }

    println!("{} {}", "Deleting branch".green(), name);

    if remote {
        delete_remote_branch(&name).expect("cannot delete remote branch");
    } else {
        delete_branch(&name).expect("cannot delete branch");
    }

    println!("{} {}", "Deleted branch:".green(), name.trim().bold());
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
        create_branch(name.trim()).expect("failed to create branch");
        println!("{} {}", "Created new branch:".green(), name.trim().bold());
    }

    repo::checkout(name.trim()).expect("cannnot checkout target");

    println!("{} {}", "Switched to branch:".green(), name.trim().bold());
}
