use os_info::{self, OSType, OSInformation};

// TODO: Somehow get the real OS version?
pub fn current_platform() -> OSInformation {
    OSInformation {
        os_type: OSType::Emscripten,
        version: os_info::unknown_version(),
    }
}
