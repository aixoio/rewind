use crate::git::repo::{init_repo, is_git_repo};

use owo_colors::OwoColorize;

pub fn run() {
    if is_git_repo() {
        println!("{}", "Reinitialized existing Git repository");
    }

    init_repo().expect("faild to initialize git repo");

    println!("{}", "Initialized Git repository".bold());
}
