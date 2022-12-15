use std::env::{args, Args};
use std::thread;
use std::time::Duration;

use net::http_request_processor::process_http_requests;
use net::ws_request_processor::process_ws_requests;


fn main() {
    println!("Starting MDB ...");

    let mut args: Args = args();
    let first = args.nth(0);

    println!("{:?}", first);
    
    net::http_request_processor::process_http_requests();
    
    net::ws_request_processor::process_ws_requests();

    println!("Started MDB");

    // Hold on to dear life, don't let the main method exit.
    loop {
        println!("Main Thread is waiting ...");
        thread::sleep(Duration::from_secs(5));
    }
}
