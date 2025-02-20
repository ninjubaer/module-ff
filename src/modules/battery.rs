#![allow(dead_code)]
use battery::Manager;
use crate::modules::module_trait::Module;

pub struct Battery {
    manager: Manager,
    battery: battery::Battery,
}
impl Battery {
    pub fn new() -> Battery {
        let manager = Manager::new().unwrap();
        let battery = manager.batteries().unwrap().next().unwrap().unwrap();
        Battery { manager, battery }
    }
    pub fn percentage(&self) -> String {
        format!(
            "{:.1}%{}",
            self.battery.state_of_charge().value * 100.0,
            if self.battery.state() == battery::State::Charging {
                " "
            } else {
                ""
            }
        )
    }
    pub fn status(&self) -> String {
        self.battery.state().to_string()
    }
    pub fn time_to_full(&self) -> String {
        match self.battery.time_to_full() {
            Some(time) => {
                let duration = std::time::Duration::from_secs(time.value as u64);
                let hours = duration.as_secs() / 3600;
                let minutes = (duration.as_secs() % 3600) / 60;
                format!("{:02}:{:02}", hours, minutes)
            },
            None => "".to_string()
        }
    }
    pub fn time_to_empty(&self) -> String {
        match self.battery.time_to_empty() {
            Some(time) => {
                let duration = std::time::Duration::from_secs(time.value as u64);
                let hours = duration.as_secs() / 3600;
                let minutes = (duration.as_secs() % 3600) / 60;
                format!("{:02}:{:02}", hours, minutes)
            }
            None => "".to_string()
        }
    }
    pub fn energy(&self) -> f32 {
        self.battery.energy().value
    }
    pub fn energy_full(&self) -> f32 {
        self.battery.energy_full().value
    }
    pub fn energy_full_design(&self) -> f32 {
        self.battery.energy_full_design().value
    }
    pub fn energy_rate(&self) -> f32 {
        self.battery.energy_rate().value
    }
    pub fn handle(&mut self, battery_module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        if !(self.manager.batteries().unwrap().count() > 0) {
            return vec![std::collections::HashMap::new()];
        }
        let mut battery_stats: Vec<std::collections::HashMap<String, String>> = Vec::new();
        for stat in battery_module {
            let mut stat_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            stat_map.insert(stat.as_str().unwrap().to_string(), match stat.as_str().unwrap() {
                "percent" => self.percentage(),
                "status" => self.status(),
                "time" => {
                    if self.status() == "charging" {
                        self.time_to_full()
                    } else {
                        self.time_to_empty()
                    }
                }
                "energy" => format!("{:.1} Wh", self.energy()),
                "energy_full" => format!("{:.1} Wh", self.energy_full()),
                "energy_full_design" => format!("{:.1} Wh", self.energy_full_design()),
                "energy_rate" => format!("{:.1} W", self.energy_rate()),
                _ => "".to_string()
            });
            battery_stats.push(stat_map);
        }
        battery_stats
    }
}

impl Module for Battery {
    fn handle(&mut self, module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        Battery::handle(self, module)
    }
}