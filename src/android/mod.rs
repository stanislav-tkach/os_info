use os_info::{self, OSType, OSInformation};

pub fn current_platform() -> OSInformation {
    OSInformation {
        os_type: OSType::Android,
        version: os_info::unknown_version(),
    }
}
