use std::thread::spawn;
use std::{
    fs,
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str, thread,
    time::Duration,
};

pub fn process_requests() {
    println!("HTTP Request Processor <<>>");

    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    });
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        // thread::sleep(Duration::from_secs(5));
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
