use std::fs;
use crate::distro::{OsType, get_os_info};

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceModel {
    RaspberryPi4(String),
    Other(String),
    Unknown,
}
impl DeviceModel {
    pub fn as_str(&self) -> &str {
        match self {
            DeviceModel::RaspberryPi4(raw) => raw,
            DeviceModel::Other(raw) => raw,
            DeviceModel::Unknown => "Unknown Device / Non-Linux",
        }
    }
}

pub fn get_model_type() -> DeviceModel {
    if let OsType::Linux { .. } = get_os_info() {
        let model_path = "/proc/device-tree/model";

        let model_name = match fs::read_to_string(model_path) {
            Ok(text) => text,
            Err(e) => {
                println!("An unexpected error occurred: {} was not found: {}", model_path, e);
                return DeviceModel::Unknown;
            }
        };

        let cname = model_name
            .replace('\0', "")
            .trim()
            .to_string();

        if cname.is_empty() {
            return DeviceModel::Unknown;
        }

        if cname.starts_with("Raspberry Pi 4 Model") {
            DeviceModel::RaspberryPi4(cname)
        } else {
            DeviceModel::Other(cname)
        }
    } else {
        DeviceModel::Unknown
    }
}
