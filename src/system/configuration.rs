use serde::{Deserialize};
use std::fs;

use super::super::cli::arg_parse::{Arguments};

#[derive(Debug, Deserialize, Clone)]
pub struct GameServerConfiguration {
    pub socket_type: String,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HttpApiServerConfiguration {
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfiguration {
    pub game: GameServerConfiguration,
    pub http_api: HttpApiServerConfiguration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfiguration {
    pub base_directory: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub app: AppConfiguration,
    pub server: ServerConfiguration,
}

pub fn configure(cli_args: Arguments) -> Result<Configuration, String> {
    let config_file = cli_args.config_file.clone();
    let yaml = match fs::read_to_string(config_file.clone()) {
        Err(e) => Err(format!("Cannot open configuration file ({}): {}", config_file, e.to_string())),
        Ok(y) => Ok(y),
    }?;
    let config = match serde_yaml::from_str(&yaml) {
        Err(e) => Err(format!("Cannot load configuration from YAML file: {}", e.to_string())),
        Ok(config) => Ok(config)
    }?;
    Ok(config)
}