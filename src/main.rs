use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};
use sha1::{Sha1, Digest};
use base64::{encode, decode};

fn main() {

    println!("Starting MDB ...");

    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            println!("WS connection");

            handle_ws_new_connection(stream);
        }
    });

    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("/mdb/frontend/index.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    // --snip--
    } else {
        println!("Not GET");
    }
}

fn handle_ws_connection(mut stream: TcpStream) {
    
    
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Web Request line: {}",request_line);
    
    // create a Sha1 object
    let mut hasher = Sha1::new();    


    // process input message
    hasher.update(b"hello world");

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();    

    let base64Hash = encode(result);

    if request_line == "GET /chat HTTP/1.1" {
        println!("Client handshake request");


        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }


}


fn handle_ws_new_connection(mut stream: TcpStream) { 
    println!("WS new");
    let buf_reader = BufReader::new(&mut stream);
    for line in buf_reader.lines() {
        println!("WS new line {}", line.unwrap());
    }
}
