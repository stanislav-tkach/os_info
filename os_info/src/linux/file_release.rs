// spell-checker:ignore sles

use std::{fmt, fs::File, io::Read, path::Path};

use log::{trace, warn};

use crate::{matcher::Matcher, Bitness, Info, Type, Version};

pub fn get() -> Option<Info> {
    retrieve(&DISTRIBUTIONS)
}

fn retrieve(distributions: &[ReleaseInfo]) -> Option<Info> {
    for release_info in distributions {
        let path = Path::new(release_info.path);

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

/// Struct containing information on how to parse distribution info from a
/// release file.
///
/// # Example
/// ```rust,ignore
/// ReleaseInfo {
///     path: "/etc/fedora-release",
///     os_type: |_| Some(Type::Fedora),
///     version: |release| {
///         Matcher::PrefixedVersion { prefix: "release" }
///             .find(&release)
///             .map(Version::from_string)
///     },
/// },
/// ```
#[derive(Clone)]
struct ReleaseInfo<'a> {
    /// The release file the struct corresponds to.
    ///
    /// # Example
    /// ```rust,ignore
    /// path: "/etc/os-release"
    /// ```
    path: &'a str,

    /// A closure that determines the os type from the release file contents.
    ///
    /// # Example
    /// ```rust,ignore
    /// //path: "/etc/mariner-release",
    /// os_type: |_| Some(Type::Mariner),
    /// ```
    os_type: for<'b> fn(&'b str) -> Option<Type>,

    /// A closure that determines the os version from the release file contents.
    ///
    /// # Example
    /// ```rust,ignore
    /// version: |release| {
    ///     Matcher::KeyValue { key: "VERSION_ID" }
    ///         .find(&release)
    ///         .map(Version::from_string)
    /// },
    /// ```
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
    ReleaseInfo {
        path: "/etc/mariner-release",
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
        path: "/etc/centos-release",
        os_type: |_| Some(Type::CentOS),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
    },
    ReleaseInfo {
        path: "/etc/fedora-release",
        os_type: |_| Some(Type::Fedora),
        version: |release| {
            Matcher::PrefixedVersion { prefix: "release" }
                .find(release)
                .map(Version::from_string)
        },
    },
    ReleaseInfo {
        path: "/etc/alpine-release",
        os_type: |_| Some(Type::Alpine),
        version: |release| Matcher::AllTrimmed.find(release).map(Version::from_string),
    },
    // TODO: This should be placed first, as most modern distributions
    // will have this file.
    ReleaseInfo {
        path: "/etc/os-release",
        os_type: |release| {
            Matcher::KeyValue { key: "ID" }
                .find(release)
                .and_then(|id| match id.as_str() {
                    // os-release information collected from
                    // https://github.com/chef/os_release

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
                    //"debian" => Debian
                    //"devuan" => Devuan
                    //"elementary" => Elementary
                    "fedora" => Some(Type::Fedora),
                    //"gentoo" => Gentoo
                    //"ios_xr" => ios_xr
                    //"kali" => Kali
                    //"mageia" => Mageia
                    //"manjaro" => Manjaro
                    "linuxmint" => Some(Type::Mint),
                    "mariner" => Some(Type::Mariner),
                    //"nexus" => Nexus
                    "nixos" => Some(Type::NixOS),
                    "ol" => Some(Type::OracleLinux),
                    "opensuse" => Some(Type::openSUSE),
                    "opensuse-leap" => Some(Type::openSUSE),
                    //"rancheros" => RancherOS
                    //"raspbian" => Raspbian
                    // note XBian also uses "raspbian"
                    "rhel" => Some(Type::RedHatEnterprise),
                    //"rocky" => Rocky
                    //"sabayon" => Sabayon
                    //"scientific" => Scientific
                    //"slackware" => Slackware
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
    ReleaseInfo {
        path: "/etc/redhat-release",
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
    fn oracle_linux() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::OracleLinux);
        assert_eq!(info.version, Version::Semantic(8, 1, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_alpine_3_12() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-alpine-3-12";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Alpine);
        assert_eq!(info.version, Version::Semantic(3, 12, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_amazon_1() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-amazon-1";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Amazon);
        assert_eq!(info.version, Version::Semantic(2018, 3, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_amazon_2() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-amazon-2";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Amazon);
        assert_eq!(info.version, Version::Semantic(2, 0, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_centos() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-centos";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::CentOS);
        assert_eq!(info.version, Version::Semantic(7, 0, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_centos_stream() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-centos-stream";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::CentOS);
        assert_eq!(info.version, Version::Semantic(8, 0, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_fedora() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-fedora-32";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Fedora);
        assert_eq!(info.version, Version::Semantic(32, 0, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_fedora_35() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-fedora-35";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Fedora);
        assert_eq!(info.version, Version::Semantic(35, 0, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_nixos() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-nixos";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::NixOS);
        assert_eq!(
            info.version,
            Version::Custom("21.05pre275822.916ee862e87".to_string())
        );
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_rhel() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-rhel";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::RedHatEnterprise);
        assert_eq!(info.version, Version::Semantic(8, 2, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_rhel_7() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-rhel-7";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::RedHatEnterprise);
        assert_eq!(info.version, Version::Semantic(7, 9, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_suse_12() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-suse-12";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::SUSE);
        assert_eq!(info.version, Version::Semantic(12, 5, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_suse_15() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-suse-15";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::SUSE);
        assert_eq!(info.version, Version::Semantic(15, 2, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_ubuntu() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-ubuntu";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Ubuntu);
        assert_eq!(info.version, Version::Semantic(18, 10, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn os_release_mint() {
        let mut distributions = [DISTRIBUTIONS[4].clone()];
        distributions[0].path = "src/linux/tests/os-release-mint";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Mint);
        assert_eq!(info.version, Version::Semantic(20, 0, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn centos() {
        let mut distributions = [DISTRIBUTIONS[1].clone()];
        distributions[0].path = "src/linux/tests/centos-release";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::CentOS);
        assert_eq!(info.version, Version::Custom("XX".to_owned()));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn fedora() {
        let mut distributions = [DISTRIBUTIONS[2].clone()];
        distributions[0].path = "src/linux/tests/fedora-release";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Fedora);
        assert_eq!(info.version, Version::Semantic(26, 0, 0));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn redhat() {
        let mut distributions = [DISTRIBUTIONS[5].clone()];
        distributions[0].path = "src/linux/tests/redhat-release";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::RedHatEnterprise);
        assert_eq!(info.version, Version::Custom("XX".to_owned()));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn alpine() {
        let mut distributions = [DISTRIBUTIONS[3].clone()];
        distributions[0].path = "src/linux/tests/alpine-release";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Alpine);
        assert_eq!(info.version, Version::Custom("A.B.C".to_owned()));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn mariner() {
        let mut distributions = [DISTRIBUTIONS[0].clone()];
        distributions[0].path = "src/linux/tests/mariner-release";

        let info = retrieve(&distributions).unwrap();
        assert_eq!(info.os_type(), Type::Mariner);
        assert_eq!(info.version, Version::Semantic(2, 0, 20220210));
        assert_eq!(info.edition, None);
        assert_eq!(info.codename, None);
    }

    #[test]
    fn release_info_debug() {
        dbg!("{:?}", &DISTRIBUTIONS[0]);
    }
}
