use crate::git::remote;

use owo_colors::OwoColorize;

pub fn run() {
    match remote::upstream() {
        Some(_) => {
            println!("{}", "Pulling changes from remote...".blue());

            remote::pull().expect("failed to pull upstream");

            println!("{}", "Pull completed!".green().bold());
        }
        None => {
            println!("{}", "No upstream branch configured".italic());
            println!("{}", "Setting upstream to origin/HEAD".blue());

            remote::pull_set_upstream().expect("failed to set upstream");

            println!("{}", "Pull completed!".green().bold());
            println!("{}", "Upstream set and changes pulled".bright_black());
        }
    };
}
