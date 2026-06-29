use crate::git::repo::{add_paths, is_git_repo};

use owo_colors::OwoColorize;

pub fn run(paths: Option<Vec<String>>) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    let paths = paths.unwrap_or_else(|| vec![".".to_string()]);

    add_paths(&paths).expect("cannot add paths");

    println!("{}", "Files staged successfully!".green().bold());
    println!("{}", "Ready for commit.".bright_black());
}
