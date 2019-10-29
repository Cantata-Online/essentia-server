use std::thread;
use std::time::{Duration};

use log::{info};

use super::super::system::configuration::{Configuration};

pub struct Engine {
    pub configuration: Option<Configuration>
}

impl Engine {
    pub const fn create_empty() -> Engine {
        Engine {
            configuration: None
        }
    }

    pub fn init(&mut self, configuration: Configuration) {
        self.configuration = Some(configuration);
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
}