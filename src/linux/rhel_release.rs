use regex::Regex;

use std::fs::File;
use std::io::Error;
use std::io::prelude::*;

use {Info, Type, Version};
use super::utils;

pub fn rhel_release() -> Info {
    match retrieve() {
        Some(release) => {
            if release.distro == Some("CentOS".to_string()) {
                Info {
                    os_type: Type::Centos,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else {
                Info {
                    os_type: Type::Redhat,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            }
        }
        None => Info {
            os_type: Type::Linux,
            version: Version::unknown(),
        },
    }
}

struct RHELRelease {
    pub distro: Option<String>,
    pub version: Option<String>,
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn retrieve() -> Option<RHELRelease> {
    if utils::file_exists("/etc/redhat-release") {
        if let Ok(release) = read_file("/etc/redhat-release") {
            Some(parse(&release))
        } else {
            None
        }
    } else if let Ok(release) = read_file("/etc/centos-release") {
        Some(parse(&release))
    } else {
        None
    }
}

fn parse(file: &str) -> RHELRelease {
    let distrib_regex = Regex::new(r"(\w+) Linux release").unwrap();
    let version_regex = Regex::new(r"release\s([\w\.]+)").unwrap();

    let distro = match distrib_regex.captures_iter(file).next() {
        Some(m) => match m.get(1) {
            Some(distro) => Some(distro.as_str().to_owned()),
            None => None,
        },
        None => None,
    };

    let version = match version_regex.captures_iter(file).next() {
        Some(m) => match m.get(1) {
            Some(version) => Some(version.as_str().to_owned()),
            None => None,
        },
        None => None,
    };

    RHELRelease {
        distro: distro,
        version: version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parses_distribution() {
        let parse_results = parse(file());
        assert_eq!(parse_results.distro, Some("CentOS".to_string()));
    }

    #[test]
    pub fn test_parses_version() {
        let parse_results = parse(file());
        assert_eq!(parse_results.version, Some("7.3.1611".to_string()));
    }

    fn file() -> &'static str {
        "CentOS Linux release 7.3.1611 (Core)".into()
    }
}
