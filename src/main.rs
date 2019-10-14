mod network;

use env_logger::Env;
use log::{info, debug};

use std::time::Duration;
use std::thread;

use network::server_udp::{Listener};

fn init_logger() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
}

fn main() {
    init_logger();

    let mut server = Listener::create();
    server.start();
    info!("Server started.");

    loop {
        thread::sleep(Duration::from_millis(5000));
        debug!("Running...");
    }
}
