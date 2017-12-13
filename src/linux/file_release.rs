use regex::Regex;

use std::fs::{ File, metadata };
use std::io::{ Error, ErrorKind };
use std::io::prelude::*;

use {Info, Type, Version};

pub fn file_release() -> Info {
    match retrieve() {
        Some(release) => {
            if release.name == "CentOS".to_string() {
                Info {
                    os_type: Type::Centos,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else if release.name == "Fedora".to_string() {
                Info {
                    os_type: Type::Fedora,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else if release.name == "Redhat".to_string() {
                Info {
                    os_type: Type::Redhat,
                    version: release
                        .version
                        .map(|x| Version::custom(x, None))
                        .unwrap_or_else(Version::unknown),
                }
            } else if release.name == "Alpine".to_string() {
                Info {
                    os_type: Type::Alpine,
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

#[derive(Debug)]
pub struct ReleaseFile {
    pub distro: Option<String>,
    pub version: Option<String>,
    name: String,
    path: String,
    regex_distro: String,
    regex_version: String,
}

impl Default for ReleaseFile {
    fn default() -> Self {
        Self {
            distro: None,
            version: None,
            name: "".to_string(),
            path: "".to_string(),
            regex_distro: "".to_string(),
            regex_version: "".to_string(),
        }
    }
}

impl ReleaseFile {
    fn exists(&self) -> bool {
        let metadata = metadata(&self.path);

        match metadata {
            Ok(md) => md.is_dir() || md.is_file(),
            Err(_) => false,
        }
    }
    fn read(&self) -> Result<String, Error> {
        if Self::exists(&self) {
            let mut file = File::open(&self.path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        } else {
            Err(
                Error::new(
                    ErrorKind::NotFound, "File does not exist!"
                )
            )
        }
    }
    fn parse(self) -> Result<Self, Error> {
        match self.read() {
            Ok(data) => {
                let distro = if self.regex_distro.len() > 0 {
                    let distrib_regex = Regex::new(&self.regex_distro).unwrap();
                    match distrib_regex.captures_iter(&data).next() {
                        Some(m) => {
                            match m.get(1) {
                                Some(distro) => Some(distro.as_str().to_owned()),
                                None => None,
                            }
                        }
                        None => None,
                    }
                } else {
                    Some(self.name.clone())
                };
                let version = if self.regex_version.len() > 0 {
                    let version_regex = Regex::new(&self.regex_version).unwrap();
                    match version_regex.captures_iter(&data).next() {
                        Some(m) => {
                            match m.get(1) {
                                Some(version) => Some(version.as_str().to_owned()),
                                None => None,
                            }
                        }
                        None => None,
                    }
                } else {
                    Some(data)
                };
                return Ok(ReleaseFile {
                    distro: distro,
                    version: version,
                    ..self
                });

            }
            Err(e) => return Err(e)
        }
    }
}

fn retrieve() -> Option<ReleaseFile> {
    let distros: Vec<ReleaseFile> = vec![
        ReleaseFile{
            name: "CentOS".to_string(),
            path: "/etc/centos-release".to_string(),
            regex_distro: r"(\w+) Linux release".to_string(),
            regex_version: r"release\s([\w\.]+)".to_string(),
            ..Default::default()
        },
        ReleaseFile{
            name: "Fedora".to_string(),
            path: "/etc/fedora-release".to_string(),
            regex_distro: r"(\w+) release".to_string(),
            regex_version: r"release\s([\w\.]+)".to_string(),
            ..Default::default()
        },
        ReleaseFile{
            name: "Redhat".to_string(),
            path: "/etc/redhat-release".to_string(),
            regex_distro: r"(\w+) Linux release".to_string(),
            regex_version: r"release\s([\w\.]+)".to_string(),
            ..Default::default()
        },
        ReleaseFile{
            name: "Alpine".to_string(),
            path: "/etc/alpine-release".to_string(),
            ..Default::default()
        },
    ];

    let mut it = distros.into_iter();
    loop {
        match it.next() {
            Some(distro) => match distro.parse() {
                Ok(release) => return Some(release),
                Err(_) => continue
            }
            None => break,
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parses_file() {
        let x = retrieve();
        println!("{:?}", x);
        assert_ne!(Some(2), Some(1));
    }
}

