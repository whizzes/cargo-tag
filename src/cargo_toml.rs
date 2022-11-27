use std::env::current_dir;
use std::fs::read_to_string;

use semver::Version;
use serde::Deserialize;

const CARGO_TOML: &str = "Cargo.toml";

/// A `Cargo.toml` file's representation as a struct
#[derive(Debug, Deserialize)]
pub struct CargoToml {
    pub(crate) package: Package,
}

/// Representation of the `Cargo.toml` `package` section
#[derive(Debug, Deserialize)]
pub struct Package {
    /// Crate name
    pub(crate) name: String,
    /// Crate version
    pub(crate) version: Version,
}

impl CargoToml {
    /// Attempts to read a `Cargo.toml` in the current directory and retrieves
    /// an instance of `CartToml` from it.
    pub fn open() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = current_dir()?;
        path.push(CARGO_TOML);

        let file = read_to_string(path)?;
        let package: CargoToml = toml::from_str(&file)?;

        Ok(package)
    }
}
