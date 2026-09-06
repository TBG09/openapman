use crate::distro::{OsType, get_os_info};
use std::cmp::PartialEq;
use std::fs::{canonicalize, read_dir, read_to_string};
use std::path::PathBuf;

pub enum NetworkDevice {
    Wireless,
    Ethernet,
    Virtual,
    Unknown,
}
#[derive(Debug, PartialEq)]
pub enum VirtualDeviceType {
    Docker,
    Loopback,
    Unknown,
    None,
}
pub struct NetworkDeviceDescriptor {
    pub name: String,
    pub plugged_in: bool,
    pub network_device_path: String,
    pub network_device_type: NetworkDevice,
    pub virtual_device_type: VirtualDeviceType,
    pub state: String,
    pub address: String,
    pub speed: i16,
    pub device_path: String, // path to the actual device interface
    pub flags: String,
}

pub fn get_network_devices() -> Vec<NetworkDeviceDescriptor> {
    let mut devices = Vec::new();
    if let OsType::Linux { .. } = get_os_info() {
        let netdirbuf = PathBuf::from("/sys/class/net/");
        let netdir = read_dir(&netdirbuf);
        for entry in netdir.unwrap() {
            let entry_unwrap = entry.unwrap();
            let iface_name = entry_unwrap.file_name().into_string().unwrap_or_default();

            let devicepathbuf = netdirbuf.join(&iface_name);
            let devicepath = devicepathbuf.display().to_string();
            let name = iface_name.clone();

            let mut statef = devicepathbuf.clone();
            statef.push("operstate");
            let state = read_to_string(statef)
                .unwrap_or("unknown".to_string())
                .trim()
                .to_string();
            let addr = read_to_string(devicepathbuf.join("address"))
                .unwrap_or("00:00:00:00:00:00".to_string())
                .trim()
                .to_string();
            let speed_str = read_to_string(devicepathbuf.join("speed")).unwrap_or("-1".to_string());
            let speed: i16 = speed_str.trim().parse().unwrap_or(-1);
            let flags_str =
                read_to_string(devicepathbuf.join("flags")).unwrap_or("0x0".to_string());
            let flags = flags_str.trim().to_string();

            let carrier = read_to_string(devicepathbuf.join("carrier")).unwrap_or_default();
            let carrierb = carrier.trim() == "1";
            let mut devpathb = devicepathbuf.clone();
            devpathb.push("device");
            let devpath = match canonicalize(devpathb) {
                Ok(p) => p.display().to_string(),
                Err(_) => "None".to_string(),
            };
            let is_physical = devpath != "None";

            let virtual_device_type = if !is_physical {
                if iface_name == "lo" {
                    VirtualDeviceType::Loopback
                } else if iface_name.contains("docker") || iface_name.contains("br-") {
                    VirtualDeviceType::Docker
                } else {
                    VirtualDeviceType::Unknown
                }
            } else {
                VirtualDeviceType::None
            };

            let mut network_device_type = if iface_name.starts_with("wlan") {
                NetworkDevice::Wireless
            } else if iface_name.starts_with("eth") {
                NetworkDevice::Ethernet
            } else {
                NetworkDevice::Unknown
            };

            if virtual_device_type != VirtualDeviceType::None {
                network_device_type = NetworkDevice::Virtual;
            }

            devices.push(NetworkDeviceDescriptor {
                name,
                plugged_in: carrierb,
                network_device_path: devicepath,
                network_device_type: network_device_type,
                virtual_device_type,
                state,
                address: addr,
                speed,
                device_path: devpath,
                flags,
            });
        }
    } else {
        return devices;
    }

    return devices;
}

// amazing naming, I know right?
pub fn remove_unimportant_devices(
    mut devices: Vec<NetworkDeviceDescriptor>,
) -> Vec<NetworkDeviceDescriptor> {
    devices.retain(|device| match device.virtual_device_type {
        VirtualDeviceType::Docker | VirtualDeviceType::Loopback => false,
        VirtualDeviceType::Unknown | VirtualDeviceType::None => true,
    });
    devices
}
