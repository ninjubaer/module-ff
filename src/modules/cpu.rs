use lazy_static::lazy_static;
use sysinfo::System;
use crate::modules::module_trait::Module;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FILETIME {
    pub dw_low_date_time: u32,
    pub dw_high_date_time: u32,
}
extern "system" {
    fn GetSystemTimes(
        lpIdleTime: *mut FILETIME,
        lpKernelTime: *mut FILETIME,
        lpUserTime: *mut FILETIME,
    ) -> i32;
}
pub struct FileTimes {
    pub idle_time: FILETIME,
    pub kernel_time: FILETIME,
    pub user_time: FILETIME,
}
lazy_static! {
    #[derive(Debug)]
    pub static ref FILE_TIMES: FileTimes = {
        let mut idle = FILETIME { dw_low_date_time: 0, dw_high_date_time: 0 };
        let mut kernel = FILETIME { dw_low_date_time: 0, dw_high_date_time: 0 };
        let mut user = FILETIME { dw_low_date_time: 0, dw_high_date_time: 0 };
        unsafe {
            GetSystemTimes(&mut idle, &mut kernel, &mut user);
        }
        FileTimes {
            idle_time: idle,
            kernel_time: kernel,
            user_time: user,
        }
    };
}
pub struct Cpu {
    system: System,
}
impl Cpu {
    pub fn new() -> Cpu {
        let mut system = System::new();
        system.refresh_cpu_all();
        Cpu { system }
    }
    pub fn cpu_usage(&mut self) -> f32 {
        let mut idle = FILETIME {
            dw_low_date_time: 0,
            dw_high_date_time: 0,
        };
        let mut kernel = FILETIME {
            dw_low_date_time: 0,
            dw_high_date_time: 0,
        };
        let mut user = FILETIME {
            dw_low_date_time: 0,
            dw_high_date_time: 0,
        };
        unsafe {
            GetSystemTimes(&mut idle, &mut kernel, &mut user);
        }
        let kernel_time = (kernel.dw_high_date_time as u64) << 32 | kernel.dw_low_date_time as u64;
        let user_time = (user.dw_high_date_time as u64) << 32 | user.dw_low_date_time as u64;
        let idle_time = (idle.dw_high_date_time as u64) << 32 | idle.dw_low_date_time as u64;
        let total_time = kernel_time + user_time;
        let total_time_diff = total_time
            - FILE_TIMES.kernel_time.dw_high_date_time as u64
            - FILE_TIMES.kernel_time.dw_low_date_time as u64
            + FILE_TIMES.user_time.dw_high_date_time as u64
            + FILE_TIMES.user_time.dw_low_date_time as u64;
        let idle_time_diff = idle_time
            - FILE_TIMES.idle_time.dw_high_date_time as u64
            - FILE_TIMES.idle_time.dw_low_date_time as u64;
        let cpu_usage = 100.0 * (total_time_diff - idle_time_diff) as f64 / total_time_diff as f64;
        cpu_usage as f32
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
    pub fn handle(&self, cpu_module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        let mut cpu = Cpu::new();
        let mut cpu_stats: Vec<std::collections::HashMap<String, String>> = Vec::new();
        for stat in cpu_module {
            let mut stat_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            stat_map.insert(stat.as_str().unwrap().to_string(), match stat.as_str().unwrap() {
                "name" => cpu.name(),
                "cores" => cpu.cores().to_string(),
                "threads" => cpu.threads().to_string(),
                "speed" => cpu.frequency().to_string(),
                "usage" => cpu.cpu_usage().to_string(),
                "vendor" => cpu.vendor(),
                "architecture" => cpu.architecture(),
                _ => "".to_string()
            });
            cpu_stats.push(stat_map);
        }
        cpu_stats
    }
}
impl Module for Cpu {
    fn handle(&mut self, module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
        Cpu::handle(&self, module)
    }
}