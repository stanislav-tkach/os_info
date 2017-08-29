use regex::Regex;

use std::process::Command;

use os_info::{self, OSInformation, OSType};

// TODO: Better matching.
pub fn lsb_release() -> OSInformation {
    match retrieve() {
        Some(release) => {
            if release.distro == Some("Ubuntu".to_string()) {
                OSInformation {
                    os_type: OSType::Ubuntu,
                    version: release.version.unwrap_or_else(os_info::unknown_version),
                }
            } else if release.distro == Some("Debian".to_string()) {
                OSInformation {
                    os_type: OSType::Debian,
                    version: release.version.unwrap_or_else(os_info::unknown_version),
                }
            } else if release.distro == Some("Arch".to_string()) {
                OSInformation {
                    os_type: OSType::Arch,
                    version: release.version.unwrap_or_else(os_info::unknown_version),
                }
            } else if release.distro == Some("CentOS".to_string()) {
                OSInformation {
                    os_type: OSType::Centos,
                    version: release.version.unwrap_or_else(os_info::unknown_version),
                }
            } else {
                OSInformation {
                    os_type: OSType::Linux,
                    version: os_info::unknown_version(),
                }
            }
        }
        None => OSInformation {
            os_type: OSType::Linux,
            version: os_info::unknown_version(),
        },
    }
}

struct LsbRelease {
    pub distro: Option<String>,
    pub version: Option<String>,
}

fn retrieve() -> Option<LsbRelease> {
    let output = match Command::new("lsb_release").arg("-a").output() {
        Ok(o) => o,
        Err(_) => return None,
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(parse(&stdout))
}

pub fn is_available() -> bool {
    match Command::new("lsb_release").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn parse(file: &str) -> LsbRelease {
    let distrib_regex = Regex::new(r"Distributor ID:\s(\w+)").unwrap();
    let distrib_release_regex = Regex::new(r"Release:\s([\w\.]+)").unwrap();

    let distro = match distrib_regex.captures_iter(file).next() {
        Some(m) => {
            match m.get(1) {
                Some(distro) => Some(distro.as_str().to_owned()),
                None => None,
            }
        }
        None => None,
    };

    let version = match distrib_release_regex.captures_iter(file).next() {
        Some(m) => {
            match m.get(1) {
                Some(version) => Some(version.as_str().to_owned()),
                None => None,
            }
        }
        None => None,
    };

    LsbRelease {
        distro: distro,
        version: version,
    }
}
