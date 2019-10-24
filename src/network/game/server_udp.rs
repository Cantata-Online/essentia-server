use std::net::UdpSocket;
use std::thread;

use log::{debug, error, info};
use super::packets;
use super::super::super::system::configuration::{GameServerConfiguration};

const BUFFER_SIZE:usize = 200;

const SERVER_STATUS_RESPONSE_OK:&'static[u8; 1] = &[0x01];

pub struct Listener {
    socket: Option<UdpSocket>,
    configuration: GameServerConfiguration,
}

impl Listener {
    pub fn create(configuration: GameServerConfiguration) -> Listener {
        Listener{
            configuration: configuration,
            socket: None
        }
    }

    pub fn start(&mut self) {
        let address = format!("{}:{}", self.configuration.host, self.configuration.port);
        let socket = UdpSocket::bind(address.clone()).unwrap();
        self.socket = Some(socket.try_clone().expect("Cannot clone a socket"));
        info!("Game server is listening on address {}", address);

        thread::spawn(move || {
            loop {
                let mut buf = [0; BUFFER_SIZE];
                let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
                                                .expect("Cannot receive data");
                if number_of_bytes < 2 {
                    continue;
                }

                let packet_code:u16 = ((buf[0] as u16) * 256 + (buf[1] as u16)).into();
                debug!("Got {} bytes: {:#X}\n", number_of_bytes, packet_code);
                match packet_code {
                    packets::PACKET_CODE_STATUS => {
                        if socket.send_to(SERVER_STATUS_RESPONSE_OK, src_addr).is_err() {
                            error!("Cannot send a packet");
                        }
                    },
                    _ => { debug!("Default handler"); },
                }
            }
        });        
    }
}