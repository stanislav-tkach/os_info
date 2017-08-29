mod lsb_release;
mod rhel_release;
mod utils;

use os_info::{self, OSType, OSInformation};

pub fn current_platform() -> OSInformation {
    if lsb_release::is_available() {
        lsb_release::lsb_release()
    } else if utils::file_exists("/etc/redhat-release") ||
               utils::file_exists("/etc/centos-release")
    {
        rhel_release::rhel_release()
    } else {
        OSInformation {
            os_type: OSType::Linux,
            version: os_info::unknown_version(),
        }
    }
}
