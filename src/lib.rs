use std::process::Command;
use std::fs;
use std::convert::AsRef;
use std::path::Path;
mod lsb_release;

///A list of supported operating system types
#[derive(Debug)]
#[derive(PartialEq)]
pub enum OSType {
    Unknown,
    Redhat,
    OSX,
    Ubuntu,
    Debian,
    Windows
}

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(md) => md.is_dir() || md.is_file(),
        Err(_) => false
    }
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
            }
            else {
                OSType::Unknown
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
    else if file_exists("/etc/redhat-release") || file_exists("/etc/centos-release") {
        OSType::Redhat
    }
    else {
        OSType::Unknown
    }
}
