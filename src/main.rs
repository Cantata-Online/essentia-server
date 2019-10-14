mod network;

use env_logger::Env;
use log::{info, debug, error};

use std::io;

use network::server_udp::{Listener};

fn init_logger() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
}

fn main() -> io::Result<()> {
    init_logger();

    let mut server = Listener::create();
    server.start();
    info!("Server started.");

    let mut do_exit:bool = false;
    while !do_exit {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                input = String::from(input.trim());
                match input.trim().as_ref() {
                    "quit" => {
                        do_exit = true;
                    },
                    _ => error!("Unknown command: {}", input)
                }
            }
            Err(error) => error!("{}", error),
        }
    }
    debug!("Exited from main loop");
    info!("Server terminated.");

    Ok(())
}
