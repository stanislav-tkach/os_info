use os_info::{OSType, OSInfo, OSVersion};

pub fn current_platform() -> OSInfo {
    OSInfo {
        os_type: OSType::Windows,
        version: OSVersion::unknown(),
    }
}
