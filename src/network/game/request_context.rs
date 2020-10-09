use std::net::SocketAddr;

use crate::network::game::UdpRequestBuffer;
use crate::network::game::connector::Connector;

pub struct RequestContext<'a> {
    pub buffer: UdpRequestBuffer,
    pub connector: &'a Connector,
    pub src_addr: SocketAddr,
}

impl<'a> RequestContext<'a> {
    pub fn new(buffer: UdpRequestBuffer, connector: &Connector, src_addr: SocketAddr) -> RequestContext {
        RequestContext {
            buffer: buffer,
            connector: connector,
            src_addr: src_addr,
        }
    }
}