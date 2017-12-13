mod lsb_release;
mod file_release;

use {Info, Type, Version};

pub fn current_platform() -> Info {
    if lsb_release::is_available() {
        lsb_release::lsb_release()
    } else {
        match file_release::retrieve(file_release::distributions()) {
            Some(release) => Info {
                os_type: release.os_type,
                version: release
                    .version
                    .map(|x| Version::custom(x, None))
                    .unwrap_or_else(Version::unknown),
            },
            None => Info {
                os_type: Type::Linux,
                version: Version::unknown(),
            },
        }
    }
}
