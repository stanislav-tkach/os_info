use os_info::{OSType, OSInfo, OSVersion};

pub fn current_platform() -> OSInfo {
    OSInformation {
        os_type: OSType::Android,
        version: OSVersion::unknown(),
    }
}
