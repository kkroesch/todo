//! # Configuration module
//!
//! This module defines the structure for the configurations items
//! and handles the loading of config files as well as environment variables.
//!
//!
//! ## Usage
//!
//! ```
//! let config = Config::new().unwrap();
//! let db = sled::open(config.db_path)?;
//! ```
//!
//! ## Example Config File
//!
//! ```
//! db_path = "/home/benutzername/.local/share/todo"
//! log_level = "info"
//!
//! [server]
//! host = "127.0.0.1"
//! port = 8080
//! ```
//!

use config::{Config as ConfigLoader, ConfigError, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Config {
    pub db_path: String,
    pub log_level: Option<String>,
    pub server: Option<ServerConfig>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("todo");
        let config_file = config_dir.join("config.toml");

        let s = ConfigLoader::builder()
            .add_source(File::with_name(config_file.to_str().unwrap()))
            .build()?;

        s.try_deserialize()
    }
}
