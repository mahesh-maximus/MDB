use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use std::io::{prelude::*, BufReader};
use crate::login_processor::LoginProcessor;

pub struct HttpRequestProcessor {
    address: String,
}

impl HttpRequestProcessor {
    pub fn new(address: String) -> Self {
        Self {
            address,
        }
    }

    pub fn print_address(&mut self) {
        println!("Tcp address for Http requests: {}", self.address); 
    } 

    pub fn process_http_requests(&mut self) {
        println!("HTTP Request Processor <<>>");
        
        let tcp_address = self.address.to_string();

        thread::spawn(|| {
            let listener = TcpListener::bind(tcp_address).unwrap();

            for stream in listener.incoming() {
                thread::spawn(||{
                    let stream = stream.unwrap();
                 Self::handle_connection(stream);
                });
            }
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        for request_item in &http_request {
            println!("Request item: {request_item}");
        }      

        println!("First request item HTTP method: {}", http_request[0]);

        println!("Request: {:#?}", http_request);

        let mut login_processor  = LoginProcessor::new("mahesh".to_string(), "123".to_string());
        login_processor.validate_username_password();
        
        let cookie_index =  http_request.iter().position(|r| r.starts_with("Cookie: ")).unwrap_or(0);
        
        println!("Cockie index: {}", cookie_index.to_string());

        if cookie_index > 0 {
            println!("Cookies aviable: {}", http_request[cookie_index]);
        } else {
            println!("Cookie not found");
        }

        let (status_line, filename) = if http_request[0] == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "index.html")
        } else if http_request[0] == "GET /sleep HTTP/1.1" {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let file_name = format!("{}{}", "/mdb/frontend/", filename);
        println!("Response filename : {}", file_name);

        let contents = fs::read_to_string(file_name).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
