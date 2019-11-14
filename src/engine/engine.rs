use std::thread;
use std::time::{Duration};

use mongodb::{Client, ThreadedClient};
use log::{info};

use super::super::system::configuration::{Configuration};
use super::super::system::error::{Error};

pub struct Engine {
    pub configuration: Configuration,
    datasource: Option<Client>,
}

impl Engine {
    pub const fn create(configuration: Configuration) -> Engine {
        Engine {
            configuration: configuration,
            datasource: None,
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        // TODO: Fetch db address from config
        let datasource = match Client::connect("localhost", 27017) {
            Ok(d) => {
                info!("Connected to database");
                Ok(d)
            },
            Err(_) => Err(Error::create(format!("Cannot connect to database")))
        }?;
        self.datasource = Some(datasource);
        info!("An engine is initialized.");
        Ok(())
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