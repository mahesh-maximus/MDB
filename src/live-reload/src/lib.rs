use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use ws_server::{WebSocketServer, WebSocketMessageType, WebSocketMessage};
use std::{path::PathBuf, sync::mpsc, thread, time::Duration};

pub struct LiveReload {
    watch_directory: String,
}

impl LiveReload {
    pub fn new(watch_directory: String) -> Self {
        Self { watch_directory }
    }

    pub fn start_to_watch(&mut self) {
        self.watch();
    }

    fn watch(&mut self) {
        let watch_directory = self.watch_directory.clone();
        println!("LiveReload.watch watching directory: {}", watch_directory);

        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();
            debouncer
                .watcher()
                .watch(
                    &PathBuf::from(watch_directory.to_string()),
                    RecursiveMode::Recursive,
                )
                .unwrap();

            for events in rx {
                for e in events.unwrap() {
                    println!("LiveReload.watch : {:?}", e);
                    break;
                }
                println!("LiveReload.watch call WS ...");
                let mut web_socket = WebSocketServer::new();
                let web_socket_message = WebSocketMessage {
                    message_type: WebSocketMessageType::LiveReload,
                    body: "Reload".to_string(),
                };
                web_socket.write_message(web_socket_message);
            }
        });
    }
}
