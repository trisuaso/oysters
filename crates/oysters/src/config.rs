//! Application config manager
use pathbufd::PathBufD;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Result;

/// Configuration file
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    /// The port to serve the server on.
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_port() -> u16 {
    5072
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: default_port(),
        }
    }
}

impl Config {
    /// Read configuration file into [`Config`]
    pub fn read(contents: String) -> Self {
        toml::from_str::<Self>(&contents).unwrap()
    }

    /// Pull configuration file
    pub fn get_config() -> Self {
        let path = PathBufD::current().extend(&[".config", "config.toml"]);

        match fs::read_to_string(&path) {
            Ok(c) => Config::read(c),
            Err(_) => {
                Self::update_config(Self::default()).expect("failed to write default config");
                Self::default()
            }
        }
    }

    /// Update configuration file
    pub fn update_config(contents: Self) -> Result<()> {
        let c = fs::canonicalize(".").unwrap();
        let here = c.to_str().unwrap();

        fs::write(
            format!("{here}/.config/config.toml"),
            toml::to_string_pretty::<Self>(&contents).unwrap(),
        )
    }
}
