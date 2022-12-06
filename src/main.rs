use std::{
    fs,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    str,
    io::{Read, Result, Write}
};
use std::thread::spawn;
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
