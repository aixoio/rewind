use crate::git::{
    log::{fetch_log, fetch_log_with_limit},
    repo::{current_branch, is_git_repo},
    status::fetch_status,
};

use owo_colors::{OwoColorize, colors::xterm::BlazeOrange};

pub fn run(limit: Option<usize>, show_all: bool) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    let limit = limit.unwrap_or(10);

    let commits = if show_all {
        fetch_log()
    } else {
        fetch_log_with_limit(limit)
    };
    let commits = commits.expect("failed to fetch commits");

    println!("{}", "Git log".fg::<BlazeOrange>().bold());

    let branch = current_branch();

    if let Ok(branch) = branch {
        println!("On branch {}", branch.bold());
    }

    println!();

    for commit in commits.iter().rev() {
        let short_hash = &commit.hash()[..7];

        if commit.refs().is_empty() {
            println!("{} {}", short_hash.bright_black(), commit.subject().bold());
        } else {
            println!(
                "{} {} {}",
                short_hash.bright_black(),
                commit.subject().bold(),
                commit.refs().join(", ").magenta(),
            );
        }
    }

    let status = fetch_status();

    if let Ok(status) = status
        && status.total_files() != 0
    {
        println!("{}", "Uncommitted changes".yellow().bold());
    }
}
