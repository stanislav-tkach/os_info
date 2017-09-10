mod lsb_release;
mod rhel_release;
mod utils;

use os_info::{OSType, OSInfo, OSVersion};

pub fn current_platform() -> OSInfo {
    if lsb_release::is_available() {
        lsb_release::lsb_release()
    } else if utils::file_exists("/etc/redhat-release") ||
               utils::file_exists("/etc/centos-release")
    {
        rhel_release::rhel_release()
    } else {
        OSInfo {
            os_type: OSType::Linux,
            version: OSVersion::unknown(),
        }
    }
}
