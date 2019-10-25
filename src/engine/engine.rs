use std::thread;

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

    pub fn run(&self) {
        thread::spawn(move || {
            
        });
    }
}