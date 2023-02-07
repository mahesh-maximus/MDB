use mdbm::MdbExitCode;
use ws_server::WebSocketServer;



pub fn run_ws_server() -> MdbExitCode {
    println!("run_ws_server fn");

    let mut web_socket_server = WebSocketServer::new();
    web_socket_server.bind_and_run("0.0.0.0:8000".to_string());

    println!("run_ws_server fn -----------");


    MdbExitCode::Ok
}

#[cfg(test)]
mod test {
    #[test]
    fn test_temp() {
        assert!(true);
    }
}