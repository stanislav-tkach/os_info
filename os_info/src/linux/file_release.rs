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

        return Some(Info {
            os_type: os_type.unwrap(),
            version: version.unwrap_or(Version::Unknown),
            bitness: Bitness::Unknown,
            ..Default::default()
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
}

impl fmt::Debug for ReleaseInfo<'_> {
    fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReleaseInfo")
            .field("path", &self.path)
            .field("os_type", &(self.os_type as fn(&'a str) -> Option<Type>))
            .field("version", &(self.version as fn(&'a str) -> Option<Version>))
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
    },
    ReleaseInfo {
        path: "etc/centos-release",
        os_type: |_| Some(Type::CentOS),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
    },
    ReleaseInfo {
        path: "etc/fedora-release",
        os_type: |_| Some(Type::Fedora),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
    },
    ReleaseInfo {
        path: "etc/alpine-release",
        os_type: |_| Some(Type::Alpine),
        version: |release| Matcher::AllTrimmed.find(release).map(Version::from_string),
    },
    ReleaseInfo {
        path: "etc/redhat-release",
        os_type: |_| Some(Type::RedHatEnterprise),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
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
                "src/linux/tests/Alpine_3_17",
                Some(Info {
                    os_type: Type::Alpine,
                    version: Version::Semantic(3, 17, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Alpine",
                Some(Info {
                    os_type: Type::Alpine,
                    version: Version::Semantic(3, 17, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Amazon_1",
                Some(Info {
                    os_type: Type::Amazon,
                    version: Version::Semantic(2018, 3, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Amazon_2",
                Some(Info {
                    os_type: Type::Amazon,
                    version: Version::Semantic(2, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Arch",
                Some(Info {
                    os_type: Type::Arch,
                    // TODO: Should be Version::Rolling
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/CentOS_7",
                Some(Info {
                    os_type: Type::CentOS,
                    version: Version::Semantic(7, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/CentOS_Stream",
                Some(Info {
                    os_type: Type::CentOS,
                    version: Version::Semantic(8, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/CentOS",
                Some(Info {
                    os_type: Type::CentOS,
                    version: Version::Custom("XX".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/CentOS_Unknown",
                Some(Info {
                    os_type: Type::CentOS,
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/EndeavourOS",
                Some(Info {
                    os_type: Type::EndeavourOS,
                    // TODO: Should be Version::Rolling
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Fedora_32",
                Some(Info {
                    os_type: Type::Fedora,
                    version: Version::Semantic(32, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Fedora_35",
                Some(Info {
                    os_type: Type::Fedora,
                    version: Version::Semantic(35, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Fedora",
                Some(Info {
                    os_type: Type::Fedora,
                    version: Version::Semantic(26, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Fedora_Unknown",
                Some(Info {
                    os_type: Type::Fedora,
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Garuda",
                Some(Info {
                    os_type: Type::Garuda,
                    // TODO: Should be Version::Rolling
                    version: Version::Unknown,
                    ..Default::default() // NOTE: lsb-release has DISTRIB_CODENAME="Talon" and DISTRIB_RELEASE=Soaring
                }),
            ),
            (
                "src/linux/tests/Mariner",
                Some(Info {
                    os_type: Type::Mariner,
                    version: Version::Semantic(2, 0, 20220210),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Mariner_Unknown",
                Some(Info {
                    os_type: Type::Mariner,
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Mint",
                Some(Info {
                    os_type: Type::Mint,
                    version: Version::Semantic(20, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/NixOS",
                Some(Info {
                    os_type: Type::NixOS,
                    version: Version::Custom("21.05pre275822.916ee862e87".to_owned()),
                    ..Default::default()
                }),
            ),
            ("src/linux/tests/none_invalid_os_release", None),
            ("src/linux/tests/none_no_release", None),
            ("src/linux/tests/none_no_path", None),
            (
                "src/linux/tests/OpenCloudOS",
                Some(Info {
                    os_type: Type::OpenCloudOS,
                    version: Version::Semantic(8, 6, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/openEuler",
                Some(Info {
                    os_type: Type::openEuler,
                    version: Version::Semantic(22, 3, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/OracleLinux",
                Some(Info {
                    os_type: Type::OracleLinux,
                    version: Version::Semantic(8, 1, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Pop_22",
                Some(Info {
                    os_type: Type::Pop,
                    version: Version::Semantic(22, 4, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Raspbian_10",
                Some(Info {
                    os_type: Type::Raspbian,
                    version: Version::Semantic(10, 0, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/RedHatEnterprise_8",
                Some(Info {
                    os_type: Type::RedHatEnterprise,
                    version: Version::Semantic(8, 2, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/RedHatEnterprise_7",
                Some(Info {
                    os_type: Type::RedHatEnterprise,
                    version: Version::Semantic(7, 9, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/RedHatEnterprise",
                Some(Info {
                    os_type: Type::RedHatEnterprise,
                    version: Version::Custom("XX".to_owned()),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/RedHatEnterprise_Unknown",
                Some(Info {
                    os_type: Type::RedHatEnterprise,
                    version: Version::Unknown,
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Solus",
                Some(Info {
                    os_type: Type::Solus,
                    version: Version::Semantic(4, 3, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/SUSE_12",
                Some(Info {
                    os_type: Type::SUSE,
                    version: Version::Semantic(12, 5, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/SUSE_15",
                Some(Info {
                    os_type: Type::SUSE,
                    version: Version::Semantic(15, 2, 0),
                    ..Default::default()
                }),
            ),
            (
                "src/linux/tests/Ubuntu",
                Some(Info {
                    os_type: Type::Ubuntu,
                    version: Version::Semantic(18, 10, 0),
                    ..Default::default()
                }),
            ),
        ];

        for (root, expected) in expected_pairs {
            assert_eq!(super::retrieve(&DISTRIBUTIONS, root), expected)
        }
    }

    #[test]
    fn release_info_debug() {
        dbg!("{:?}", &DISTRIBUTIONS[0]);
    }
}
