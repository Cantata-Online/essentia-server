use mongodb::{Document, bson, doc};

pub struct Account {
    pub login: String,
    pub password: String
}

impl Account {
    pub fn to_bson(&self) -> Document {
        doc! {
            "login": self.login.clone(),
            "password": self.password.clone(),
        }
    }
}