mod cli;
mod engine;
mod network;
use std::sync::{Arc, Mutex};
mod system;

use env_logger::Env;
use log::{info};

use engine::engine::{Engine};
use network::game::server_udp::{start as game_server_start};
use network::http_api::server::{start as http_api_server_start};
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

fn start_engine(engine_arc: Arc<Mutex<Engine>>) -> Result<(), Error> {
    let mut engine = engine_arc.lock().unwrap();
    engine.init()?;
    engine.run();
    Ok(())
}

fn main() -> Result<(), Error> {
    let config = init_config()?;
    let engine_arc = Arc::new(Mutex::new(Engine::create(config)));
    start_engine(engine_arc.clone())?;
    game_server_start(engine_arc.clone())?;
    http_api_server_start(engine_arc.clone())?;

    cli::handler();

    info!("Server terminated.");
    Ok(())
}
