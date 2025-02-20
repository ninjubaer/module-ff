mod chalk;
mod config;
mod format;
mod modules;
use chalk::Chalk;
use format::Format;
use lazy_static::lazy_static;
use modules::cpu::Cpu;
use modules::disks::Disk;
use modules::memory::Memory;
use modules::battery::Battery;
use modules::module_trait::Module;
use serde_json;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIG: RwLock<serde_json::Value> = RwLock::new(config::import_config());
    static ref HELP: serde_json::Value = serde_json::json!({
        "width": {
            "args": ["-w", "--width"],
            "description": "Change the width of the output in the terminal.\nThe Format is a float:\n0.0 - 1.0 (0% - 100%)",
            "example": "`-w=0.5` or `--width=0.5`"
        },
        "align": {
            "args": ["-a", "--align"],
            "description": "Change the alignment of the output in the terminal.\nThe Format is a string:\nleft, right, center",
            "example": "`-a=left` or `--align=left`"
        },
        "config": {
            "args": ["-c", "--config"],
            "description": "Change the configuration of the program.\nThis will open the configuration file in your default text editor.",
            "example": "`-c` or `--config`"
        },
        "version": {
            "args": ["-v", "--version"],
            "description": "Print the version of the program to the terminal.",
            "example": "`-v` or `--version`"
        }
    });
}

fn display_help() -> String {
    let mut string = String::new();
    for (key, value) in HELP.as_object().unwrap() {
        string.push_str(&format!(
            "{}\n",
            &Chalk::colorize(
                &format!(
                    "{} [ {} ]",
                    &key,
                    value["args"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(|arg| arg.as_str())
                        .collect::<Vec<&str>>()
                        .join(" | ")
                ),
                CONFIG.read().unwrap()["theme"]["headers"].as_u64().unwrap() as u32,
                true,
            ),
        ));
        for line in value["description"].as_str().unwrap().lines() {
            string.push_str(&format!(
                "   {}\n",
                &Chalk::colorize(
                    line,
                    CONFIG.read().unwrap()["theme"]["text"]["key"].as_u64().unwrap() as u32,
                    false,
                ),
            ));
        }
        string.push_str("\n");
    }
    string

}

fn main() {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    modules.insert("CPU".to_string(), Box::new(Cpu::new()));
    modules.insert("Memory".to_string(), Box::new(Memory::new()));
    modules.insert("Battery".to_string(), Box::new(Battery::new()));
    modules.insert("Disks".to_string(), Box::new(Disk::new()));
    let mut data: Vec<Vec<HashMap<String, String>>> = Vec::new();
    //modules.insert("battery".to_string(), Box::new(Battery::new())); // Assuming Battery::new() exists

    let arghandler = config::ArgHandler::new();
    let args = arghandler.handle();
    if args.len() > 0 {
        for arg in args {
            if arg.contains_key(&config::Args::Help) {
                let string = display_help();
                println!("{}", string);
                return;
            }
            else if arg.contains_key(&config::Args::Version) {
                println!("version {}", CONFIG.read().unwrap()["version"].as_str().unwrap());
                return;
            }
            else if arg.contains_key(&config::Args::Config) {
                config::open_config();
                return;
            }
            else if arg.contains_key(&config::Args::Width) {
                let width = arg.get(&config::Args::Width).unwrap();
                if let Ok(width) = width.parse::<f64>() {
                    if width > 0.0 && width <= 1.0 {
                        CONFIG.write().unwrap()["theme"]["width"] = serde_json::Value::from(width);
                    }
                }
            }
            else if arg.contains_key(&config::Args::Align) {
                let align = arg.get(&config::Args::Align).unwrap().as_str();
                if align == "left" || align == "right" || align == "center" {
                    CONFIG.write().unwrap()["theme"]["align"] = serde_json::Value::from(align);
                }
            }
            else {
                println!("\x1b[1;31m{} is not a valid argument.\x1b[0m\nTo get help use ff -h", arg.get(&config::Args::Invalid).unwrap());
                return;                
            }
        }
    }
    for module in CONFIG.read().unwrap()["modules"].as_array().unwrap() {
        if let Some(module_name) = module["name"].as_str() {
            if let Some(module_instance) = modules.get_mut(module_name) {
                data.push(
                    module_instance
                        .handle(module.as_object().unwrap()["stats"].as_array().unwrap()),
                );
            }
        }
    }
    let mut str = String::new();
    for (index, module) in data.iter().enumerate() {
        str.push_str(&format!(
            "{}\n",
            &Format::center(
                (Format::console_width() as f64 * CONFIG.read().unwrap()["theme"]["width"].as_f64().unwrap())
                    as u32,
                &Chalk::colorize(
                    &format!(
                        "{}  {}",
                        char::from_u32(
                            u32::from_str_radix(
                                CONFIG.read().unwrap()["modules"][index]["icon"].as_str().unwrap(),
                                16
                            )
                            .unwrap()
                        )
                        .unwrap(),
                        &CONFIG.read().unwrap()["modules"][index]["name"].as_str().unwrap(),
                    ),
                    CONFIG.read().unwrap()["theme"]["headers"].as_u64().unwrap() as u32,
                    true,
                ),
            )
        ));
        for stat in module {
            for (key, value) in stat {
                str.push_str(&format!(
                    "{}\n",
                    &Format::stretch(
                        (Format::console_width() as f64
                            * CONFIG.read().unwrap()["theme"]["width"].as_f64().unwrap())
                            as u32,
                        &Chalk::colorize(
                            &key,
                            CONFIG.read().unwrap()["theme"]["text"]["key"].as_u64().unwrap() as u32,
                            false,
                        ),
                        &Chalk::colorize(
                            &value,
                            CONFIG.read().unwrap()["theme"]["text"]["value"].as_u64().unwrap() as u32,
                            false,
                        ),
                    )
                ));
            }
        }
    }
    for str in str.lines() {
        println!(
            "{}",
            match CONFIG.read().unwrap()["theme"]["align"].as_str().unwrap() {
                "center" => Format::center(Format::console_width(), str),
                "right" => Format::right(Format::console_width(), str),
                _ => Format::left(Format::console_width(), str),
            }
        )
    }
}
