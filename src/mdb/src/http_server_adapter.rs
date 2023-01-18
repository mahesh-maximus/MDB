use mdbm::MdbExitCode;
use std::thread;
use std::{
    io,
    path::PathBuf,
    process::{Child, Command, Stdio},
};
use web_server::WebServer;

pub fn run_web_server(tcp_proxy_path: String) -> MdbExitCode {
    let mut tcp_proxy_child_process = run_tcp_proxy(tcp_proxy_path);

    let exit_thread = thread::Builder::new()
        .name("mm_http_server".to_owned())
        .spawn(move || {
            println!("press ANY key to exit.");
            let mut user_input = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut user_input).unwrap();
            println!("Console input received to exit: {} ", user_input);
            println!("Killing TCP Proxy");
            tcp_proxy_child_process
                .kill()
                .expect("Failed to kill TCP Proxy");
                println!("Exiting MDB ...");
            std::process::exit(mdbm::MdbExitCode::GenericError as i32);
        })
        .expect("Web Server thread spawn failed.");

    let web_server_thread = thread::Builder::new()
        .name("mm_http_server".to_owned())
        .spawn(move || {
            WebServer::new("temp".to_string()).bind_and_run(&PathBuf::from("/tmp/mdb.socket"));
        })
        .expect("Web Server thread spawn failed.");

    // This call to thread::join() should block until the API thread has processed the
    // shutdown-internal and returns from its function.
    exit_thread.join().unwrap();
    web_server_thread.join().unwrap();

    MdbExitCode::Ok
}

fn run_tcp_proxy(tcp_proxy_path: String) -> Child {
    let tcp_proxy_child_process = Command::new("python3")
        .arg(tcp_proxy_path.to_string())
        .current_dir("/mdb/tcp-proxy")
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Failed to start TCP Proxy.");

    tcp_proxy_child_process
}
