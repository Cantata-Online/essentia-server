mod cli;
mod network;
mod system;

use env_logger::Env;
use log::{info, debug, error};

use network::game::server_udp::{Listener};

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let cli_args = cli::arg_parse::parse();

    let config_option = system::configuration::configure(cli_args);
    if config_option.is_err() {
        error!("{}", config_option.unwrap_err());
        return;
    }
    let config = config_option.unwrap();

    let mut server = Listener::create();
    server.start();
    info!("Server started.");

    cli::handler();

    debug!("Exited from main loop");
    info!("Server terminated.");
}
