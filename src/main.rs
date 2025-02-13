mod config;
mod modules;
mod chalk;
mod format;
use lazy_static::lazy_static;
use std::collections::HashMap;
use modules::cpu::Cpu;
use modules::module_trait::Module;
use chalk::Chalk;
use format::Format;

lazy_static!(
    static ref CONFIG: serde_json::Value = config::import_config();
);
fn main() {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    modules.insert("cpu".to_string(), Box::new(Cpu::new()));
    let mut data: Vec<Vec<HashMap<String, String>>> = Vec::new();
    //modules.insert("battery".to_string(), Box::new(Battery::new())); // Assuming Battery::new() exists

    for module in CONFIG["modules"].as_array().unwrap() {
        if let Some(module_name) = module["name"].as_str() {
            if let Some(module_instance) = modules.get_mut(module_name) {
                data.push(module_instance.handle(module.as_object().unwrap()["stats"].as_array().unwrap()));
            }
        }
    }
    for module in data {
        for stat in module {
            for (key, value) in stat {
                println!("{}", Format::center(Format::console_width(),&Format::stretch((Format::console_width() as f32 / 4.0 * 3.0) as u32, &Chalk::colorize(&key, CONFIG["theme"]["text"].as_u64().unwrap() as u32, false), &Chalk::colorize(&value, 0xC74DED, false))));
            }
        }
    }
}
