use std::net::UdpSocket;
use std::thread;

use log::{debug, error};
use super::packets;

const BUFFER_SIZE:usize = 200;

const SERVER_STATUS_RESPONSE_OK:&'static[u8; 1] = &[0x01];

pub struct Listener {
    socket: Option<UdpSocket>
}

impl Listener {
    pub fn create() -> Listener {
        Listener{
            socket: None
        }
    }

    pub fn start(&mut self) {
        let socket = UdpSocket::bind("127.0.0.1:2300").unwrap();
        self.socket = Some(socket.try_clone().expect("Cannot clone a socket"));

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