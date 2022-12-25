pub struct LoginProcessor {
    username: String,
    password: String,
}

impl LoginProcessor { 
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
        }
    }

    pub fn validate_username_password(&mut self) {
        println!("validating useanem & password: {}, {}", self.username.to_string(), self.password.to_string());
    }
}

pub fn test() { 
    println!("auth ___ test");
}
