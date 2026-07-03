use clap::Subcommand;

use inquire::Confirm;
use owo_colors::OwoColorize;

use crate::{
    check_for_git_repo,
    git::tag::{
        self, create_annotated_tag, create_lightweight_tag, fetch_all_tags, parse_git_tags,
        push_all_tags,
    },
    handle_error,
};

#[derive(Subcommand, Debug)]
pub enum TagCommand {
    List,
    Create {
        name: String,
        #[arg(short, long)]
        message: Option<String>,
    },
    Delete {
        name: String,
    },
    Push,
}

pub fn run(command: TagCommand) {
    check_for_git_repo!();

    match command {
        TagCommand::List => list_tags(),
        TagCommand::Create { name, message } => create_tag(name, message),
        TagCommand::Delete { name } => delete_tag(name),
        TagCommand::Push => push_tags(),
    }
}

fn push_tags() {
    println!("{}", "Pushing all tags".green());

    handle_error!(push_all_tags());

    println!("{}", "All tags pushed!".bright_green(),);
}

fn delete_tag(name: String) {
    println!("{} {}", "Deleting tag:".green(), name.green().bold());

    let help_message = format!("Are you sure you want to delete tag {}?", name);
    let check = match Confirm::new("Confirm Tag Deletion")
        .with_help_message(&help_message)
        .with_default(false)
        .prompt()
    {
        Ok(check) => check,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red().bold(), err.bold());
            return;
        }
    };
    if !check {
        return;
    }

    handle_error!(tag::delete_tag(&name));

    println!("{}", "Deleted tag successfully!".bright_green(),);
    println!("{} {}", "Tag:".bright_black(), name.bold());
}

fn create_tag(name: String, message: Option<String>) {
    match message {
        Some(message) => {
            println!(
                "{} {}",
                "Creating annotated tag:".green(),
                name.green().bold()
            );

            handle_error!(create_annotated_tag(&name, &message));

            println!("{}", "Annotated tag created successfully!".bright_green(),);
            println!("{} {}", "Tag:".bright_black(), name.bold());
            println!("{} {}", "Message:".bright_black(), message);
        }
        None => {
            println!(
                "{} {}",
                "Creating lightweight tag:".green(),
                name.green().bold()
            );

            handle_error!(create_lightweight_tag(&name));

            println!("{}", "Lightweight tag created successfully!".bright_green(),);
            println!("{} {}", "Tag:".bright_black(), name.bold());
        }
    }
}

fn list_tags() {
    println!("{}", "Tags:".blue().bold());

    let stdout = match fetch_all_tags() {
        Ok(stdout) => stdout,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red().bold(), err.bold());
            return;
        }
    };
    let tags: Vec<_> = parse_git_tags(&stdout).collect();

    if tags.is_empty() {
        println!("     {}", "No tags found".bright_black());
    }

    for tag in tags {
        println!(
            "     {} ({})",
            tag.name().bold(),
            tag.relative_date().bright_black()
        );
        println!("        {}", tag.subject().cyan());
    }
}
