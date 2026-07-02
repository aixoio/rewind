use clap::Subcommand;

use owo_colors::OwoColorize;

use crate::git::{
    repo::is_git_repo,
    tag::{fetch_all_tags, parse_git_tags},
};

#[derive(Subcommand, Debug)]
pub enum TagCommand {
    List,
    Create {
        name: String,
        #[arg(short, long)]
        message: Option<String>,
    },
}

pub fn run(command: TagCommand) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    match command {
        TagCommand::List => list_tags(),
        TagCommand::Create { name, message } => create_tag(name, message),
    }
}

fn create_tag(name: String, message: Option<String>) {
    println!("{name}{message:?}");
}

fn list_tags() {
    println!("{}", "Tags:".blue().bold());

    let stdout = fetch_all_tags().expect("failed to fetch all tags");
    let tags = parse_git_tags(&stdout);

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
