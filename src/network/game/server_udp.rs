use std::thread;
use std::sync::{Arc, Mutex};

use log::{debug, error};

use crate::engine::engine::{Engine};
use crate::network::game::connector::Connector;
use crate::network::game::{BUFFER_SIZE, packet_codes, packets};
use crate::network::game::request_context::RequestContext;
use crate::system::error::{Error};

const SERVER_STATUS_RESPONSE_OK:&'static[u8; 1] = &[0x01];

fn handle_packet_status(context: RequestContext) {
    if context.connector.send(context.src_addr, SERVER_STATUS_RESPONSE_OK).is_err() {
        error!("Cannot send a packet");
    }
}

fn handle_packet_login(context: RequestContext) {
    debug!("Login packet received");
    let data = &context.buffer[0..64];
    let packet = packets::PacketLogin::create_from_bytes(&data);
    let account = packet.to_model();
    let mut response = packets::PacketLoginResponse::create();
    {
        let engine = context.connector.engine_arc.lock().unwrap();
        let is_succeeded = engine.account_login(account);
        response.status = is_succeeded;
    }

    match context.connector.send(context.src_addr, &response.to_vector()) {
        Err(_) => { error!("Failed to send a login response packet"); },
        _ => {},
    };
}

fn thread_fn(connector: Connector) {
    loop {
        let mut buf = [0; BUFFER_SIZE];
        let (number_of_bytes, src_addr) = connector.read_udp(&mut buf).unwrap();

        if number_of_bytes < 2 {
            continue;
        }

        let packet_code = ((buf[1] as u16) << 8) + buf[0] as u16;
        debug!("Got {} bytes: {:#X}\n", number_of_bytes, packet_code);
        let context = RequestContext::new(buf, &connector, src_addr);
        match packet_code {
            packet_codes::PACKET_CODE_STATUS => handle_packet_status(context),
            packet_codes::PACKET_CODE_LOGIN => handle_packet_login(context),
            _ => { debug!("Default handler"); },
        }
    }
}

pub fn start(engine_arc: Arc<Mutex<Engine>>) -> Result<(), Error> {
    let connector = {
        let engine = engine_arc.lock().unwrap();
        let game_configuration = &engine.configuration.server.game;
        Connector::create(engine_arc.clone(), game_configuration.host.clone(), game_configuration.port)
    };
    thread::spawn(move || thread_fn(connector));
    Ok(())
}