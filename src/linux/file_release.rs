// spell-checker:ignore distro, distros

use regex::Regex;

use std::{fs::{self, File}, io::{Error, ErrorKind}};

use Type;

/// `ReleaseFile` Structure
/// Holds information about a distro specific release file.
/// Information can include the type of distro, a human readable
/// name for the distro, the distro version, the path to the
/// release file (i.e: /etc/centos-release), a distro regex
/// which will parse the name of the distro from the release
/// file and a version regex which will parse the version
/// from the release file.
#[derive(Debug)]
pub struct ReleaseFile {
    pub(crate) os_type: Type,
    pub distro: Option<String>,
    pub version: Option<String>,
    name: String,
    path: String,
    regex_distro: String,
    regex_version: String,
}

/// `ReleaseFile` Structure Default Values
/// Sets all default values for a `ReleaseFile`
/// structure.
impl Default for ReleaseFile {
    fn default() -> Self {
        Self {
            os_type: Type::Unknown,
            distro: None,
            version: None,
            name: "".to_string(),
            path: "".to_string(),
            regex_distro: "".to_string(),
            regex_version: "".to_string(),
        }
    }
}

/// `ReleaseFile` Implementation
/// Helper functions for a `ReleaseFile` structure
impl ReleaseFile {
    /// ReleaseFile.exists()
    /// Does a release file exist?
    fn exists(&self) -> bool {
        match fs::metadata(&self.path) {
            Ok(md) => md.is_dir() || md.is_file(),
            Err(_) => false,
        }
    }

    /// ReleaseFile.read()
    /// Get data inside of a release file.
    fn read(&self) -> Result<String, Error> {
        if self.exists() {
            let mut file = File::open(&self.path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            Err(Error::new(ErrorKind::NotFound, "File does not exist!"))
        }
    }

    /// ReleaseFile.parse()
    /// Parse the distribution name and version information
    /// from a release file.
    fn parse(self) -> Result<Self, Error> {
        match self.read() {
            Ok(data) => {
                let distro = if !self.regex_distro.is_empty() {
                    let distro_regex = Regex::new(&self.regex_distro).unwrap();
                    match distro_regex.captures_iter(&data).next() {
                        Some(m) => match m.get(1) {
                            Some(distro) => Some(distro.as_str().to_owned()),
                            None => None,
                        },
                        None => None,
                    }
                } else {
                    Some(self.name.clone())
                };
                let version = if !self.regex_version.is_empty() {
                    let version_regex = Regex::new(&self.regex_version).unwrap();
                    match version_regex.captures_iter(&data).next() {
                        Some(m) => match m.get(1) {
                            Some(version) => Some(version.as_str().trim_right().to_owned()),
                            None => None,
                        },
                        None => None,
                    }
                } else {
                    Some(data.trim_right().to_string())
                };
                Ok(ReleaseFile {
                    distro,
                    version,
                    ..self
                })
            }
            Err(e) => Err(e),
        }
    }
}

/// distributions()
/// Returns a vector of instantiated `ReleaseFile`
/// structures. This vector contains all supported
/// distributions and how to parse their version
/// information from their release file.
pub fn distributions() -> Vec<ReleaseFile> {
    vec![
        ReleaseFile {
            os_type: Type::Centos,
            name: "CentOS".to_string(),
            path: "/etc/centos-release".to_string(),
            regex_distro: r"(\w+) Linux release".to_string(),
            regex_version: r"release\s([\w\.]+)".to_string(),
            ..Default::default()
        },
        ReleaseFile {
            os_type: Type::Fedora,
            name: "Fedora".to_string(),
            path: "/etc/fedora-release".to_string(),
            regex_distro: r"(\w+) release".to_string(),
            regex_version: r"release\s([\w\.]+)".to_string(),
            ..Default::default()
        },
        ReleaseFile {
            os_type: Type::Redhat,
            name: "Redhat".to_string(),
            path: "/etc/redhat-release".to_string(),
            regex_distro: r"(\w+) Linux release".to_string(),
            regex_version: r"release\s([\w\.]+)".to_string(),
            ..Default::default()
        },
        ReleaseFile {
            os_type: Type::Alpine,
            name: "Alpine".to_string(),
            path: "/etc/alpine-release".to_string(),
            ..Default::default()
        },
    ]
}

/// retrieve()
/// Parses the a vector of `ReleaseFile` structures.
/// If the release file in `ReleaseFile`.path exists,
/// the information will be parsed and returned.
pub fn retrieve(distros: Vec<ReleaseFile>) -> Option<ReleaseFile> {
    let it = distros.into_iter();
    for distro in it {
        match distro.parse() {
            Ok(release) => return Some(release),
            Err(_) => continue,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    pub fn test_file_centos() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/centos-release");

        let distros: Vec<ReleaseFile> = vec![
            ReleaseFile {
                os_type: Type::Centos,
                name: "CentOS".to_string(),
                path: file.into_os_string().into_string().unwrap(),
                regex_distro: r"(\w+) Linux release".to_string(),
                regex_version: r"release\s([\w\.]+)".to_string(),
                ..Default::default()
            },
        ];

        let result = retrieve(distros).unwrap();
        assert_eq!(result.os_type, Type::Centos);
        assert_eq!(result.distro, Some("Centos".to_string()));
        assert_eq!(result.version, Some("XX".to_string()));
        assert_eq!(result.name, "CentOS".to_string());
    }

    #[test]
    pub fn test_file_fedora() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/fedora-release");

        let distros: Vec<ReleaseFile> = vec![
            ReleaseFile {
                os_type: Type::Fedora,
                name: "Fedora".to_string(),
                path: file.into_os_string().into_string().unwrap(),
                regex_distro: r"(\w+) release".to_string(),
                regex_version: r"release\s([\w\.]+)".to_string(),
                ..Default::default()
            },
        ];

        let result = retrieve(distros).unwrap();
        assert_eq!(result.os_type, Type::Fedora);
        assert_eq!(result.distro, Some("Fedora".to_string()));
        assert_eq!(result.version, Some("26".to_string()));
        assert_eq!(result.name, "Fedora".to_string());
    }

    #[test]
    pub fn test_file_redhat() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/redhat-release");

        let distros: Vec<ReleaseFile> = vec![
            ReleaseFile {
                os_type: Type::Redhat,
                name: "Redhat".to_string(),
                path: file.into_os_string().into_string().unwrap(),
                regex_distro: r"(\w+) Linux release".to_string(),
                regex_version: r"release\s([\w\.]+)".to_string(),
                ..Default::default()
            },
        ];

        let result = retrieve(distros).unwrap();
        assert_eq!(result.os_type, Type::Redhat);
        assert_eq!(result.distro, Some("Redhat".to_string()));
        assert_eq!(result.version, Some("XX".to_string()));
        assert_eq!(result.name, "Redhat".to_string());
    }

    #[test]
    pub fn test_file_alpine() {
        let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file.push("src/linux/tests/alpine-release");

        let distros: Vec<ReleaseFile> = vec![
            ReleaseFile {
                os_type: Type::Alpine,
                name: "Alpine".to_string(),
                path: file.into_os_string().into_string().unwrap(),
                ..Default::default()
            },
        ];

        let result = retrieve(distros).unwrap();
        assert_eq!(result.os_type, Type::Alpine);
        assert_eq!(result.distro, Some("Alpine".to_string()));
        assert_eq!(result.version, Some("A.B.C".to_string()));
        assert_eq!(result.name, "Alpine".to_string());
    }
}
