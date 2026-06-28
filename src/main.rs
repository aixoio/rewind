use clap::Parser;
use rewind::cli::Cli;

fn main() {
    let cli = Cli::parse();

    cli.command.run();
}
