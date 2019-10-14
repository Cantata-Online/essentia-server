use std::net::UdpSocket;
use std::str;
use std::thread;

const BUFFER_SIZE:usize = 200;

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

                let (_number_of_bytes, src_addr) = socket.recv_from(&mut buf)
                                                .expect("Cannot receive data");
                println!("Got a message: {}", str::from_utf8(&buf).unwrap());
                let error_opt = socket.send_to(b"test", src_addr).err();
                if !error_opt.is_none() {
                    println!("Failed to send a message");
                }
                
            }
        });        
    }
}