// use std::thread;
// use std::net::TcpListener;
// use std::thread::spawn;
// use tungstenite::accept;

// pub struct WebSocketRequestProcessor {
//     address: String,
// }


// impl WebSocketRequestProcessor {
//     pub fn new(address: String) -> Self {
//         Self {
//             address,
//         }
//     }

//     pub fn print_address(&mut self) {
//         println!("Web Socket address: {}", self.address);
//     }

//     pub fn process_ws_requests(&mut self) {
//         println!("WS Reqiest processor");
        
//         let tcp_address = self.address.to_string();

//         thread::spawn(|| {
//             let server = TcpListener::bind(tcp_address).unwrap();
//             for stream in server.incoming() {
//                 spawn(move || {
//                     let mut websocket = accept(stream.unwrap()).unwrap();
//                     loop {
//                         let msg = websocket.read_message().unwrap();

//                         // We do not want to send back ping/pong messages.
//                         if msg.is_binary() || msg.is_text() {
//                             websocket.write_message(msg).unwrap();
//                         }
//                     }
//                 });
//             }
//         });
//     }

// }
