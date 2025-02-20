#![allow(dead_code)]
use sysinfo::System;
use crate::modules::module_trait::Module;

pub struct Cpu {
    system: System,
}
impl Cpu {
    pub fn new() -> Cpu {
        let mut system = System::new();
        system.refresh_cpu_all();
        Cpu { system }
    }
    pub fn cpu_usage(&mut self) -> String {
        self.system.refresh_cpu_usage();
        std::thread::sleep(std::time::Duration::from_millis(200));
        self.system.refresh_cpu_usage();
        let cpu_usage: String = format!("{:.1}%", self.system.global_cpu_usage());
        cpu_usage
    }
    pub fn name(&self) -> String {
        self.system.cpus()[0].brand().trim().to_string()
    }
    pub fn cores(&self) -> u8 {
        self.system.physical_core_count().unwrap() as u8
    }
    pub fn frequency(&self) -> u32 {
        self.system.cpus()[0].frequency() as u32
    }
    pub fn threads(&self) -> u8 {
        self.system.cpus().len() as u8
    }
    pub fn architecture(&self) -> String {
        std::env::consts::ARCH.to_string()
    }
    pub fn vendor(&self) -> String {
        self.system.cpus()[0].vendor_id().to_string()
    }
    pub fn handle(&mut self, cpu_module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        let mut cpu_stats: Vec<std::collections::HashMap<String, String>> = Vec::new();
        for stat in cpu_module {
            let mut stat_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            stat_map.insert(stat.as_str().unwrap().to_string(), match stat.as_str().unwrap() {
                "name" => self.name(),
                "cores" => self.cores().to_string(),
                "threads" => self.threads().to_string(),
                "speed" => self.frequency().to_string(),
                "usage" => self.cpu_usage().to_string(),
                "vendor" => self.vendor(),
                "architecture" => self.architecture(),
                _ => "".to_string()
            });
            cpu_stats.push(stat_map);
        }
        cpu_stats
    }
}
impl Module for Cpu {
    fn handle(&mut self, module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        Cpu::handle(self, module)
    }
}