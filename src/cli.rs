use clap::{Parser, Subcommand};

use crate::cargo_toml::CargoToml;
use crate::git::Git;
use crate::version::Version;

const ABOUT: &str = r#"Cargo plugin to bump crate's versions and Git tag them
for release.

"cargo tag" helps to automate the process of bumping versions
similar to how "npm version" does.

When bumping versions with "cargo tag", the
Cargo.toml and Cargo.lock files are updated with the new version, then a Git
commit and a Git tag are both created."#;

#[derive(Parser)]
#[command(bin_name = "cargo")]
#[command(next_line_help = true)]
#[command(name = "cargo", author, version, about, long_about = Some(ABOUT))]
pub enum Cli {
    #[command(subcommand)]
    Tag(Command),
}

impl Cli {
    pub fn exec(self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Tag(cmd) => cmd.exec(),
        }
    }
}

#[derive(Clone, Subcommand)]
pub enum Command {
    /// Print current package version
    Current,
    /// Bumps crate's minor version and create a git tag
    Minor,
    /// Bumps crate's major version and create a git tag
    Major,
    /// Bumps crate's patch version and create a git tag
    Patch,
}

impl Command {
    pub fn exec(&self) -> Result<(), Box<dyn std::error::Error>> {
        match *self {
            Command::Current => {
                let cargo_toml = CargoToml::open().unwrap();

                println!("{}", cargo_toml.package.version);
            }
            Command::Major | Command::Minor | Command::Patch => {
                let cargo_toml = CargoToml::open()?;
                let repository = Git::open("main")?;
                let mut version = Version::from(&cargo_toml.package.version);

                match self {
                    Command::Major => version.bump_major(),
                    Command::Minor => version.bump_minor(),
                    Command::Patch => version.bump_patch(),
                    _ => unreachable!(),
                };

                cargo_toml
                    .write_version(&version)
                    .expect("Failed to write version to Cargo.toml");

                cargo_toml
                    .run_cargo_check()
                    .expect("Failed to run `cargo check`");

                repository
                    .commit(&format!("chore: bump version to {}", version))
                    .expect("Failed to commit files");

                repository
                    .tag(&version, "chore: bump version to {}")
                    .expect("Failed to create Git tag");
            }
        }

        Ok(())
    }
}
