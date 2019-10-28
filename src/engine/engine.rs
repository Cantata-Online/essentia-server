use std::thread;
use std::time::{Duration};

use log::{info};

use super::super::system::configuration::{Configuration};

pub struct Engine {
    configuration: Configuration
}

impl Engine {
    pub fn create(configuration: Configuration) -> Engine {
        Engine{
            configuration: configuration
        }
    }

    pub fn init(&self) {
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