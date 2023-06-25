use cargo_tag::cli::Cli;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli::Tag(args) = Cli::parse();
    args.command.exec(args.env)
}
