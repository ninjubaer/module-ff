use serde_json::json;
use std::{
    collections::HashMap, env, fs, path::Path
};
pub fn open_config() -> () {
    let localappdata = env::var("LOCALAPPDATA").unwrap();
    let config_path = Path::new(&localappdata).join("ninju_fastfestch");
    let config_file = config_path.join("config.json");
    // open conig file in default editor
    if config_file.exists() {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(&["/C", "start", "", config_file.to_str().unwrap()])
                .spawn()
                .expect("failed to open config file");
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(config_file)
                .spawn()
                .expect("failed to open config file");
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(config_file)
                .spawn()
                .expect("failed to open config file");
        }
    }
}
pub fn import_config() -> serde_json::Value {
    let mut localappdata: String;
    #[cfg(target_os="windows")]
    {
        localappdata = env::var("LOCALAPPDATA").unwrap();
    }
    #[cfg(not(target_os="windows"))]
    {
        localappdata = env::var("HOME").unwrap();
    }
    let config_path = Path::new(&localappdata).join("ninju_fastfestch");
    if !config_path.exists() {
        fs::create_dir_all(&config_path).unwrap();
    }
    let config_file = config_path.join("config.json");
    if !config_file.exists() {
        let default_config = json!({
            "version": "0.1.0",
            "theme": {
                "gradient": {
                    "from": 0xffffff,
                    "to": 0x000000
                },
                "headers": 0x9FA1C9,
                "text": {"key":0xF5C2E7,"value":0x75AFFA},
                "width": 0.5,
                "align": "left"
            },
            "modules": [
                {
                    "name": "CPU",
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
                },
                {
                    "icon": "efc5",
                    "name": "Memory",
                    "stats": [
                        "total",
                        "used",
                        "free",
                        "available",
                        "swap_total",
                        "swap_used",
                        "swap_free",
                        "percent"
                    ]
                }
            ]
        });
        fs::write(&config_file, default_config.to_string()).unwrap();
    }
    let config_content = fs::read_to_string(&config_file).unwrap();
    serde_json::from_str(&config_content).unwrap()
}

pub struct ArgHandler {
    pub args: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Args {
    Help,
    Version,
    Config,
    Width,
    Align,
    Invalid
}

impl ArgHandler {
    pub fn new() -> ArgHandler {
        ArgHandler {
            args: env::args().collect(),
        }
    }
    pub fn handle(&self) -> Vec<HashMap<Args, String>> {
        let mut args = Vec::new();
        for arg in self.args.iter().skip(1) {
            let argname = arg.split('=').collect::<Vec<&str>>()[0];
            args.push(match argname {
                "-h" | "--help" => {
                    let mut map = HashMap::new();
                    map.insert(Args::Help, "".to_string());
                    map
                }
                "-v" | "--version" => {
                    let mut map = HashMap::new();
                    map.insert(Args::Version, "".to_string());
                    map
                }
                "-c" | "--config" => {
                    let mut map = HashMap::new();
                    map.insert(Args::Config, "".to_string());
                    map
                }
                "-w" | "--width" => {
                    if !arg.contains('=') {
                        let mut map = HashMap::new();
                        map.insert(Args::Help, "".to_string());
                        return vec![map];
                    }
                    let mut map = HashMap::new();
                    map.insert(Args::Width, arg.split('=').collect::<Vec<&str>>()[1].to_string());
                    map
                }
                "-a" | "--align" => {
                    if !arg.contains('=') {
                        let mut map = HashMap::new();
                        map.insert(Args::Help, "".to_string());
                        return vec![map];
                    }
                    let mut map = HashMap::new();
                    map.insert(Args::Align, arg.split('=').collect::<Vec<&str>>()[1].to_string());
                    map
                }
                _ => {
                    let mut map = HashMap::new();
                    map.insert(Args::Invalid, arg.to_string());
                    map
                }
            })
        }
        args
    }
}