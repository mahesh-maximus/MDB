use lazy_static::lazy_static;
use std::thread::{self, spawn};
use std::{net::TcpListener, sync::Mutex};
use tungstenite::accept;

//https://users.rust-lang.org/t/global-mutable-variables-in-rust/77056
//https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=820f36fe2c8faf6303a2b567faf47ac8
lazy_static! {
    pub static ref WS_SERVER: Mutex<WebSocketServer> = {
        let ws_server = WebSocketServer::new(false);
        Mutex::new(ws_server)
    };
}

pub struct WebSocketServer {
    example: bool,
}

impl WebSocketServer {
    pub fn new(example: bool) -> Self {
        println!("WebSocketServer.new fn .");
        WebSocketServer { example }
    }

    pub fn bind_and_run(&mut self, address: String) {
        println!(
            "WebSocketServer.bind_and_run fn {}.",
            self.example.to_string()
        );

        thread::spawn(move || {
            let server = TcpListener::bind(address.to_string()).unwrap();
            println!("WebSocketServer.bind_and_run fn waiting for incoming ...");
            for stream in server.incoming() {
                spawn(move || {
                    let mut websocket = accept(stream.unwrap()).unwrap();
                    loop {
                        let msg = websocket.read_message().unwrap();

                        if msg.is_binary() || msg.is_text() {
                            websocket.write_message(msg).unwrap();
                        }
                    }
                });
            }
        });
    }
}
