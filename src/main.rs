mod network;
mod system;

use env_logger::Env;
use log::{info, debug};
use std::io;

use network::server_udp::{Listener};
use system::cli_handler::{cli_handler};


fn main() -> io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let mut server = Listener::create();
    server.start();
    info!("Server started.");

    cli_handler();

    debug!("Exited from main loop");
    info!("Server terminated.");

    Ok(())
}
