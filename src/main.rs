use std::process::ExitCode;

use clap::Parser;
use rewind::cli::Cli;

fn main() -> ExitCode {
    let cli = Cli::parse();

    cli.command.run()
}
