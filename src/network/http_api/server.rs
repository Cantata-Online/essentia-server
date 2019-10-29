extern crate hyper;

use std::net::{ToSocketAddrs};
use std::thread;
use log::{error, info};

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

use super::super::super::engine::engine::{Engine};
use super::super::super::system::error::{Error};

struct Handler<'a> {
    engine: &'a Engine
}

impl Handler<'_> {
    fn create(engine: &Engine) -> Handler {
        Handler{
            engine: engine
        }
    }

    fn handle(&self, req: Request<Body>) -> Response<Body> {
        if "POST" == req.method() && "/accounts" == req.uri() {
            self.engine.account_create();
            return Response::new(Body::from(format!("Registration")));
        }

        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(""))
            .unwrap()
    }
}

pub fn start(engine: &'static Engine) -> Result<(), Error> {
    let http_api_config = &engine.configuration.as_ref().unwrap().server.http_api;
    let address_string = format!("{}:{}", http_api_config.host, http_api_config.port);
    let mut sock_addr = match address_string.to_socket_addrs() {
        Err(_) => Err(Error::create(format!("Invalid socket address: {}", address_string.clone()))),
        Ok(a) => Ok(a)
    }?;
    let sock_addr = sock_addr.next().unwrap();

    thread::spawn(move || {
        info!("Starting HTTP API server on address {}...", address_string);
        let server = Server::bind(&sock_addr)
            .serve(move || {
                let handler = Handler::create(engine);
                service_fn_ok(move |req: Request<Body>| {
                    handler.handle(req)
                })
            })
            .map_err(|e| error!("server error: {}", e));
        hyper::rt::run(server); 
    });
    Ok(())
}