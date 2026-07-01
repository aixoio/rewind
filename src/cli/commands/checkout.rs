use crate::git::repo::is_git_repo;

use owo_colors::OwoColorize;

pub fn run(target: String) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    println!("target: {target}");
}
