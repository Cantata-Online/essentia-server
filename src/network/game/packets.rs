use bincode;
use serde::Serialize;

use super::packet_codes;
use super::super::super::data::models::account::Account;

pub struct PacketLogin {
    pub header: u16,
    pub login: String,
    pub password: String,
}

impl PacketLogin {
    pub fn create_from_bytes(bytes: &[u8]) -> PacketLogin {
        let packet = PacketLogin {
            header: packet_codes::PACKET_CODE_LOGIN,
            login: String::from(String::from_utf8(bytes[2..32].to_vec()).unwrap_or("TEST1".to_string()).trim_end_matches('\0')),
            password: String::from(String::from_utf8(bytes[33..63].to_vec()).unwrap_or("TEST2".to_string()).trim_end_matches('\0')),
        };
        packet
    }

    pub fn to_model(&self) -> Account {
        let mut account = Account::create();
        account.login = self.login.clone();
        account.password = self.password.clone();

        account
    }
}

#[derive(Serialize)]
pub struct PacketLoginResponse {
    pub header: u16,
    pub status: bool,
    pub token: [u8;16],
}

impl PacketLoginResponse {
    pub fn create() -> PacketLoginResponse {
        PacketLoginResponse {
            header: packet_codes::PACKET_CODE_STATUS_RESPONSE,
            status: false,
            token: [0; 16],
        }
    }

    pub fn to_vector(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}