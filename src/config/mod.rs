use serde_json::json;
use std::{
    env, fs, path::Path
};

pub fn import_config() -> serde_json::Value {
    let localappdata = env::var("LOCALAPPDATA").unwrap();
    let config_path = Path::new(&localappdata).join("ninju_fastfestch");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }
    let config_file = config_path.join("config.json");
    if !config_file.exists() {
        let default_config = json!({
            "theme": {
                "gradient": {
                    "from": 0xffffff,
                    "to": 0x000000
                },
                "headers": 0xffffff,
                "text": 0xffffff,
            },
            "modules": [
                {
                    "name": "cpu",
                    "icon": "f4bc",
                    "stats": [
                        "name",
                        "cores",
                        "threads",
                        "speed",
                        "usage",
                        "vendor",
                        "architecture"
                    ]
                }
            ]
        });
        fs::write(&config_file, default_config.to_string()).unwrap();
    }
    let config_content = fs::read_to_string(&config_file).unwrap();
    serde_json::from_str(&config_content).unwrap()
}