use {Info, Type, Version};

pub fn current_platform() -> Info {
    Info {
        os_type: Type::Android,
        version: Version::unknown(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Android, version.os_type());
    }
}
