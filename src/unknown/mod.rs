use {Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("unknown::current_platform is called");
    Info::unknown()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Unknown, version.os_type());
        assert_eq!(Version::unknown(), version.version());
    }
}
