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
    let yaml_content_option = fs::read_to_string(cli_args.config_file);
    if yaml_content_option.is_err() {
        return Err(String::from("Cannot open configuration file."));
    }
    let yaml_content = yaml_content_option.unwrap();

    let configuration_option = serde_yaml::from_str(&yaml_content);
    if configuration_option.is_err() {
        return Err(configuration_option.unwrap_err().to_string());
    }
    let configuration = configuration_option.unwrap();
    Ok(configuration)
}