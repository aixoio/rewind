use crate::git::remote;

use owo_colors::OwoColorize;

pub fn run() {
    let upstream = remote::upstream();

    if upstream.is_none() {
        println!("{}", "No upstream branch configured".italic());

        println!("{}", "Setting upstream to origin/HEAD".blue());

        remote::push_set_upstream().expect("failed to set upstream");

        println!("{}", "Push completed!".green().bold());
        println!("{}", "Upstream set and changes pushed".bright_black());
    }

    if upstream.is_some() {
        println!("{}", "Pushing changes to remote...".blue());

        remote::push().expect("failed to push upstream");

        println!("{}", "Push completed!".green().bold());
    }
}
