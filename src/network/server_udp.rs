use std::net::UdpSocket;
use std::str;

pub struct Listener {

}

impl Listener {
    pub fn create() -> Listener {
        Listener{}
    }

    pub fn start(&self) {
        let mut socket = UdpSocket::bind("127.0.0.1:2300").unwrap();

        let mut buf = [0; 10];

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
                                        .expect("Didn't receive data");
        println!("Got a message: {}", str::from_utf8(&buf).unwrap());
    }
}