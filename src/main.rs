mod cli;
mod engine;
mod network;
mod system;

use env_logger::Env;
use log::{info};

use engine::engine::{Engine};
use network::game::server_udp::{start as game_server_start};
use network::http_api::server::{start as http_api_server_start};
use system::configuration::{Configuration};
use system::error::{Error};

static mut ENGINE: Engine = Engine::create_empty();

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

fn start_engine(configuration: Configuration) -> Result<(), Error> {
    // let engine = Engine::create(configuration);
    unsafe {
        ENGINE.init(configuration);
        ENGINE.run();
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let config = init_config()?;
    unsafe {
        start_engine(config)?;
        let engine2 = Box::from(&ENGINE);
        game_server_start(engine2.as_ref())?;
        http_api_server_start(engine2.as_ref())?;
    }

    cli::handler();

    info!("Server terminated.");
    Ok(())
}
