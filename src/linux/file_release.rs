use regex::Regex;

use std::{fs::File, path::Path, io::Read};

use {Info, Type, Version};

pub fn get() -> Option<Info> {
    retrieve(&DISTRIBUTIONS)
}

fn retrieve(distributions: &[ReleaseInfo]) -> Option<Info> {
    for release_info in distributions {
        if !Path::new(release_info.path).exists() {
            continue;
        }

        let mut file = match File::open(&release_info.path) {
            Ok(val) => val,
            Err(_) => continue,
        };

        let mut file_content = String::new();
        if let Err(_) = file.read_to_string(&mut file_content) {
            continue;
        }

        let version = if !release_info.version_regex.is_empty() {
            let version_regex = Regex::new(release_info.version_regex).unwrap();

            version_regex
                .captures_iter(&file_content)
                .next()
                .and_then(|c| c.get(1))
                .map(|v| v.as_str().trim_right().to_owned())
        } else {
            Some(file_content.trim_right().to_string())
        }.map(|x| Version::custom(x, None))
            .unwrap_or_else(Version::unknown);

        return Some(Info::new(release_info.os_type, version));
    }

    None
}

struct ReleaseInfo {
    os_type: Type,
    path: &'static str,
    version_regex: &'static str,
}

/// List of all supported distributions and the information on how to parse their version from the
/// release file.
const DISTRIBUTIONS: [ReleaseInfo; 4] = [
    ReleaseInfo {
        os_type: Type::Centos,
        path: "/etc/centos-release",
        version_regex: r"release\s([\w\.]+)",
    },
    ReleaseInfo {
        os_type: Type::Fedora,
        path: "/etc/fedora-release",
        version_regex: r"release\s([\w\.]+)",
    },
    ReleaseInfo {
        os_type: Type::Redhat,
        path: "/etc/redhat-release",
        version_regex: r"release\s([\w\.]+)",
    },
    ReleaseInfo {
        os_type: Type::Alpine,
        path: "/etc/alpine-release",
        version_regex: "",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_file_centos() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/centos-release");

        let mut distributions = [DISTRIBUTIONS[0]];
        distributions.path = file.into_os_string().into_string().unwrap();

        let info = retrieve(distros).unwrap();
        assert_eq!(Type::Centos, version.os_type());
        assert_eq!(result.version, Version::custom("XX".to_string(), None));
    }

    #[test]
    fn test_file_fedora() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/fedora-release");

        let mut distributions = [DISTRIBUTIONS[1]];
        distributions.path = file.into_os_string().into_string().unwrap();

        let info = retrieve(distros).unwrap();
        assert_eq!(Type::Fedora, version.os_type());
        assert_eq!(result.version, Version::custom("26".to_string(), None));
    }

    #[test]
    fn test_file_redhat() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/redhat-release");

        let mut distributions = [DISTRIBUTIONS[2]];
        distributions.path = file.into_os_string().into_string().unwrap();

        let info = retrieve(distros).unwrap();
        assert_eq!(Type::Redhat, version.os_type());
        assert_eq!(result.version, Version::custom("XX".to_string(), None));
    }

    #[test]
    fn test_file_alpine() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/alpine-release");

        let mut distributions = [DISTRIBUTIONS[3]];
        distributions.path = file.into_os_string().into_string().unwrap();

        let info = retrieve(distros).unwrap();
        assert_eq!(Type::Alpine, version.os_type());
        assert_eq!(result.version, Version::custom("A.B.C".to_string(), None));
    }
}
