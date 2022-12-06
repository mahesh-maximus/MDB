use std::thread::spawn;
use std::{
    fs,
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    str, thread,
    time::Duration,
};
use tungstenite::accept;
mod http_request_processor;
mod ws_request_processor;

fn main() {
    println!("Starting MDB ...");

    ws_request_processor::process_requests();

    http_request_processor::process_requests();

    println!("Started MDB");

    loop {
        println!("Main Thread is waiting ...");
        thread::sleep(Duration::from_secs(5));
    }
}
