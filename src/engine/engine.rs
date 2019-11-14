use std::thread;
use std::time::{Duration};

use log::{info};

use super::super::system::configuration::{Configuration};

pub struct Engine {
    pub configuration: Configuration
}

impl Engine {
    pub const fn create(configuration: Configuration) -> Engine {
        Engine {
            configuration: configuration
        }
    }

    pub fn init(&mut self) {
        info!("An engine is initialized.");
    }

    pub fn run(&self) {
        info!("Starting the engine...");
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1000));
            }
        });
    }

    pub fn account_create(&self) {
        // TODO: implement account creation. Maybe need to make submodules(account management, player actions, stats, etc)
        info!("TEST: {}", self.configuration.server.game.socket_type.clone());
    }
}