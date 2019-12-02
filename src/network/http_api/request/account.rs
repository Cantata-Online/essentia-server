use serde::{Deserialize};

use super::super::super::super::data::models::account::{Account};

#[derive(Deserialize)]
pub struct AccountCreateRequest {
    pub login: String,
    pub password: String
}

impl AccountCreateRequest {
    pub fn to_engine_struct(&self) -> Account {
        Account {
            login: self.login.clone(),
            password: self.password.clone(),
        }
    }
}