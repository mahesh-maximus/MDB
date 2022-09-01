use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    str
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
    let mut  isHandshake = false;
    let mut  webSocketKey = String::new();
    for line in buf_reader.lines() {
        let uwLine = line.unwrap();
       // println!("WS new line {}", line.unwrap());
        if uwLine.starts_with("Sec-WebSocket-Key:") {        
            println!("Web Socket All  Key: {}__", uwLine);
            let abc: String = uwLine.trim().chars().skip(18).take(uwLine.len()).collect();
            webSocketKey = abc.to_string();
            println!("Web Socket Key: {}__", webSocketKey);
            break;
        } else if uwLine.starts_with("GET / HTTP/1.1") {
            println!("Handshake GET");
            isHandshake = true;
        } else {
           println!("Line : {}", uwLine);
        }
    }

    if(isHandshake) {
      //  println!("WSK: {}", webSocketKey);

        let mut hasher = Sha1::new();    
        hasher.update(webSocketKey.as_bytes());
        hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
        let result = hasher.finalize();   
        let key  = encode(result);

        let response = format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {key}\r\n\r\n",);
        
        println!("RESPONSE HKS: {}", response);

        stream.write_all(response.as_bytes()).unwrap();

    } else {
         println!("NOT Handshake");
    }

    
}
