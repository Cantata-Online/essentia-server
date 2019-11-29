use std::thread;
use std::time::{Duration};

use mongodb::{Client, ThreadedClient};
use mongodb::{bson, doc};
use mongodb::db::ThreadedDatabase;
use log::{info};

use super::super::system::configuration::{Configuration};
use super::super::system::error::{Error};
use super::models::account::{Account};

const URI_MONGODB: &str = "mongodb://";

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
        let datasource = match Client::connect(host, port) {
            Ok(d) => {
                info!("Connected to database");
                Ok(d)
            },
            Err(_) => Err(Error::create(format!("Cannot connect to database")))
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
        let datasource = self.datasource.as_ref().unwrap();
        let database = datasource.db("main");
        let accounts = database.collection("accounts");


        // accounts.find("asd", "asdsda");
        // accounts.insert_one(doc!{ "title": "Back to the Future" }, None).unwrap();
        accounts.insert_one(account.to_bson(), None).unwrap();
        let existing_account_option = match accounts.find_one(Some(doc! {
            "login": account.login.clone()
        }), None) {
            Ok(r) => Ok(r),
            Err(_) => Err(Error::create(format!("An internal error occurred"))),
        }?;
        if !existing_account_option.is_none() {
            return Err(Error::create(format!("An account with provided login already exists")));
        }
        Ok(())
    }
}