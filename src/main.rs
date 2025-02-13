mod chalk;
mod config;
mod format;
mod modules;
use chalk::Chalk;
use format::Format;
use lazy_static::lazy_static;
use modules::cpu::Cpu;
use modules::module_trait::Module;
use std::collections::HashMap;

lazy_static! {
    static ref CONFIG: serde_json::Value = config::import_config();
}
fn main() {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    modules.insert("CPU".to_string(), Box::new(Cpu::new()));
    let mut data: Vec<Vec<HashMap<String, String>>> = Vec::new();
    //modules.insert("battery".to_string(), Box::new(Battery::new())); // Assuming Battery::new() exists

    for module in CONFIG["modules"].as_array().unwrap() {
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
                (Format::console_width() as f64 * CONFIG["theme"]["width"].as_f64().unwrap()) as u32,
                &Chalk::colorize(
                    &format!(
                        "{}  {}",
                        char::from_u32(
                            u32::from_str_radix(
                                CONFIG["modules"][index]["icon"].as_str().unwrap(),
                                16
                            )
                            .unwrap()
                        )
                        .unwrap(),
                        &CONFIG["modules"][index]["name"].as_str().unwrap(),
                    ),
                    CONFIG["theme"]["headers"].as_u64().unwrap() as u32,
                    true,
                ),
            )
        ));
        for stat in module {
            for (key, value) in stat {
                str.push_str(&format!(
                    "{}\n",
                    &Format::stretch(
                        (Format::console_width() as f64 * CONFIG["theme"]["width"].as_f64().unwrap()) as u32,
                        &Chalk::colorize(
                            &key,
                            CONFIG["theme"]["text"]["key"].as_u64().unwrap() as u32,
                            false,
                        ),
                        &Chalk::colorize(
                            &value,
                            CONFIG["theme"]["text"]["value"].as_u64().unwrap() as u32,
                            false,
                        ),
                    )
                ));
            }
        }
    }
    for str in str.lines() {
        println!("{}", match CONFIG["theme"]["align"].as_str().unwrap() {
            "center" => Format::center(Format::console_width(), str),
            "right" => Format::right(Format::console_width(), str),
            _ => Format::left(Format::console_width(), str),
            
        })
    }
}
