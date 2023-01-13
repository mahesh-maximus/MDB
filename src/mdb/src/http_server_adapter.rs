use std::process::{Command, Stdio};


pub fn run_http_server(tcp_proxy_path: String) {
    run_tcp_proxy(tcp_proxy_path);
    
}

fn run_tcp_proxy(tcp_proxy_path: String) {
    let echo_child = Command::new("python3")
        .arg(tcp_proxy_path.to_string())
        .current_dir("/mdb/tcp-proxy")
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start HTTP Proxy process.");
}
