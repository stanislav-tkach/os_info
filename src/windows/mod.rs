mod version;
use {Type, Info, Version};

pub fn current_platform() -> Info {
    let version = version::Win32Version::osvi();
    let version = match version.osvi {
        Some(v) => Version::semantic(
            v.dwMajorVersion as u64,
            v.dwMinorVersion as u64,
            v.dwBuildNumber  as u64,
            version.edition
        ),
        None    => Version::unknown(),
    };

    Info {
        os_type: Type::Windows,
        version,
    }
}
