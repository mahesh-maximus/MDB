use std::path::PathBuf;
pub use micro_http::{
    Body, HttpServer, Method, Request, RequestError, Response, ServerError, ServerRequest,
    ServerResponse, StatusCode, Version,
};

pub struct HttpServer {
    
}

impl HttpServer {
   pub fn new() -> Self

    pub fn bind_and_run(
        &mut self,
        path: &PathBuf,
        api_payload_limit: usize
    } {
        
    }

}

