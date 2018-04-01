// spell-checker:ignore uname

use std::{fs::File, io::Read};

use {Info, Type, Version};

pub fn current_platform() -> Info {
    let version = get_version().map_or_else(|| Version::unknown(), |v| Version::custom(v, None));
    Info {
        os_type: Type::Redox,
        version,
    }
}

fn get_version() -> Option<String> {
    let mut file = File::open("sys:uname").ok()?;
    let mut version = String::new();
    file.read_to_string(&mut version).ok()?;
    Some(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Redox, version.os_type());
    }
}
