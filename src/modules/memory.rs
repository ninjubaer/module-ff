#![allow(dead_code)]
use sysinfo::System;
use crate::modules::module_trait::Module;
pub enum MemoryUnit {
    B,
    KB,
    MB,
    GB,
}
impl MemoryUnit {
    pub fn to_string(&self) -> String {
        match self {
            MemoryUnit::B => "B".to_string(),
            MemoryUnit::KB => "KB".to_string(),
            MemoryUnit::MB => "MB".to_string(),
            MemoryUnit::GB => "GB".to_string(),
        }
    }
    pub fn to_u64(&self) -> u64 {
        match self {
            MemoryUnit::B => 1,
            MemoryUnit::KB => 1024,
            MemoryUnit::MB => 1024 * 1024,
            MemoryUnit::GB => 1024 * 1024 * 1024,
        }
    }
    pub fn convert(from: MemoryUnit, to: MemoryUnit, value: u64) -> f64 {
        value as f64 * from.to_u64() as f64 / to.to_u64() as f64
    }
}

pub struct Memory {
    system: System,
}
impl Memory {
    pub fn new() -> Memory {
        let mut system = System::new();
        system.refresh_memory();
        Memory { system }
    }
    pub fn total(&self, unit: Option<MemoryUnit>) -> f64 {
        match unit {
            Some(unit) => MemoryUnit::convert(MemoryUnit::B, unit, self.system.total_memory()),
            None => self.system.total_memory() as f64,
            
        }
    }
    pub fn used(&self, unit: Option<MemoryUnit>) -> f64 {
        match unit {
            Some(unit) => MemoryUnit::convert(MemoryUnit::B, unit, self.system.used_memory()),
            None => self.system.used_memory() as f64,
        }
    }
    pub fn free(&self, unit: Option<MemoryUnit>) -> f64 {
        match unit {
            Some(unit) => MemoryUnit::convert(MemoryUnit::B, unit, self.system.total_memory() - self.system.used_memory()),
            None => self.system.total_memory() as f64 - self.system.used_memory() as f64,
        }
    }
    pub fn available(&self, unit: Option<MemoryUnit>) -> f64 {
        match unit {
            Some(unit) => MemoryUnit::convert(MemoryUnit::B, unit, self.system.total_memory() - self.system.used_memory() - self.system.total_swap()),
            None => self.system.total_memory() as f64 - self.system.used_memory() as f64 - self.system.total_swap() as f64,
        }
    }
    pub fn swap_total(&self, unit: Option<MemoryUnit>) -> f64 {
        match unit {
            Some(unit) => MemoryUnit::convert(MemoryUnit::B, unit, self.system.total_swap()),
            None => self.system.total_swap() as f64,
        }
    }
    pub fn swap_used(&self, unit: Option<MemoryUnit>) -> f64 {
        match unit {
            Some(unit) => MemoryUnit::convert(MemoryUnit::B, unit, self.system.used_swap()),
            None => self.system.used_swap() as f64,
        }
    }
    pub fn swap_free(&self, unit: Option<MemoryUnit>) -> f64 {
        match unit {
            Some(unit) => MemoryUnit::convert(MemoryUnit::B, unit, self.system.total_swap() - self.system.used_swap()),
            None => self.system.total_swap() as f64 - self.system.used_swap() as f64,
        }
    }
    pub fn handle(&self, memory_module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        let mut memory_stats: Vec<std::collections::HashMap<String, String>> = Vec::new();
        for stat in memory_module {
            let mut stat_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            stat_map.insert(stat.as_str().unwrap().to_string(), match stat.as_str().unwrap() {
                "total" => format!("{:.1} {}", self.total(Some(MemoryUnit::GB)), MemoryUnit::GB.to_string()),
                "used" => format!("{:.1} {}", self.used(Some(MemoryUnit::GB)), MemoryUnit::GB.to_string()),
                "free" => format!("{:.1} {}", self.free(Some(MemoryUnit::GB)), MemoryUnit::GB.to_string()),
                "available" => format!("{:.1} {}", self.available(Some(MemoryUnit::GB)), MemoryUnit::GB.to_string()),
                "swap_total" => format!("{:.1} {}", self.swap_total(Some(MemoryUnit::GB)), MemoryUnit::GB.to_string()),
                "swap_used" => format!("{:.1} {}", self.swap_used(Some(MemoryUnit::GB)), MemoryUnit::GB.to_string()),
                "swap_free" => format!("{:.1} {}", self.swap_free(Some(MemoryUnit::GB)), MemoryUnit::GB.to_string()),
                "percent" => format!("{:.1}%", self.used(None) / self.total(None) * 100.0),
                _ => "".to_string()
            });
            memory_stats.push(stat_map);
        }
        memory_stats
    }
}

impl Module for Memory {
    fn handle(&mut self, module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        Memory::handle(self, module)
    }
}