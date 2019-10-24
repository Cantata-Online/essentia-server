extern crate hyper;

use std::net::{ToSocketAddrs};
use std::thread;
use log::{error, info};

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use super::super::super::system::configuration::{HttpApiServerConfiguration};

fn http_handler(req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(format!("{}; {}", req.method(), req.uri())))
}

pub fn start(configuration: HttpApiServerConfiguration) -> Result<(), String> {
    let address_string = format!("{}:{}", configuration.host, configuration.port);
    let sock_addr = address_string.to_socket_addrs()
        .expect(format!("Invalid socket address: {}", address_string.clone()).as_str())
        .next()
        .unwrap();

    let new_svc = || {
        service_fn_ok(http_handler)
    };
    let server = Server::bind(&sock_addr)
        .serve(new_svc)
        .map_err(|e| error!("server error: {}", e));

    info!("Starting HTTP API server on address {}...", address_string);
    thread::spawn(move || {
        hyper::rt::run(server);
    });
    Ok(())
}