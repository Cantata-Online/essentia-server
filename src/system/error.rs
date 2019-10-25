use std::fmt;

use log::{error};

pub struct Error {
    message: String
}

impl Error {
    pub fn create(message: String) -> Error {
        Error{
            message: message.clone()
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error!("{}", self.message);
        write!(
            f,
            "An error occurred: {}",
            self.message
        )
    }
}