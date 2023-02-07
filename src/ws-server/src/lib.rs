use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::thread::{self};
use std::time::Duration;
use std::{net::TcpListener, sync::Mutex};
use tungstenite::{accept, Message};

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
    static ref READ_QUEUE: Mutex<VecDeque<String>> = {
        let read_queue = VecDeque::new();
        Mutex::new(read_queue)
    };
}

#[derive(Serialize, Deserialize)]
pub enum WebSocketMessageType {
    Ping,
    LiveReload,
}

#[derive(Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: WebSocketMessageType,
    pub body: String,
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

    pub fn get_read_queue_len(&mut self) -> usize {
        WS_SERVER.lock().unwrap().get_read_queue_len()
    }
}

struct WebSocketServerInner {}

impl WebSocketServerInner {
    fn new() -> Self {
        println!("WebSocketServer.new fn .");
        WebSocketServerInner {}
    }

    fn bind_and_run(&mut self, address: String) {
        println!("WebSocketServer.bind_and_run fn.");

        thread::spawn(move || {
            let server = TcpListener::bind(address.to_string()).unwrap();
            println!("WebSocketServer.bind_and_run fn waiting for incoming ...");

            for stream in server.incoming() {
                println!("WebSocketServer.bind_and_run fn, got the incoming client connection ...");
                let stream = stream.unwrap();
                let read_timeout_result = stream.set_read_timeout(Some(Duration::from_secs(10)));
                if read_timeout_result.is_err() {
                    panic!("WebSocketServer.bind_and_run fn, We have panic now !!!, since we cannot set read timeout.");
                }

                let write_timeout_result = stream.set_write_timeout(Some(Duration::from_secs(10)));
                if write_timeout_result.is_err() {
                    panic!("WebSocketServer.bind_and_run fn, We have panic now !!!, since we cannot set write timeout.");
                }

                let mut websocket = accept(stream).unwrap();
                let mut retry_count = 0;
                loop {
                    while SEND_QUEUE.lock().unwrap().len() > 0 {
                        println!("WebSocketServer.bind_and_run, number of messages in the send queue is {}.", SEND_QUEUE.lock().unwrap().len().to_string());
                        let message_text =
                            serde_json::to_string(&SEND_QUEUE.lock().unwrap().pop_front().unwrap())
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

                    println!("WebSocketServer.bind_and_run, try to read ping from client {}/5.", retry_count.to_string()) ;
                    let message_from_client_result = websocket.read_message();
                    if message_from_client_result.is_err() {
                        println!(
                            "WebSocketServer.bind_and_run, message read from client err: {}",
                            message_from_client_result.err().unwrap().to_string()
                        );
                        if retry_count >= 4 {
                            println!("WebSocketServer.bind_and_run, message read 5/5 attempts exhausted !!!.");
                            break;
                        }
                        retry_count = retry_count + 1;
                        continue;
                    }

                    retry_count = 0;
                    READ_QUEUE
                        .lock()
                        .unwrap()
                        .push_back(message_from_client_result.unwrap().to_string());

                    println!("WebSocketServer.bind_and_run, received PING from client.");
                }
                println!("WebSocketServer.bind_and_run, since message read attempt count exhausted, waiting for a new incoming client request.");
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

    fn get_read_queue_len(&mut self) -> usize {
        READ_QUEUE.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{WebSocketMessage, WebSocketMessageType, WebSocketServer};
    use std::{thread, time};
    use tungstenite::{connect, Message};
    use url::Url;

    #[test]
    fn test_all() {
        let mut web_socket = WebSocketServer::new();
        web_socket.bind_and_run("localhost:7777".to_string());

        let (mut socket, response) =
            connect(Url::parse("ws://localhost:7777").unwrap()).expect("Can't connect WS server.");

        println!("Connected to the WS server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }

        let web_socket_message = WebSocketMessage {
            message_type: WebSocketMessageType::Ping,
            body: "This is the PING Body".to_string(),
        };
        web_socket.write_message(web_socket_message);
        assert_eq!(web_socket.get_send_queue_len(), 1);
        println!(
            "Send queue length: {}",
            web_socket.get_send_queue_len().to_string()
        );

        let msg = socket.read_message().expect("Error reading message");
        assert!(!msg.is_empty());
        println!("Received message from server: {}", msg);

        socket
            .write_message(Message::Text("PING".to_string()))
            .unwrap();
        thread::sleep(time::Duration::from_secs(1));
        println!(
            "Read queue length: {}",
            web_socket.get_read_queue_len().to_string()
        );
        assert_eq!(web_socket.get_read_queue_len(), 1);

        thread::sleep(time::Duration::from_secs(60));
    }
}
