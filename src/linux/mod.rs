mod lsb_release;
mod file_release;
mod rhel_release;
mod utils;

use {Type, Info, Version};

pub fn current_platform() -> Info {
    if lsb_release::is_available() {
        lsb_release::lsb_release()
    } else if utils::file_exists("/etc/redhat-release") ||
               utils::file_exists("/etc/centos-release")
    {
        rhel_release::rhel_release()
    } else {
        Info {
            os_type: Type::Linux,
            version: Version::unknown(),
        }
    }
}
