use std::env::{args, Args};
use std::thread;
use std::time::Duration;

use net::ws_request_processor::WebSocketRequestProcessor;
use net::http_request_processor::HttpRequestProcessor;

fn main() {
    unsafe {
        // Harmless print to standard output.
        libc::syscall(libc::SYS_write, libc::STDOUT_FILENO, "Hello, world!\n", 14);
    }

    println!("Starting MDB ...");

    let mut args: Args = args();
    let first = args.nth(0);

    println!("{:?}", first);
   
    let mut http_request_processor = HttpRequestProcessor::new("0.0.0.0:3000".to_string());
    http_request_processor.print_address();
    http_request_processor.process_http_requests();
    
    let mut ws_request_processor = WebSocketRequestProcessor::new("0.0.0.0:8000".to_string());
    ws_request_processor.process_ws_requests();
    ws_request_processor.print_address();

    println!("Started MDB");

    // Hold on to dear life, don't let the main method exit.
    loop {
        println!("Main Thread is waiting ...");
        thread::sleep(Duration::from_secs(5));
    }
}
