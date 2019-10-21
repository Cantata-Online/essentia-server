use serde::{Serialize, Deserialize};
use std::fs;

use super::super::cli::arg_parse::{Arguments};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GameServerConfiguration {
    socket_type: String,
    host: String,
    port: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfiguration {
    game: GameServerConfiguration
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    server: ServerConfiguration
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
        //return Err(String::from("Cannot parse the YAML configuration file."));
    }
    let configuration = configuration_option.unwrap();
    Ok(configuration)
}