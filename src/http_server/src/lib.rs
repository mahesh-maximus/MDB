use std::path::PathBuf;



pub struct HttpServer {
    temp: String,
}

impl HttpServer {
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
        println!("Unix socket path: {}", path.display());


    }
}

