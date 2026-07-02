use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum TagCommand {
    List,
}

pub fn run(command: TagCommand) {
    println!("{:?}", command);
}
