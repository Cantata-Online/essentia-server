use std::net::UdpSocket;
use std::thread;

use log::{debug, error, info};
use super::packets;
use super::super::super::engine::engine::{Engine};
use super::super::super::system::error::{Error};

const BUFFER_SIZE:usize = 200;

const SERVER_STATUS_RESPONSE_OK:&'static[u8; 1] = &[0x01];

pub fn start(engine: &Engine) -> Result<(), Error> {
    let game_configuration = &engine.configuration.as_ref().unwrap().server.game;
    let address = format!("{}:{}", game_configuration.host, game_configuration.port);
    let socket = UdpSocket::bind(address.clone()).unwrap();
    info!("Game server is listening on address {}", address);

    thread::spawn(move || {
        loop {
            let mut buf = [0; BUFFER_SIZE];
            let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
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
    Ok(())
}