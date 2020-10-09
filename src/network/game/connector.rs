use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};

use log::{info};

use super::super::super::engine::engine::{Engine};

// This struct performs UDP I/O operations, encrypts/decrypts packets, etc
pub struct Connector {
    pub engine_arc: Arc<Mutex<Engine>>,
    socket: UdpSocket,
}

impl Connector {
    pub fn create(engine_arc: Arc<Mutex<Engine>>, host: String, port: i32) -> Connector {
        let address = format!("{}:{}", host, port);
        let socket = UdpSocket::bind(address.clone()).unwrap();
        info!("Game server is listening on address {}", address);
        let connector = Connector {
            engine_arc: engine_arc,
            socket: socket
        };

        connector
    }

    pub fn read_udp(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.socket.recv_from(buf)
    }

    pub fn send(&self, receiver: SocketAddr, payload: &[u8]) -> io::Result<usize> {
        self.socket.send_to(payload, receiver)
    }
}