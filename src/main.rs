use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    str,
    io::{Read, Result, Write}
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

            process(stream);
        }
    });


    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    


}

fn handle_connection(mut stream: TcpStream) {
   /*
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
    }*/

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let baseFrontendPath = "/mdb/frontend/";
    let a = format!("{}{}", baseFrontendPath, filename);
    println!("zzzzzzzzzzzzzzzzzzz : {}", a);
    let contents = fs::read_to_string(a).unwrap();
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();  
}

fn process(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 80 * 1024];
    let len = stream.read(&mut buf)?;
    let data = &buf[..len];

    if data.starts_with(b"GET / HTTP/1.1") {
        let req = String::from_utf8(data.to_vec()).unwrap();

        println!("Received:\n\n{req}");

        let sec_key = req
            .lines()
            .filter_map(|line| line.split_once(": "))
            .find_map(|(key, value)| key.contains("Sec-WebSocket-Key").then_some(value))
            .unwrap();

        let res = response(sec_key);

        println!("Sending Responce:\n\n{res}");
        stream.write_all(res.as_bytes())?;

        // ----------------------------------------------------
            // std::thread::sleep_ms(2000);
            
    }
    Ok(())
}

pub fn response(key: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut m = Sha1::new();
    m.update(key.as_bytes());
    m.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11"); // Magic string
    let key = base64::encode(m.finalize());

    format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {key}\r\n\r\n",)
}

pub fn apply_mask<const S: usize>(keys: [u8; S], payload: &mut [u8]) {
    payload
        .iter_mut()
        .zip(keys.into_iter().cycle())
        .for_each(|(p, m)| *p ^= m);
}

