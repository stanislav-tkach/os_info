use {Type, Info, Version};

// TODO: Somehow get the real OS version?
pub fn current_platform() -> Info {
    Info {
        os_type: Type::Emscripten,
        version: Version::unknown(),
    }
}
