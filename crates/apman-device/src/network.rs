use std::fs::read_dir;
use crate::distro::{get_os_info, OsType};

pub fn get_network_devices() -> Vec<String> {
    let mut devices = Vec::new();
    if let OsType::Linux { .. } = get_os_info() {
        let mut netdir = read_dir("/sys/class/net/");
        for entry in netdir.unwrap() {
            let entry = entry.unwrap();
            let name: String = entry.file_name().to_string_lossy().into_owned();

            devices.push(name);
        }
    } else {
        return devices
    }

    return devices;
}
// amazing naming, I know right?
pub fn remove_unimportant_devices(mut devices: Vec<String>) -> Vec<String> {
    devices.retain(|device| !device.starts_with("br-") && device != "lo");
    devices
}
