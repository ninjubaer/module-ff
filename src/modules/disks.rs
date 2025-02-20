use crossterm::style::PrintStyledContent;
use sysinfo::{
	Disk as sysinfo_disk,
	Disks as sysinfo_disks,
};
use crate::modules::module_trait::Module;
pub enum DiskUnit {
	B,
	KB,
	MB,
	GB,
}
pub enum DiskType {
	HDD,
	SSD,
	NVME,
}
pub enum DiskState {
	Active,
	Inactive,
}
impl DiskUnit {
	pub fn to_string(&self) -> String {
		match self {
			DiskUnit::B => "B".to_string(),
			DiskUnit::KB => "KB".to_string(),
			DiskUnit::MB => "MB".to_string(),
			DiskUnit::GB => "GB".to_string(),
		}
	}
	pub fn to_u64(&self) -> u64 {
		match self {
			DiskUnit::B => 1,
			DiskUnit::KB => 1024,
			DiskUnit::MB => 1024 * 1024,
			DiskUnit::GB => 1024 * 1024 * 1024,
		}
	}
	pub fn convert(from: DiskUnit, to: DiskUnit, value: u64) -> f64 {
		value as f64 * from.to_u64() as f64 / to.to_u64() as f64
	}
}
impl DiskType {
	pub fn to_string(&self) -> String {
		match self {
			DiskType::HDD => "HDD".to_string(),
			DiskType::SSD => "SSD".to_string(),
			DiskType::NVME => "NVME".to_string(),
		}
	}
}
impl DiskState {
	pub fn to_string(&self) -> String {
		match self {
			DiskState::Active => "Active".to_string(),
			DiskState::Inactive => "Inactive".to_string(),
		}
	}
}

pub struct Disk {
	disks: sysinfo_disks,
}

impl Disk {
	pub fn new() -> Disk {
		let mut disks = sysinfo_disks::new();
		disks.refresh(false);
		Disk { disks }
	}
	pub fn handle(&mut self, disk_module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
		let mut disk_stats: Vec<std::collections::HashMap<String, String>> = Vec::new();
		for disk in self.disks.iter() {
			let mut disk_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
			for stat in disk_module {
				disk_map.insert(stat.as_str().unwrap().to_string(), match stat.as_str().unwrap() {
					"name" => disk.diskname(),
					"mount_point" => disk.mountpoint(),
					"file_system" => disk.filesystem(),
					"total" => disk.total(None).to_string(),
					"used" => disk.used(None).to_string(),
					"free" => disk.free(None).to_string(),
					"available" => disk.available(None).to_string(),
					"removable" => disk.removable().to_string(),
					_ => "".to_string()
				});
			}
			disk_stats.push(disk_map);
			let mut disk_map = std::collections::HashMap::new();
			disk_map.insert("".to_string(), "".to_string());
			disk_stats.push(disk_map);
		}
		disk_stats
	}
}
trait DiskTrait {
	fn diskname(&self) -> String;
	fn mountpoint(&self) -> String;
	fn filesystem(&self) -> String;
	fn total(&self, unit: Option<DiskUnit>) -> f64;
	fn used(&self, unit: Option<DiskUnit>) -> f64;
	fn free(&self, unit: Option<DiskUnit>) -> f64;
	fn available(&self, unit: Option<DiskUnit>) -> f64;
	fn removable(&self) -> bool;
}
impl DiskTrait for sysinfo_disk {
	fn diskname(&self) -> String {
		self.name().to_str().unwrap().to_string()
	}
	fn mountpoint(&self) -> String {
		self.mount_point().to_str().unwrap().to_string()
	}
	fn filesystem(&self) -> String {
		self.file_system().to_str().unwrap().to_string()
	}
	fn total(&self, unit: Option<DiskUnit>) -> f64 {
		match unit {
			Some(unit) => DiskUnit::convert(DiskUnit::B, unit, self.total_space()),
			None => self.total_space() as f64,
		}
	}
	fn used(&self, unit: Option<DiskUnit>) -> f64 {
		match unit {
			Some(unit) => DiskUnit::convert(DiskUnit::B, unit, self.total_space() - self.available_space()),
			None => (self.total_space() - self.available_space()) as f64,
		}
	}
	fn free(&self, unit: Option<DiskUnit>) -> f64 {
		match unit {
			Some(unit) => DiskUnit::convert(DiskUnit::B, unit, self.available_space()),
			None => self.available_space() as f64,
		}
	}
	fn available(&self, unit: Option<DiskUnit>) -> f64 {
		match unit {
			Some(unit) => DiskUnit::convert(DiskUnit::B, unit, self.available_space()),
			None => self.available_space() as f64,
		}
	}
	fn removable(&self) -> bool {
		self.is_removable()
	}
}

impl Module for Disk {
	fn handle(&mut self, module: &Vec<serde_json::Value>) -> Vec<std::collections::HashMap<String, String>> {
		Disk::handle(self, module)
	}
}