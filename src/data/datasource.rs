use super::models::account::{Account};
use super::super::system::error::{Error};

pub trait Datasource {
    fn account_create(&self, account: Account) -> Result<(), Error>;
    fn account_auth(&self, account: Account) -> bool;
}