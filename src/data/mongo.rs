use log::{info};
use mongodb::{Client, ThreadedClient};
use mongodb::{bson, doc};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;

use super::datasource::{Datasource};
use super::super::data::models::account::{Account};
use super::super::system::error::{Error};

pub struct Mongo {
    client: Client,
}

pub struct MongoConfig {
    pub host: String,
    pub port: u16,
}

impl Mongo {
    fn get_table(&self, name: &str) -> Collection {
        let client = &self.client;
        let database = client.db("main");
        let accounts = database.collection(name);

        accounts
    }

    pub fn create(mongo_config: MongoConfig) -> Result<Mongo, Error> {
        let client = match Client::connect(mongo_config.host.as_str(), mongo_config.port) {
            Ok(d) => {
                info!("Connected to database");
                Ok(d)
            },
            Err(_) => Err(Error::create(format!("Cannot connect to database")))
        }?;
        let mongo = Mongo {
            client: client,
        };

        Ok(mongo)
    }
}

impl Datasource for Mongo {
    fn account_create(&self, account: Account) -> Result<(), Error> {
        let accounts = self.get_table("accounts");
        let existing_account_option = match accounts.find_one(Some(doc! {
            "login": account.login.clone()
        }), None) {
            Ok(r) => Ok(r),
            Err(_) => Err(Error::create(format!("An internal error occurred"))),
        }?;
        if !existing_account_option.is_none() {
            return Err(Error::create(format!("An account with provided login already exists")));
        }
        match accounts.insert_one(account.to_bson(), None) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::create(format!("Failed to create an account")))
        }
    }
}