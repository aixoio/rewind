use clap::Subcommand;

pub mod status;

#[derive(Subcommand)]
pub enum Commands {
    Status,
}
