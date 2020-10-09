pub mod connector;
pub mod packets;
pub mod packet_codes;
pub mod request_context;
pub mod server_udp;

const BUFFER_SIZE:usize = 200;

type UdpRequestBuffer = [u8; BUFFER_SIZE];