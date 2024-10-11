//! # Configuration module
//!
//! ## Usage
//!
//!
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
//!

use config::{Config as ConfigLoader, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_path: String,
    //pub log_level: Option<String>,
    //pub server: Option<ServerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    //pub host: String,
    //pub port: u16,
}

pub fn load_settings() -> Result<Config, Box<dyn std::error::Error>> {
    // Erstellen eines neuen Konfigurations-Loaders
    let mut settings = ConfigLoader::default();

    // Standardwerte festlegen
    let default_db_path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("todo")
        .to_str()
        .unwrap()
        .to_string();

    // Konfigurationsdatei hinzufügen (optional)
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("todo");
    let config_file = config_dir.join("config.toml");

    if config_file.exists() {
        settings.merge(File::from(config_file))?;
    } else {
        // Falls die Konfigurationsdatei nicht existiert, erstellen wir sie mit den Standardwerten
        std::fs::create_dir_all(&config_dir)?;
        std::fs::write(config_file, format!("db_path = \"{}\"", default_db_path))?;
    }

    let config = settings.try_deserialize()?;
    Ok(config)
}
