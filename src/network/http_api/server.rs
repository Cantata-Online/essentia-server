extern crate hyper;

use std::net::{ToSocketAddrs};
use std::thread;
use log::{error, info};

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use super::super::super::system::configuration::{Configuration};

fn http_handler(req: Request<Body>) -> Response<Body> {
    if "POST" == req.method() && "/accounts" == req.uri() {
        return Response::new(Body::from(format!("Registration")));
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(""))
        .unwrap()
}

pub fn start(configuration: Configuration) -> Result<(), String> {
    let http_api_config = configuration.server.http_api;
    let address_string = format!("{}:{}", http_api_config.host, http_api_config.port);
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