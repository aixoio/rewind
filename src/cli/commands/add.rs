use crate::git::repo::{add_paths, is_git_repo};

use owo_colors::OwoColorize;

macro_rules! handle_error {
    ($ex:expr) => {
        if let Err(err) = $ex {
            println!("{} {}", "error:".bright_red().bold(), err.bold());
            return;
        };
    };
}

pub fn run(paths: Option<Vec<String>>) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    let paths = paths.unwrap_or_else(|| vec![".".to_string()]);

    handle_error!(add_paths(&paths));

    println!("{}", "Files staged successfully!".green().bold());
    println!("{}", "Ready for commit.".bright_black());
}
