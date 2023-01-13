use mdbm::MdbExitCode;
use std::env::{args, Args};
use std::thread;
use std::time::Duration;

mod http_server_adapter;

fn main_exitable() -> MdbExitCode {
    http_server_adapter::run_http_server("main.py".to_string());
    MdbExitCode::Ok
}

fn main() {
    println!("Starting MDB ...");
    unsafe {
        // Harmless print to standard output.
        libc::syscall(
            libc::SYS_write,
            libc::STDOUT_FILENO,
            "Hello, world from Sys Call!\n",
            14,
        );
    }

    main_exitable();

    println!("Started MDB");

    // Hold on to dear life, don't let the main method exit.
    loop {
        println!("Main Thread is waiting ...");
        thread::sleep(Duration::from_secs(5));
    }
}
