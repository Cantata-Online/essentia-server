use std::net::UdpSocket;
use std::str;

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
        self.socket = Some(UdpSocket::bind("127.0.0.1:2300").unwrap());
        let socket = self.socket.as_ref().unwrap();

        let mut buf = [0; 10];

        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
                                        .expect("Didn't receive data");
        socket.send_to(b"test", src_addr);
        println!("Got a message: {}", str::from_utf8(&buf).unwrap());
        
    }
}