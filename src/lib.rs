use std::process::Command;
use std::fs;
use std::convert::AsRef;
use std::path::Path;
mod lsb_release;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum OSType {
    Unknown,
    Redhat,
    OSX,
    Ubuntu
}

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(md) => md.is_dir() || md.is_file(),
        Err(_) => false
    }
}

fn is_os_x() -> bool {
    match Command::new("sw_vers").output() {
        Ok(output) => output.status.success(),
        Err(_) => false
    }
}

fn lsb_release() -> OSType {
    match lsb_release::from_file("/etc/lsb-release") {
        Ok(release) => {
            if release.distro == Some("Ubuntu".to_string()) {
                OSType::Ubuntu
            }
            else {
                OSType::Unknown
            }
        },
        Err(_) => OSType::Unknown
    }

}

pub fn current_platform() -> OSType {
    if is_os_x() {
        OSType::OSX
    }
    else if file_exists("/etc/lsb-release") {
        lsb_release()
    }
    else if file_exists("/etc/redhat-release") || file_exists("/etc/centos-release") {
        OSType::Redhat
    }
    else {
        OSType::Unknown
    }
}
