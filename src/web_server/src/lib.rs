use std::path::PathBuf;
use logger::info;


pub struct WebServer {
    temp: String,
}

impl WebServer {
    pub fn new(temp: String) -> Self {
        Self {
            temp,
        }
    }

    pub fn bind_and_run(
        &mut self,
        path: &PathBuf
    ) {
        println!("{}", self.temp.to_string());
        info!("Unix socket path: {}", path.display());


    }
}

