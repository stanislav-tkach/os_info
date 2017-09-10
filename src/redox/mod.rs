use os_info::{OSType, OSInfo, OSVersion};

pub fn current_platform() -> OSInformation {
    OSInformation {
        os_type: OSType::Redox,
        version: OSVersion::unknown(),
    }
}
