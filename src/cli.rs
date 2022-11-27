use clap::{Parser, Subcommand};

use crate::cargo_toml::CargoToml;

#[derive(Parser)]
#[command(author, version, about, long_about = Some("Cargo plugin to tag crate version"))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    /// Print current package version
    Current,
}

impl Command {
    pub fn exec(&self) {
        match *self {
            Command::Current => {
                let cargo_toml = CargoToml::open().unwrap();

                print!("{:#?}", cargo_toml);
            }
        }
    }
}
