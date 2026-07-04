#[macro_export]
macro_rules! print_error {
    ($err:expr) => {
        use owo_colors::OwoColorize;

        eprintln!("{} {}", "error:".bright_red().bold(), $err.bold());
    };
}

#[macro_export]
macro_rules! handle_error {
    ($ex:expr) => {
        if let Err(err) = $ex {
            use std::process::ExitCode;
            use $crate::print_error;

            print_error!(err);
            return ExitCode::FAILURE;
        };
    };
}

#[macro_export]
macro_rules! check_for_git_repo {
    () => {
        use $crate::git::repo::is_git_repo;

        if !is_git_repo() {
            use std::process::ExitCode;
            use $crate::print_error;

            print_error!("not a git repository");
            return ExitCode::FAILURE;
        }
    };
}
