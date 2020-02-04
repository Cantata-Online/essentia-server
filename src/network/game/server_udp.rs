use std::thread;
use std::sync::{Arc, Mutex};

use log::{debug, error};

use super::connector::Connector;
use super::packets;
use super::super::super::engine::engine::{Engine};
use super::super::super::system::error::{Error};

const BUFFER_SIZE:usize = 200;

const SERVER_STATUS_RESPONSE_OK:&'static[u8; 1] = &[0x01];

fn thread_fn(connector: Connector) {
    loop {
        let mut buf = [0; BUFFER_SIZE];
        // let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).unwrap();
        let (number_of_bytes, src_addr) = connector.read_udp(&mut buf).unwrap();

        if number_of_bytes < 2 {
            continue;
        }

        let packet_code:u16 = ((buf[1] as u16) * 256 + (buf[0] as u16)).into();
        debug!("Got {} bytes: {:#X}\n", number_of_bytes, packet_code);
        match packet_code {
            packets::PACKET_CODE_STATUS => {
                if connector.send(src_addr, SERVER_STATUS_RESPONSE_OK).is_err() {
                    error!("Cannot send a packet");
                }
            },
            _ => { debug!("Default handler"); },
        }
    }
}

pub fn start(engine_arc: Arc<Mutex<Engine>>) -> Result<(), Error> {
    let engine = engine_arc.lock().unwrap();
    let game_configuration = &engine.configuration.server.game;
    let connector = Connector::create(engine_arc.clone(), game_configuration.host.clone(), game_configuration.port);

    thread::spawn(move || thread_fn(connector));
    Ok(())
}