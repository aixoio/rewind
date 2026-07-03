use crate::{
    check_for_git_repo,
    git::{
        branch::current_branch,
        log::{fetch_log, fetch_log_with_limit, parse_commit_log},
        status::{fetch_status, parse_status},
    },
};

use owo_colors::OwoColorize;

pub fn run(limit: Option<usize>, show_all: bool) {
    check_for_git_repo!();

    let limit = limit.unwrap_or(10);

    let commits = if show_all {
        fetch_log()
    } else {
        fetch_log_with_limit(limit)
    };
    let commits = match commits {
        Ok(commits) => commits,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red().bold(), err.bold());
            return;
        }
    };

    let commits = parse_commit_log(&commits);

    println!("{}", "Git log".cyan().bold());

    if let Ok(branch) = current_branch() {
        println!("On branch {}", branch.bold());
    }

    println!();

    for commit in commits {
        let short_hash = &commit.hash()[..7];

        if commit.refs().is_empty() {
            println!("{} {}", short_hash.bright_black(), commit.subject().bold());
        } else {
            println!(
                "{} {} {}",
                short_hash.bright_black(),
                commit.subject().bold(),
                commit.refs().blue(),
            );
        }
    }

    let Ok(status) = fetch_status() else {
        return;
    };

    if parse_status(&status).total_files() != 0 {
        println!("{}", "Uncommitted changes".yellow().bold());
    }
}
