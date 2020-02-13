use std::process::Command;

use log::trace;

use crate::{Bitness, Info, Matcher, Type, Version};

pub fn current_platform() -> Info {
    trace!("macos::current_platform is called");

    let info = Info {
        os_type: Type::Macos,
        version: version(),
        bitness: Bitness::Unknown,
    };
    trace!("Returning {:?}", info);
    info
}

fn version() -> Version {
    let version = match product_version() {
        None => {
            return Version::unknown();
        }
        Some(val) => val,
    };

    if let Some((major, minor, patch)) = parse_semantic_version(&version) {
        Version::semantic(major, minor, patch, None)
    } else {
        Version::custom(version, None)
    }
}

fn parse_semantic_version(version: &str) -> Option<(u64, u64, u64)> {
    let parts: Vec<_> = version.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }

    let major: u64 = parts[0].parse().ok()?;
    let minor: u64 = parts[1].parse().ok()?;
    let patch: u64 = parts.get(2).unwrap_or(&"0").parse().ok()?;
    Some((major, minor, patch))
}

fn product_version() -> Option<String> {
    match Command::new("sw_vers").output() {
        Ok(val) => {
            let output = String::from_utf8_lossy(&val.stdout);
            trace!("sw_vers command returned {:?}", output);
            parse(&output)
        }
        Err(e) => {
            trace!("sw_vers command failed with {:?}", e);
            None
        }
    }
}

fn parse(sw_vers_output: &str) -> Option<String> {
    Matcher::PrefixedVersion {
        prefix: "ProductVersion:",
    }
    .find(sw_vers_output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Macos, version.os_type());
    }

    #[test]
    fn os_version() {
        let version = version();
        assert_ne!(Version::unknown(), version);
    }

    #[test]
    fn string_product_version() {
        let version = product_version();
        assert!(version.is_some());
    }

    #[test]
    fn semantic_version() {
        let test_data = [
            ("", None),
            ("some test", None),
            ("0", None),
            ("0.", None),
            ("0.1", Some((0, 1, 0))),
            ("0.1.", None),
            ("0.1.2", Some((0, 1, 2))),
            ("0.1.2.", None),
            ("1.0.0", Some((1, 0, 0))),
            ("0.0.1", Some((0, 0, 1))),
            ("10.1", Some((10, 1, 0))),
            ("a.b.c", None),
            ("hello.world", None),
        ];

        for &(input, ref expected_result) in &test_data {
            let res = parse_semantic_version(input);
            assert_eq!(&res, expected_result);
        }
    }

    #[test]
    fn parse_version() {
        let parse_output = parse(sw_vers_output());
        assert_eq!(parse_output, Some("10.10.5".to_string()));
    }

    fn sw_vers_output() -> &'static str {
        "ProductName:	Mac OS X\n\
         ProductVersion:	10.10.5\n\
         BuildVersion:	14F27"
    }

    #[test]
    fn parse_beta_version() {
        let parse_output = parse(sw_vers_output_beta());
        assert_eq!(parse_output, Some("10.15".to_string()));
    }

    fn sw_vers_output_beta() -> &'static str {
        "ProductName:	Mac OS X\n\
         ProductVersion:	10.15\n\
         BuildVersion:	19A546d"
    }

    #[test]
    fn parse_double_digit_patch_version() {
        let parse_output = parse(sw_vers_output_double_digit_patch_version());
        assert_eq!(parse_output, Some("10.15.21".to_string()));
    }

    fn sw_vers_output_double_digit_patch_version() -> &'static str {
        "ProductName:	Mac OS X\n\
         ProductVersion:	10.15.21\n\
         BuildVersion:	ABCD123"
    }
}
