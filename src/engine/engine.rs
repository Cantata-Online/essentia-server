use std::thread;
use std::time::{Duration};

use log::{info};

use super::super::system::configuration::{Configuration};
use super::super::system::error::{Error};
use super::super::data::models::account::{Account};
use super::super::data::datasource::{Datasource};
use super::super::data::mongo::{Mongo, MongoConfig};

const URI_MONGODB: &str = "mongodb://";

pub struct Engine {
    pub configuration: Configuration,
    pub datasource: Option<Mongo>,
}

impl Engine {
    pub const fn create(configuration: Configuration) -> Engine {
        Engine {
            configuration: configuration,
            datasource: None,
        }
    }

    fn get_datasource(&self) -> &Mongo {
        self.datasource.as_ref().unwrap()
    }

    fn init_datasource(&mut self) -> Result<(), Error> {
        if self.configuration.engine.datasource.uri[..URI_MONGODB.len()] != *URI_MONGODB {
            return Err(Error::create(format!("Unknown datasource. Only mongodb:// is a valid datasource")))
        }
        let host_port = &self.configuration.engine.datasource.uri[URI_MONGODB.len()..];
        let host_port: Vec<&str> = host_port.split(":").collect();
        if host_port.len() != 2 {
            return Err(Error::create(format!("Port is not specified. Please set a datasource URI in format mongodb://host:port")))
        }
        let host = host_port[0];
        let port: u16 = match host_port[1].parse() {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::create(format!("Invalid port: {}", host_port[1])))
        }?;

        let mongo_config = MongoConfig {
            host: String::from(host),
            port: port
        };
        let datasource = match Mongo::create(mongo_config) {
            Ok(d) => Ok(d),
            Err(e) => Err(e),
        }?;

        self.datasource = Some(datasource);
        Ok(())
    }

    pub fn init(&mut self) -> Result<(), Error> {
        self.init_datasource()?;
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

    pub fn account_create(&self, account: Account) -> Result<(), Error> {
        self.datasource.as_ref().unwrap().account_create(account)
    }

    pub fn account_login(&self, account: Account) -> bool {
        self.get_datasource().account_auth(account)
    }
}