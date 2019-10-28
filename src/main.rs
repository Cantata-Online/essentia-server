mod cli;
mod engine;
mod network;
mod system;

use env_logger::Env;
use log::{info};

use engine::engine::{Engine};
use network::game::server_udp::{start as game_server_start};
use network::http_api::server::{start as http_server_start};
use system::configuration::{Configuration};
use system::error::{Error};

fn init_config() -> Result<Configuration, Error> {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format_module_path(false)
        .init();
    let cli_args = cli::arg_parse::parse();
    match system::configuration::configure(cli_args) {
        Err(e) => Err(Error::create(e)),
        Ok(c) => Ok(c),
    }
}

fn start_engine(configuration: Configuration) -> Result<Engine, Error> {
    let engine = Engine::create(configuration);
    engine.init();
    engine.run();
    Ok(engine)
}

fn start_api_server(configuration: Configuration) -> Result<(), Error> {
    let http_api_result = http_server_start(configuration);
    match http_api_result {
        Err(e) => Err(Error::create(e)),
        Ok(_) => Ok(()),
    }
}

fn start_game_server(configuration: Configuration) -> Result<(), Error> {
    let result = game_server_start(configuration);
    match result {
        Err(e) => Err(Error::create(e)),
        Ok(_) => Ok(()),
    }
}

fn main() -> Result<(), Error> {
    let config = init_config()?;
    let engine = start_engine(config.clone())?;
    start_game_server(config.clone())?;
    start_api_server(config.clone())?;
    cli::handler();

    info!("Server terminated.");
    Ok(())
}
