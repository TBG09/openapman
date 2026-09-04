use std::cmp::PartialEq;
use std::fs;
use crate::model::{get_model_type, DeviceModel};

#[derive(Debug, Clone)]
pub enum BaseDistro {
    Debian,
    Arch,
    Fedora,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Distro {
    Ubuntu,
    Debian,
    Arch,
    Fedora,
    RaspberryPi,
    None,
}

#[derive(Debug, Clone)]
pub enum OsType {
    Linux {
        base: BaseDistro,
        distro: Distro,
        release: OsRelease,
    },
    Windows,
    Android,
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct OsRelease {
    pub pretty_name: String,
    pub name: String,
    pub version_id: String,
    pub version_name: String,
    pub version_codename: String,
    pub id: String,
    pub home_url: String,
    pub support_url: String,
    pub bug_report_url: String,
}


pub fn get_os_info() -> OsType {
    if cfg!(target_os = "linux") {
        let osdata = match fs::read_to_string("/etc/os-release") {
            Ok(text) => text,
            Err(e) => {
                println!("An unexpected error occurred: /etc/os-release was not found: {}", e);
                return OsType::Unknown;
            }
        };

        let mut pretty_name = String::new();
        let mut name = String::new();
        let mut version_id = String::new();
        let mut version_name = String::new();
        let mut version_codename = String::new();
        let mut id = String::new();
        let mut home_url = String::new();
        let mut support_url = String::new();
        let mut bug_report_url = String::new();

        for lne in osdata.lines() {
            let trimmed = lne.trim();

            if trimmed.is_empty() || trimmed.starts_with("#") {
                continue;
            }

            if let Some((key, value)) = trimmed.split_once("=") {
                let cvalue = value.trim_matches(|c| c == '"' || c == '\'');

                match key {
                    "PRETTY_NAME" => pretty_name = cvalue.to_string(),
                    "NAME" => name = cvalue.to_string(),
                    "VERSION_ID" => version_id = cvalue.to_string(),
                    "VERSION" => version_name = cvalue.to_string(),
                    "VERSION_CODENAME" => version_codename = cvalue.to_string(),
                    "ID" => id = cvalue.to_string(),
                    "HOME_URL" => home_url = cvalue.to_string(),
                    "SUPPORT_URL" => support_url = cvalue.to_string(),
                    "BUG_REPORT_URL" => bug_report_url = cvalue.to_string(),
                    _ => {}
                }
            }
        }

        let (base, mut distro) = match id.as_str() {
            "ubuntu" => (BaseDistro::Debian, Distro::Ubuntu),
            "debian" => (BaseDistro::Debian, Distro::Debian),
            "arch" => (BaseDistro::Arch, Distro::Arch),
            "fedora" => (BaseDistro::Fedora, Distro::Fedora),
            _ => (BaseDistro::None, Distro::None),
        };

        let release_info = OsRelease {
            pretty_name,
            name,
            version_id,
            version_name,
            version_codename,
            id,
            home_url,
            support_url,
            bug_report_url,
        };

        OsType::Linux {
            base,
            distro,
            release: release_info
        }
    } else if cfg!(target_os = "windows") {
        OsType::Windows
    } else if cfg!(target_os = "android") {
        OsType::Android
    } else {
        OsType::Unknown
    }
}
