extern crate rustc_serialize;
use std::thread;
use std::sync::{Arc, Mutex};
use rouille::Request;
use rouille::Response;
use rouille::Server;
use rouille::{try_or_400};

use log::{info};

use super::super::super::engine::engine::{Engine};
use super::super::super::system::error::{Error};
use super::request::account::{AccountCreateRequest};

struct Handler {
    engine_arc: Arc<Mutex<Engine>>
}


impl Handler {
    fn create(engine_arc: Arc<Mutex<Engine>>) -> Handler {
        Handler{
            engine_arc: engine_arc
        }
    }

    fn handle(&self, req: &Request) -> Response {
        let engine = self.engine_arc.lock().unwrap();
        if "POST" == req.method() && "/accounts" == req.url() {
            let request_struct: AccountCreateRequest = try_or_400!(rouille::input::json_input(req));
            let account = request_struct.to_engine_struct();
            let response = match engine.account_create(account) {
                Ok(_) => Response::text(format!("Login is {}", request_struct.login)),
                Err(e) => Response::text(format!("An error occurred: {}", e.message)),
            };
            return response;
        }
        Response::empty_404()
    }
}

pub fn start(engine_arc: Arc<Mutex<Engine>>) -> Result<(), Error> {
    let address_string = {
        let engine = engine_arc.lock().unwrap();
        let http_api_config = &engine.configuration.server.http_api;
        format!("{}:{}", http_api_config.host, http_api_config.port)
    };

    let engine_server_thread_arc = engine_arc.clone();
    thread::spawn(move || {
        let engine_http_handler_arc = engine_server_thread_arc.clone();
        let server = Server::new(address_string.clone(), move |request| {
            let handler = Handler::create(engine_http_handler_arc.clone());
            handler.handle(&request)
        });
        info!("Starting HTTP API server on address {}...", address_string);
        server.unwrap().run();
    });
    Ok(())
}