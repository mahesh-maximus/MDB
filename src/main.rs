use std::net::TcpListener;

fn main() {
    println!("$$$$$$$$$$$$$$$$$$$$$$$$$");
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

