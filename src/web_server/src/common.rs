use std::fs;

use serde_json::Value;

pub fn get_content(file_name: String) -> String {
    println!("Response filename : {}", file_name);
    let content = fs::read_to_string(file_name).unwrap();

    let dev_config = fs::read_to_string(format!("{}{}", "/mdb/", "dev_config.json")).unwrap();
    println!("Dev JSON config path: {}", dev_config);
    let config: Value = serde_json::from_str(dev_config.as_str()).unwrap();
    println!("Dev json config BASE URI: {}", config["base_uri"]);
    let base_uri = &config["base_uri"].to_string().replace("\"", "");
    content.replace("#{config::base_uri}", base_uri)
}