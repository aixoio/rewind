use std::process::ExitCode;

use inquire::{
    Text,
    ui::{Color, RenderConfig, StyleSheet},
    validator::Validation,
};
use owo_colors::OwoColorize;

use crate::{
    check_for_git_repo,
    git::{
        repo::{add_paths, commit},
        status::fetch_status,
    },
    handle_error, return_error,
};

pub fn run(message: Option<String>) -> ExitCode {
    check_for_git_repo!();

    println!();

    if message.is_none() {
        handle_error!(add_paths(&["."]));

        println!("{}", "Staged all files".blue());
        println!();
    }

    let raw_status = match fetch_status() {
        Ok(status) => status,
        Err(err) => {
            return_error!(err);
        }
    };

    println!("{}:", "Files to be committed".bright_green().bold());

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

    let message = match message {
        Some(msg) => msg,
        None => match Text::new("Commit message:")
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
        {
            Ok(msg) => msg,
            Err(err) => {
                return_error!(format!(
                    "{} {}",
                    "error:".bright_red().bold(),
                    format!("failed to read commit message: {err}").bold(),
                ));
            }
        },
    };

    handle_error!(commit(&message));

    println!();

    println!("{}", "Commit successful!".bright_green().bold());
    println!(
        "{} {}",
        "Message:".bright_black().bold(),
        message.bright_black()
    );

    ExitCode::SUCCESS
}
