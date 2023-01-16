use mdbm::MdbExitCode;
use std::env::{args, Args};
use std::thread;
use std::time::Duration;
use std::{io, panic, process};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

mod http_server_adapter;

fn main_exitable() -> MdbExitCode {
    // Start firecracker by setting up a panic hook, which will be called before
    // terminating as we're building with panic = "abort".
    // It's worth noting that the abort is caused by sending a SIG_ABORT signal to the process.
    panic::set_hook(Box::new(move |info| {
        // We're currently using the closure parameter, which is a &PanicInfo, for printing the
        // origin of the panic, including the payload passed to panic! and the source code location
        // from which the panic originated.
        println!("MDB {}", info);
    }));

    http_server_adapter::run_web_server("main.py".to_string());

    // let term = Arc::new(AtomicBool::new(false));
    // signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term));
    // while !term.load(Ordering::Relaxed) {

    // } 


    // for x in 1..100 {
    //     println!("Main Thread is waiting ... till 100, now {}", x.to_string());
    //     thread::sleep(Duration::from_secs(2));
    // }

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

    let exit_code = main_exitable();
    std::process::exit(exit_code as i32);
}
