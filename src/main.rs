mod cli;
mod network;

use env_logger::Env;
use log::{info, debug};
use std::io;

use network::game::server_udp::{Listener};

fn main() -> io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let mut server = Listener::create();
    server.start();
    info!("Server started.");

    cli::handler();

    debug!("Exited from main loop");
    info!("Server terminated.");

    Ok(())
}
