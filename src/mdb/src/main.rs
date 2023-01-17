use logger::{error, info, LOGGER};
use mdbm::MdbExitCode;
use std::ops::Deref;
use std::panic;

mod http_server_adapter;

fn main_exitable() -> MdbExitCode {
    if let Err(err) = LOGGER.deref().configure(Some("MDB->".to_string())) {
        println!("Could not configure the log subsystem: {}", err);
        return MdbExitCode::GenericError;
    }

    // Start firecracker by setting up a panic hook, which will be called before
    // terminating as we're building with panic = "abort".
    // It's worth noting that the abort is caused by sending a SIG_ABORT signal to the process.
    panic::set_hook(Box::new(move |info| {
        // We're currently using the closure parameter, which is a &PanicInfo, for printing the
        // origin of the panic, including the payload passed to panic! and the source code location
        // from which the panic originated.
        error!("MDB {}", info);
    }));

    http_server_adapter::run_web_server("main.py".to_string());

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
