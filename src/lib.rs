extern crate regex;

use std::process::Command;
mod lsb_release;
mod windows_ver;
mod rhel_release;
mod utils;

///A list of supported operating system types
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum OSType {
    Unknown,
    Redhat,
    OSX,
    Ubuntu,
    Debian,
    Windows,
    Arch,
    CentOS
}

fn is_windows() -> bool {
    if cfg!(target_os="windows") {
        return true;
    } else {
        return false;
    }
}

fn is_os_x() -> bool {
    match Command::new("sw_vers").output() {
        Ok(output) => output.status.success(),
        Err(_) => false
    }
}

fn lsb_release() -> OSType {
    match lsb_release::retrieve() {
        Some(release) => {
            if release.distro == Some("Ubuntu".to_string()) {
                OSType::Ubuntu
            }
            else if release.distro == Some("Debian".to_string()) {
                OSType::Debian
            } else if release.distro == Some("Arch".to_string()) {
                OSType::Arch
            }
            else if release.distro == Some("CentOS".to_string()){
                OSType::CentOS
            }
            else {
                OSType::Unknown
            }
        },
        None => OSType::Unknown
    }
}

fn rhel_release() -> OSType {
    match rhel_release::retrieve() {
        Some(release) => {
            if release.distro == Some("CentOS".to_string()) {
                OSType::CentOS
            } else {
                OSType::Redhat
            }
        },
        None => OSType::Unknown
    }
}

///Returns the current operating system type
///
///#Example
///
///```
///use os_type;
///let os = os_type::current_platform();
///```
pub fn current_platform() -> OSType {
    if is_os_x() {
        OSType::OSX
    }
    else if is_windows() {
        OSType::Windows
    }
    else if lsb_release::is_available() {
        lsb_release()
    }
    else if utils::file_exists("/etc/redhat-release") || utils::file_exists("/etc/centos-release") {
        rhel_release()
    }
    else {
        OSType::Unknown
    }
}
