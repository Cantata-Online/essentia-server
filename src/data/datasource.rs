use std::marker::Send;

use crate::data::models::account::{Account};
use crate::system::error::{Error};


pub trait Datasource: Send {
    fn account_create(&self, account: Account) -> Result<(), Error>;
    fn account_auth(&self, account: Account) -> bool;
}