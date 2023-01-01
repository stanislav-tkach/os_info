// spell-checker:ignore sles

use std::{fmt, fs::File, io::Read, path::Path};

use log::{trace, warn};

use crate::{matcher::Matcher, Bitness, Info, Type, Version};

pub fn get() -> Option<Info> {
    retrieve(&DISTRIBUTIONS, "/")
}

fn retrieve(distributions: &[ReleaseInfo], root: &str) -> Option<Info> {
    for release_info in distributions {
        let path = Path::new(root).join(release_info.path);

        if !path.exists() {
            trace!("Path '{}' doesn't exist", release_info.path);
            continue;
        }

        let mut file = match File::open(&path) {
            Ok(val) => val,
            Err(e) => {
                warn!("Unable to open {:?} file: {:?}", &path, e);
                continue;
            }
        };

        let mut file_content = String::new();
        if let Err(e) = file.read_to_string(&mut file_content) {
            warn!("Unable to read {:?} file: {:?}", &path, e);
            continue;
        }

        let os_type = (release_info.os_type)(&file_content);

        // If os_type is indeterminate, try the next release_info
        if os_type.is_none() {
            continue;
        }

        let version = (release_info.version)(&file_content);
        let edition = (release_info.edition)(&file_content);
        let codename = (release_info.codename)(&file_content);

        return Some(Info {
            os_type: os_type.unwrap(),
            version: version.unwrap_or(Version::Unknown),
            edition,
            codename,
            bitness: Bitness::Unknown,
        });
    }

    // Failed to determine os info
    None
}

/// Struct containing information on how to parse distribution info from a release file.
#[derive(Clone)]
struct ReleaseInfo<'a> {
    /// Relative path to the release file this struct corresponds to from root.
    path: &'a str,

    /// A closure that determines the os type from the release file contents.
    os_type: for<'b> fn(&'b str) -> Option<Type>,

    /// A closure that determines the os version from the release file contents.
    version: for<'b> fn(&'b str) -> Option<Version>,

    /// A closure that determines the os edition (variant) from the release file contents.
    edition: for<'b> fn(&'b str) -> Option<String>,

    /// A closure that determines the os codename from the release file contents.
    codename: for<'b> fn(&'b str) -> Option<String>,
}

