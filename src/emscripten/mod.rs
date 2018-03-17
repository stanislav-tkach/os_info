use {Info, Type, Version};

// TODO: Somehow get the real OS version?
pub fn current_platform() -> Info {
    Info {
        os_type: Type::Emscripten,
        version: Version::unknown(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Emscripten, *version.version());
    }
}
