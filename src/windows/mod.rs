mod version;
use {Type, Info, Version};

pub fn current_platform() -> Info {
    let version = match version::retrieve() {
        Some(v) => Version::semantic(
            v.dwMajorVersion as u64,
            v.dwMinorVersion as u64,
            v.dwBuildNumber  as u64,
            Some(String::from(""))
        ),
        None    => Version::unknown(),
    };

    Info {
        os_type: Type::Windows,
        version,
    }
}
