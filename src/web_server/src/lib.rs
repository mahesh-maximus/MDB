use std::path::PathBuf;
use micro_http::{HttpServer, Response, StatusCode, Body, Version};

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
            for request in server.requests().unwrap() {
                println!("Request received");
                let response = request.process(|request| {
                    let mut response = Response::new(Version::Http11, StatusCode::OK);
                    let response_body = b"response body";
                    response.set_body(Body::new(response_body.to_vec()));
                    response
                });
                server.respond(response);
                println!("Sent response to client");
            }
        }
    }
}