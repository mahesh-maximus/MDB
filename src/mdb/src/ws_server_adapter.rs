use mdbm::MdbExitCode;
use ws_server::WS_SERVER;



pub fn run_ws_server() -> MdbExitCode {
    println!("run_ws_server fn");

    WS_SERVER.lock().unwrap().bind_and_run("0.0.0.0:8000".to_string());
    println!("run_ws_server fn -----------");


    MdbExitCode::Ok
}