impl fmt::Debug for ReleaseInfo<'_> {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReleaseInfo")
            .field("path", &self.path)
            .field("os_type", &(self.os_type as fn(&'a str) -> Option<Type>))
            .field("version", &(self.version as fn(&'a str) -> Option<Version>))
            .field("edition", &(self.edition as fn(&'a str) -> Option<String>))
            .finish()
    }
}

/// List of all supported distributions and the information on how to parse their version from the
/// release file.
static DISTRIBUTIONS: [ReleaseInfo; 6] = [
    // Keep this first; most modern distributions have this file.
    ReleaseInfo {
        path: "etc/os-release",
        os_type: |release| {
            Matcher::KeyValue { key: "ID" }
                .find(release)
                .and_then(|id| match id.as_str() {
                    // os-release information collected from
                    // https://github.com/chef/os_release
                    // and iso installations

                    //"almalinux" => Alma
                    "alpine" => Some(Type::Alpine),
                    "amzn" => Some(Type::Amazon),
                    //"antergos" => Antergos
                    //"aosc" => AOSC
                    "arch" => Some(Type::Arch),
                    //"artix" => Artix
                    "centos" => Some(Type::CentOS),
                    //"clear-linux-os" => ClearLinuxOS
                    //"clearos" => ClearOS
                    //"coreos"
                    //"cumulus-linux" => Cumulus
                    "debian" => Some(Type::Debian),
                    //"devuan" => Devuan
                    //"elementary" => Elementary
                    "endeavouros" => Some(Type::EndeavourOS),
                    "fedora" => Some(Type::Fedora),
                    "garuda" => Some(Type::Garuda),
                    "gentoo" => Some(Type::Gentoo),
                    //"ios_xr" => ios_xr
                    //"kali" => Kali
                    //"mageia" => Mageia
                    "manjaro" => Some(Type::Manjaro),
                    "mariner" => Some(Type::Mariner),
                    "linuxmint" => Some(Type::Mint),
                    //"nexus" => Nexus
                    "nixos" => Some(Type::NixOS),
                    "opencloudos" => Some(Type::OpenCloudOS),
                    "openEuler" => Some(Type::openEuler),
                    "opensuse" => Some(Type::openSUSE),
                    "opensuse-leap" => Some(Type::openSUSE),
                    "ol" => Some(Type::OracleLinux),
                    "pop" => Some(Type::Pop),
                    //"rancheros" => RancherOS
                    "raspbian" => Some(Type::Raspbian),
                    // NOTE: XBian also uses "raspbian"
                    // => Some(Type::Redhat),
                    "rhel" => Some(Type::RedHatEnterprise),
                    //"rocky" => Rocky
                    //"sabayon" => Sabayon
                    //"scientific" => Scientific
                    //"slackware" => Slackware
                    "solus" => Some(Type::Solus),
                    "sled" => Some(Type::SUSE), // SUSE desktop
                    "sles" => Some(Type::SUSE),
                    "sles_sap" => Some(Type::SUSE), // SUSE SAP
                    "ubuntu" => Some(Type::Ubuntu),
                    //"virtuozzo" => Virtuozzo
                    //"void" => Void
                    //"XCP-ng" => xcp-ng
                    //"xenenterprise" => xcp-ng
                    //"xenserver" => xcp-ng
                    _ => None,
                })
        },
        version: |release| {
            Matcher::KeyValue { key: "VERSION_ID" }
                .find(release)
                .map(Version::from_string)
        },
        edition: |release| Matcher::KeyValue { key: "VARIANT" }.find(release),
        codename: |release| {
            Matcher::KeyValue {
                key: "VERSION_CODENAME",
            }
            .find(release)
            .filter(|v| !v.is_empty())
        },
    },
    // Older distributions must have their specific release file parsed.
    ReleaseInfo {
        path: "etc/mariner-release",
        os_type: |_| Some(Type::Mariner),
        version: |release| {
            Matcher::PrefixedVersion {
                prefix: "CBL-Mariner",
            }
            .find(release)
            .map(Version::from_string)
        },
        edition: |_| None,
        codename: |_| None,
    },
    ReleaseInfo {
        path: "etc/centos-release",
        os_type: |_| Some(Type::CentOS),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
        edition: |_| None,
        codename: |_| None,
    },
    ReleaseInfo {
        path: "etc/fedora-release",
        os_type: |_| Some(Type::Fedora),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
        edition: |_| None,
        codename: |_| None,
    },
    ReleaseInfo {
        path: "etc/alpine-release",
        os_type: |_| Some(Type::Alpine),
        version: |release| Matcher::AllTrimmed.find(release).map(Version::from_string),
        edition: |_| None,
        codename: |_| None,
    },
    ReleaseInfo {
        path: "etc/redhat-release",
        os_type: |_| Some(Type::RedHatEnterprise),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
        edition: |_| None,
        codename: |_| None,
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn retrieve() {
        let expected_pairs = [
            (
                "src/linux/tests/Alpine/3.17.0",
                Some(Info {
                    os_type: Type::Alpine,
                    version: Version::Semantic(3, 17, 0),
                    ..Default::default()
                }),
            ),
            (
                // Isolated alpine-release test
                "src/linux/tests/Alpine/3.17.0/alpine-release",
                Some(Info {
                    os_type: Type::Alpine,
                    version: Version::Semantic(3, 17, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Amazon/2018.3.0",
                Some(Info {
                    os_type: Type::Amazon,
                    version: Version::Semantic(2018, 3, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Amazon/2.0.0",
                Some(Info {
                    os_type: Type::Amazon,
                    version: Version::Semantic(2, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Arch/rolling",
                Some(Info {
                    os_type: Type::Arch,
                    // TODO: Should be Version::Rolling
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                // CentOS Linux
                "src/linux/tests/CentOS/7.0.0",
                Some(Info {
                    os_type: Type::CentOS,
                    version: Version::Semantic(7, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/CentOS/7.0.0/centos-release",
                Some(Info {
                    os_type: Type::CentOS,
                    version: Version::Semantic(7, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                // CentOS Stream
                "src/linux/tests/CentOS/8.0.0",
                Some(Info {
                    os_type: Type::CentOS,
                    version: Version::Semantic(8, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Debian/11.0.0",
                Some(Info {
                    os_type: Type::Debian,
                    version: Version::Semantic(11, 0, 0),
                    codename: Some("bullseye".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/EndeavourOS/rolling",
                Some(Info {
                    os_type: Type::EndeavourOS,
                    // TODO: Should be Version::Rolling
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Fedora/32.0.0/cloud",
                Some(Info {
                    os_type: Type::Fedora,
                    version: Version::Semantic(32, 0, 0),
                    edition: Some("Cloud Edition".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Fedora/35.0.0/workstation",
                Some(Info {
                    os_type: Type::Fedora,
                    version: Version::Semantic(35, 0, 0),
                    edition: Some("Workstation Edition".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Fedora/35.0.0/workstation/fedora-release",
                Some(Info {
                    os_type: Type::Fedora,
                    version: Version::Semantic(35, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Garuda/rolling",
                Some(Info {
                    os_type: Type::Garuda,
                    // TODO: Should be Version::Rolling
                    version: Version::Unknown,
                    ..Default::default() // NOTE: lsb-release has DISTRIB_CODENAME="Talon" and DISTRIB_RELEASE=Soaring
                }),
            ),
            (
                "src/linux/tests/Manjaro/rolling",
                Some(Info {
                    os_type: Type::Manjaro,
                    version: Version::Unknown,
                    // TODO: Should be Version::Rolling
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Mariner/2.0.20220210",
                Some(Info {
                    os_type: Type::Mariner,
                    version: Version::Semantic(2, 0, 20220210),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Mint/20.0.0",
                Some(Info {
                    os_type: Type::Mint,
                    version: Version::Semantic(20, 0, 0),
                    codename: Some("ulyana".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/NixOS/21.05pre275822.916ee862e87",
                Some(Info {
                    os_type: Type::NixOS,
                    version: Version::Custom("21.05pre275822.916ee862e87".to_owned()),
                    codename: Some("okapi".to_owned()),
                    ..Default::default()
                }),
            ),
            ("src/linux/tests/none_invalid_os_release", None),
            ("src/linux/tests/none_no_path", None),
            ("src/linux/tests/none_no_release", None),
            (
                "src/linux/tests/OpenCloudOS/8.6.0",
                Some(Info {
                    os_type: Type::OpenCloudOS,
                    version: Version::Semantic(8, 6, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/openEuler/22.3.0",
                Some(Info {
                    os_type: Type::openEuler,
                    version: Version::Semantic(22, 3, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/OracleLinux/8.1.0/server",
                Some(Info {
                    os_type: Type::OracleLinux,
                    version: Version::Semantic(8, 1, 0),
                    edition: Some("Server".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Pop/22.4.0",
                Some(Info {
                    os_type: Type::Pop,
                    version: Version::Semantic(22, 4, 0),
                    codename: Some("jammy".to_string()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Raspbian/10.0.0",
                Some(Info {
                    os_type: Type::Raspbian,
                    version: Version::Semantic(10, 0, 0),
                    codename: Some("buster".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/RedHatEnterprise/8.2.0",
                Some(Info {
                    os_type: Type::RedHatEnterprise,
                    version: Version::Semantic(8, 2, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/RedHatEnterprise/7.9.0/server",
                Some(Info {
                    os_type: Type::RedHatEnterprise,
                    version: Version::Semantic(7, 9, 0),
                    edition: Some("Server".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/RedHatEnterprise/7.9.0/server/redhat-release",
                Some(Info {
                    os_type: Type::RedHatEnterprise,
                    version: Version::Semantic(7, 9, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Solus/4.3.0",
                Some(Info {
                    os_type: Type::Solus,
                    version: Version::Semantic(4, 3, 0),
                    codename: Some("fortitude".to_string()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/SUSE/12.5.0",
                Some(Info {
                    os_type: Type::SUSE,
                    version: Version::Semantic(12, 5, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/SUSE/15.2.0",
                Some(Info {
                    os_type: Type::SUSE,
                    version: Version::Semantic(15, 2, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Ubuntu/18.10.0",
                Some(Info {
                    os_type: Type::Ubuntu,
                    version: Version::Semantic(18, 10, 0),
                    codename: Some("cosmic".to_string()),
                    ..Default::default()
                }),
            ),
        ];

        for (root, expected) in expected_pairs {
            assert_eq!(
                super::retrieve(&DISTRIBUTIONS, root),
                expected,
                "the parsed release files (left) at `{}` did not match expected values (right)",
                root
            )
        }
    }

    #[test]
    fn release_info_debug() {
        format!("{:?}", &DISTRIBUTIONS[0]);
    }
}
