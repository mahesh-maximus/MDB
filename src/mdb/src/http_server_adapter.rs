use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};

use http_server::HttpServer;

pub fn run_http_server(tcp_proxy_path: String) {
    run_tcp_proxy(tcp_proxy_path);

    let http_server_thread = thread::Builder::new()
        .name("mm_http_server".to_owned())
        .spawn(move || {
            HttpServer::new("temp".to_string()).bind_and_run(&PathBuf::from("/tmp/mdb.socket"));
        });

    // This call to thread::join() should block until the API thread has processed the
    // shutdown-internal and returns from its function.
    //http_server_thread.join().unwrap();
}

fn run_tcp_proxy(tcp_proxy_path: String) {
    let echo_child = Command::new("python3")
        .arg(tcp_proxy_path.to_string())
        .current_dir("/mdb/tcp-proxy")
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start HTTP Proxy process.");
}
