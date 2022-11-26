use cargo_tag::cli::{Cli, Command};
use clap::Parser;

fn main() {
   let cli = Cli::parse();

   match &cli.command {
      Some(Command::Current) => println!("Print current package version"),
      None => {}
   }
}
