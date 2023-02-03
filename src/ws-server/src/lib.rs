use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::thread::{self, spawn};
use std::{net::TcpListener, sync::Mutex, time};
use tungstenite::{accept, Message};
use serde::{Deserialize, Serialize};


//https://users.rust-lang.org/t/global-mutable-variables-in-rust/77056
//https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=820f36fe2c8faf6303a2b567faf47ac8
lazy_static! {
    static ref WS_SERVER: Mutex<WebSocketServerInner> = {
        let ws_server = WebSocketServerInner::new();
        Mutex::new(ws_server)
    };
    static ref SEND_QUEUE: Mutex<VecDeque<WebSocketMessage>> = {
        let send_queue = VecDeque::new();
        Mutex::new(send_queue)
    };
}

#[derive(Serialize, Deserialize)]
pub enum WebSocketMessageType {
    Ping,
    LiveReload,
}

#[derive(Serialize, Deserialize)]
pub struct WebSocketMessage {
    message_type: WebSocketMessageType,
    body: String,
}

pub struct WebSocketServer {}

impl WebSocketServer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bind_and_run(&mut self, address: String) {
        WS_SERVER.lock().unwrap().bind_and_run(address);
    }

    pub fn write_message(&mut self, message: WebSocketMessage) {
        WS_SERVER.lock().unwrap().write_message(message);
    }

    pub fn get_send_queue_len(&mut self) -> usize {
        WS_SERVER.lock().unwrap().get_send_queue_len()
    }
}

struct WebSocketServerInner {}

impl WebSocketServerInner {
    pub fn new() -> Self {
        println!("WebSocketServer.new fn .");
        WebSocketServerInner {}
    }

    fn bind_and_run(&mut self, address: String) {
        println!("WebSocketServer.bind_and_run fn.");

        thread::spawn(move || {
            let server = TcpListener::bind(address.to_string()).unwrap();
            println!("WebSocketServer.bind_and_run fn waiting for incoming ...");

            for stream in server.incoming() {
                spawn(move || {
                    let mut websocket = accept(stream.unwrap()).unwrap();

                    loop {
                        while SEND_QUEUE.lock().unwrap().len() > 0 {
                            let message_text = serde_json::to_string(
                                &SEND_QUEUE.lock().unwrap().pop_front().unwrap(),
                            )
                            .unwrap();

                            println!(
                                "WebSocketServer.bind_and_run, try to send message: '{}' to client",
                                message_text
                            );
                            websocket
                                .write_message(Message::Text(message_text))
                                .unwrap();
                            println!("WebSocketServer.bind_and_run, message sent to client.");
                        }

                        thread::sleep(time::Duration::from_millis(100));

                        //let msg = websocket.read_message().unwrap();

                        // if msg.is_binary() || msg.is_text() {
                        //     websocket.write_message(msg).unwrap();
                        // }
                    }
                });
            }
        });
    }

    pub fn get_send_queue_len(&mut self) -> usize {
        SEND_QUEUE.lock().unwrap().len()
    }

    pub fn write_message(&mut self, message: WebSocketMessage) {
        println!("WebSocketServer.write_message fn.");
        SEND_QUEUE.lock().unwrap().push_back(message);
    }
}

#[cfg(test)]
mod tests {
    use crate::{WebSocketMessage, WebSocketMessageType, WebSocketServer};
    use std::{thread, time};
    use tungstenite::connect;
    use url::Url;

    #[test]
    fn test_all() {
        let mut ws = WebSocketServer::new();
        ws.bind_and_run("localhost:7777".to_string());

        thread::sleep(time::Duration::from_millis(100));

        let (mut socket, response) =
            connect(Url::parse("ws://localhost:7777").unwrap()).expect("Can't connect WS server.");

        println!("Connected to the WS server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }

        thread::sleep(time::Duration::from_secs(1));

        let web_socket_message = WebSocketMessage {
            message_type: WebSocketMessageType::Ping,
            body: "body".to_string(),
        };
        ws.write_message(web_socket_message);

        println!(
            "_________________ : {}",
            ws.get_send_queue_len().to_string()
        );

        thread::sleep(time::Duration::from_secs(1));

        let msg = socket.read_message().expect("Error reading message");
        
        println!("Received******************************: {}", msg);

        thread::sleep(time::Duration::from_secs(1));
    }
}
