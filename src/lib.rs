use std::process::Command;
use std::fs;
use std::convert::AsRef;
use std::path::Path;

#[derive(Debug)]
pub enum OSType {
    Unknown,
    Redhat,
    OSX
}

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(md) => md.is_dir() || md.is_file(),
        Err(_) => false
    }
}

fn is_os_x() -> bool {
    let output = Command::new("sw_vers").output().unwrap();
    output.status.success()
}

pub fn current_platform() -> OSType {
    if file_exists("/etc/redhat-release") || file_exists("/etc/centos-release") {
        OSType::Redhat
    } else if is_os_x() {
        OSType::OSX
    } else {
        OSType::Unknown
    }
}
