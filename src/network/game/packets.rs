use super::packet_codes;

pub struct PacketLogin {
    pub header: u16,
    pub login: String,
    pub password: String,
}

impl PacketLogin {
    pub fn create_from_bytes(bytes: &[u8]) -> PacketLogin {
        let packet = PacketLogin {
            header: packet_codes::PACKET_CODE_LOGIN,
            login: String::from_utf8(bytes[2..32].to_vec()).unwrap_or("TEST1".to_string()),
            password: String::from_utf8(bytes[33..63].to_vec()).unwrap_or("TEST2".to_string()),
        };
        packet
    }
}