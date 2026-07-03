use crate::{
    git::repo::{init_repo, is_git_repo},
    handle_error,
};

use owo_colors::OwoColorize;

pub fn run() {
    if is_git_repo() {
        println!("Reinitialized existing Git repository");
    }

    handle_error!(init_repo());

    println!("{}", "Initialized Git repository".bold());
}
