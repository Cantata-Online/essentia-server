mod network;

use std::time::Duration;
use std::thread;

use network::server_udp::{Listener};

fn main() {
    let server = Listener::create();

    server.start();
    println!("Server started.");

    loop {
        thread::sleep(Duration::from_millis(5000));
        println!("Running...");
    }
}
