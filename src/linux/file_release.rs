use std::{fs::File, io::Read, path::Path};

use log::{trace, warn};
use regex::Regex;

use crate::{Info, Type, Version};

pub fn get() -> Option<Info> {
    retrieve(&DISTRIBUTIONS)
}

fn retrieve(distributions: &[ReleaseInfo]) -> Option<Info> {
    for release_info in distributions {
        if !Path::new(release_info.path).exists() {
            trace!("Path '{}' doesn't exist", release_info.path);
            continue;
        }

        let mut file = match File::open(&release_info.path) {
            Ok(val) => val,
            Err(e) => {
                warn!("Unable to open {:?} file: {:?}", release_info.path, e);
                continue;
            }
        };

        let mut file_content = String::new();
        if let Err(e) = file.read_to_string(&mut file_content) {
            warn!("Unable to read {:?} file: {:?}", release_info.path, e);
            continue;
        }

        let version = if !release_info.version_regex.is_empty() {
            let version_regex = Regex::new(release_info.version_regex).unwrap();

            version_regex
                .captures_iter(&file_content)
                .next()
                .and_then(|c| c.get(1))
                .map(|v| v.as_str().trim_end().to_owned())
        } else {
            Some(file_content.trim_end().to_string())
        }
        .map(|x| Version::custom(x, None))
        .unwrap_or_else(Version::unknown);

        return Some(Info::new(release_info.os_type, version));
    }

    None
}

#[derive(Debug, Clone)]
struct ReleaseInfo<'a> {
    os_type: Type,
    path: &'a str,
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
    use pretty_assertions::assert_eq;

    #[test]
    fn centos() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/centos-release");

        let path = file.into_os_string().into_string().unwrap();
        let mut distributions = [DISTRIBUTIONS[0].clone()];
        distributions[0].path = &path;

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Centos);
        assert_eq!(info.version, Version::custom("XX", None));
    }

    #[test]
    fn fedora() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/fedora-release");

        let path = file.into_os_string().into_string().unwrap();
        let mut distributions = [DISTRIBUTIONS[1].clone()];
        distributions[0].path = &path;

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Fedora);
        assert_eq!(info.version, Version::custom("26", None));
    }

    #[test]
    fn redhat() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/redhat-release");

        let path = file.into_os_string().into_string().unwrap();
        let mut distributions = [DISTRIBUTIONS[2].clone()];
        distributions[0].path = &path;

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Redhat);
        assert_eq!(info.version, Version::custom("XX", None));
    }

    #[test]
    fn alpine() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/alpine-release");

        let path = file.into_os_string().into_string().unwrap();
        let mut distributions = [DISTRIBUTIONS[3].clone()];
        distributions[0].path = &path;

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Alpine);
        assert_eq!(info.version, Version::custom("A.B.C", None));
    }
}
