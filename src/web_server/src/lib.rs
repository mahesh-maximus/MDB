mod parsed_request;
mod request;
use micro_http::{HttpServer, Request, Response};
use std::path::PathBuf;

use crate::parsed_request::ParsedRequest;

pub mod common;

pub struct WebServer {
    temp: String,
}

impl WebServer {
    pub fn new(temp: String) -> Self {
        Self { temp }
    }

    pub fn bind_and_run(&mut self, path: &PathBuf) {
        println!("{}", self.temp.to_string());
        println!("Unix socket path: {}", path.display());

        std::fs::remove_file(path).unwrap_or_default();

        let mut server = HttpServer::new(path).unwrap();
        server.start_server().unwrap();
        println!("Started micro http server");
        loop {
            let request_vec = match server.requests() {
                Ok(vec) => vec,
                Err(err) => {
                    // print request error, but keep server running
                    println!("API Server error on retrieving incoming request: {}", err);
                    continue;
                }
            };
            for server_request in request_vec {
                let request_processing_start_us =
                    utils::time::get_time_us(utils::time::ClockType::Monotonic);
                server.respond(
                    // Use `self.handle_request()` as the processing callback.
                    server_request.process(|request| self.handle_request(request)),
                );

                let delta_us = utils::time::get_time_us(utils::time::ClockType::Monotonic)
                    - request_processing_start_us;
                println!("Total previous API call duration: {} us.", delta_us);
            }
        }
    }

    pub fn handle_request(&mut self, request: &Request) -> Response {
        ParsedRequest::try_from_request(request)
    }
}
 