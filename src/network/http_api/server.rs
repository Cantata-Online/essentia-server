extern crate hyper;

use std::net::{ToSocketAddrs};
use std::thread;
use std::sync::{Arc, Mutex};
use log::{error, info};

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use super::super::super::engine::engine::{Engine};
use super::super::super::system::error::{Error};

struct Handler {
    engine_arc: Arc<Mutex<Engine>>
}

impl Handler {
    fn create(engine_arc: Arc<Mutex<Engine>>) -> Handler {
        Handler{
            engine_arc: engine_arc
        }
    }

    fn handle(&self, req: Request<Body>) -> Response<Body> {
        if "POST" == req.method() && "/accounts" == req.uri() {
            let engine = self.engine_arc.lock().unwrap();
            engine.account_create();
            return Response::new(Body::from(format!("Registration")));
        }

        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(""))
            .unwrap()
    }
}

pub fn start(engine_arc: Arc<Mutex<Engine>>) -> Result<(), Error> {
    let engine = engine_arc.lock().unwrap();
    let http_api_config = &engine.configuration.server.http_api;
    let address_string = format!("{}:{}", http_api_config.host, http_api_config.port);
    let mut sock_addr = match address_string.to_socket_addrs() {
        Err(_) => Err(Error::create(format!("Invalid socket address: {}", address_string.clone()))),
        Ok(a) => Ok(a)
    }?;
    let sock_addr = sock_addr.next().unwrap();

    let engine_server_thread_arc = engine_arc.clone();
    thread::spawn(move || {
        info!("Starting HTTP API server on address {}...", address_string);
        let engine_http_handler_arc = engine_server_thread_arc.clone();
        let server = Server::bind(&sock_addr)
            .serve(move || {
                let handler = Handler::create(engine_http_handler_arc.clone());
                service_fn_ok(move |req: Request<Body>| {
                    handler.handle(req)
                })
            })
            .map_err(|e| error!("server error: {}", e));
        hyper::rt::run(server); 
    });
    Ok(())
}