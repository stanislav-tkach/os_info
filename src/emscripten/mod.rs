use os_info::{OSType, OSInfo, OSVersion};

// TODO: Somehow get the real OS version?
pub fn current_platform() -> OSInfo {
    OSInformation {
        os_type: OSType::Emscripten,
        version: OSVersion::unknown(),
    }
}
