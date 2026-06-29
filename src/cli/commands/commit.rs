use inquire::{
    Text,
    ui::{Color, RenderConfig, StyleSheet},
    validator::Validation,
};
use owo_colors::{OwoColorize, colors::xterm::BlazeOrange};

use crate::git::{
    repo::{add_paths, commit, is_git_repo},
    status::fetch_status_raw,
};

pub fn run(message: Option<String>) {
    if !is_git_repo() {
        eprintln!("{}", "Not a git repository".bright_red().bold());
        return;
    }

    println!();

    if message.is_none() {
        add_paths(&[".".to_string()]).expect("cannot add paths");

        println!("{}", "Staged all files".green());
        println!();
    }

    let raw_status = fetch_status_raw().expect("cannot fetch status");

    println!("{}:", "Files to be committed".fg::<BlazeOrange>().bold());

    for line in raw_status.lines() {
        println!("     {line}");
    }

    println!();

    let style = RenderConfig {
        answer: StyleSheet {
            fg: Some(Color::Grey),
            bg: None,
            ..Default::default()
        },
        ..Default::default()
    };

    inquire::set_global_render_config(style);

    let message = message.unwrap_or_else(|| {
        Text::new("commit message:")
            .with_validator(|s: &str| {
                if s.trim().is_empty() {
                    Ok(Validation::Invalid(
                        "You must enter a commit message".into(),
                    ))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()
            .unwrap()
    });

    commit(&message).expect("faild to commit");

    println!();

    println!("{}", "Commit successful!".green().bold());
    println!(
        "{} {}",
        "Message:".bright_black().bold(),
        message.bright_black().italic()
    );
}
