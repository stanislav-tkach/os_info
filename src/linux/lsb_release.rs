use regex::Regex;

use std::process::Command;

use {Info, Type, Version};

// TODO: Better matching.
pub fn lsb_release() -> Info {
    match retrieve() {
        Some(release) => {
            if release.distro == Some("Ubuntu".to_string()) {
                Info {
                    os_type: Type::Ubuntu,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else if release.distro == Some("Debian".to_string()) {
                Info {
                    os_type: Type::Debian,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else if release.distro == Some("Arch".to_string()) {
                Info {
                    os_type: Type::Arch,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else if release.distro == Some("CentOS".to_string()) {
                Info {
                    os_type: Type::Centos,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else if release.distro == Some("Fedora".to_string()) {
                Info {
                    os_type: Type::Fedora,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else {
                Info {
                    os_type: Type::Linux,
                    version: Version::unknown(),
                }
            }
        }
        None => Info {
            os_type: Type::Linux,
            version: Version::unknown(),
        },
    }
}

struct LsbRelease {
    pub distro: Option<String>,
    pub version: Option<String>,
}

fn retrieve() -> Option<LsbRelease> {
    let output = match Command::new("lsb_release").arg("-a").output() {
        Ok(o) => o,
        Err(_) => return None,
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(parse(&stdout))
}

pub fn is_available() -> bool {
    match Command::new("lsb_release").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn parse(file: &str) -> LsbRelease {
    let distrib_regex = Regex::new(r"Distributor ID:\s(\w+)").unwrap();
    let distrib_release_regex = Regex::new(r"Release:\s+([\w]+[.]?[\w]+?)?").unwrap();

    let distro = match distrib_regex.captures_iter(file).next() {
        Some(m) => {
            match m.get(1) {
                Some(distro) => Some(distro.as_str().to_owned()),
                None => None,
            }
        }
        None => None,
    };

    let version = match distrib_release_regex.captures_iter(file).next() {
        Some(m) => {
            match m.get(1) {
                Some(version) => Some(version.as_str().to_owned()),
                None => None,
            }
        }
        None => None,
    };

    LsbRelease {
        distro: distro,
        version: version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parses_lsb_distro() {
        let parse_results = parse(file());
        assert_eq!(parse_results.distro, Some("Debian".to_string()));
    }

    #[test]
    pub fn test_parses_lsb_version() {
        let parse_results = parse(file());
        assert_eq!(parse_results.version, Some("7.8".to_string()));
    }

    #[test]
    pub fn test_parses_arch_lsb_distro() {
        let parse_results = parse(arch_file());
        assert_eq!(parse_results.distro, Some("Arch".to_string()));
    }

    #[test]
    pub fn test_parses_arch_lsb_version() {
        let parse_results = parse(arch_file());
        assert_eq!(parse_results.version, Some("rolling".to_string()));
    }

    #[test]
    pub fn test_parses_fedora_lsb_distro() {
        let parse_results = parse(fedora_file());
        assert_eq!(parse_results.distro, Some("Fedora".to_string()));
    }

    #[test]
    pub fn test_parses_fedora_lsb_version() {
        let parse_results = parse(fedora_file());
        assert_eq!(parse_results.version, Some("26".to_string()));
    }

    fn file() -> &'static str {
        "
Distributor ID:	Debian
Description:	Debian GNU/Linux 7.8 (wheezy)
Release:	7.8
Codename:	wheezy
"
    }

    fn arch_file() -> &'static str {
        "
LSB Version:	1.4
Distributor ID:	Arch
Description:	Arch Linux
Release:	rolling
Codename:	n/a
"
    }

    fn fedora_file() -> &'static str {
        "
LSB Version:    :core-4.1-amd64:core-4.1-noarch:cxx-4.1-amd64:cxx-4.1-noarch
Distributor ID: Fedora
Description:    Fedora release 26 (Twenty Six)
Release:    26
Codename:   TwentySix
"
    }
}
