use std::thread;
use std::fs;
use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::time::Duration;
use std::thread::spawn;
use tungstenite::accept;

pub fn process_requests() {
    println!("WS Reqiest processor");
    thread::spawn(|| {
        //  let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

        let server = TcpListener::bind("0.0.0.0:8000").unwrap();
        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                loop {
                    let msg = websocket.read_message().unwrap();

                    // We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        websocket.write_message(msg).unwrap();
                    }
                }
            });
        }
    });
}
