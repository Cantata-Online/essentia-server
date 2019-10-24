mod cli;
mod network;
mod system;

use env_logger::Env;
use log::{info, debug, error};

use network::game::server_udp::{Listener};
use network::http_api::server::{start as http_server_start};

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let cli_args = cli::arg_parse::parse();

    let config_option = system::configuration::configure(cli_args);
    if config_option.is_err() {
        error!("{}", config_option.unwrap_err());
        return;
    }
    let config = config_option.unwrap();

    let mut game_server = Listener::create(config.server.game.clone());
    game_server.start();
    let http_api_result = http_server_start(config);
    if http_api_result.is_err() {
        error!("{}", http_api_result.unwrap_err());
        return;
    }
    cli::handler();

    debug!("Exited from main loop");
    info!("Server terminated.");
}
