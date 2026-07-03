#[macro_export]
macro_rules! handle_error {
    ($ex:expr) => {
        if let Err(err) = $ex {
            use owo_colors::OwoColorize;

            eprintln!("{} {}", "error:".bright_red().bold(), err.bold());
            return;
        };
    };
}

#[macro_export]
macro_rules! check_for_git_repo {
    () => {
        use $crate::git::repo::is_git_repo;

        if !is_git_repo() {
            use owo_colors::OwoColorize;

            eprintln!(
                "{} {}",
                "error:".bright_red().bold(),
                "not a git repository".bold()
            );
            return;
        }
    };
}
