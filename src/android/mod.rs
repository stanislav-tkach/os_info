use {Info, Type, Version};

pub fn current_platform() -> Info {
    Info {
        os_type: Type::Android,
        version: Version::unknown(),
    }
}
