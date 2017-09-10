use std::fmt::{self, Display, Formatter};

/// Holds information about Operating System type and its version
/// If the version could not be fetched it defaults to `0.0.0`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OSInfo {
    pub os_type: OSType,
    pub version: OSVersion,
}

impl OSInfo {
    pub fn unknown() -> Self {
        Self {
            os_type: OSType::Unknown,
            version: OSVersion::unknown(),
        }
    }
}

///A list of supported operating system types
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OSType {
    Unknown,
    Android,
    Emscripten,
    Linux,
    Redhat,
    Ubuntu,
    Debian,
    Arch,
    Centos,
    Macos,
    Redox,
    Windows,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OSVersion {
    pub version: VersionType,
    pub edition: String,
}

impl OSVersion {
    pub fn custom(version: String, edition: String) -> Self {
        Self {
            version: VersionType::Custom(version),
            edition: edition,
        }
    }

    pub fn unknown() -> Self {
        Self {
            version: VersionType::Unknown,
            edition: "".to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VersionType {
    Unknown,
    Semantic(u64, u64, u64),
    Custom(String),
}

impl Display for OSType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            OSType::Redhat => write!(f, "Red Hat Linux"),
            OSType::Arch => write!(f, "Arch Linux"),
            OSType::Centos => write!(f, "CentOS"),
            OSType::Macos => write!(f, "Mac OS"),
            _ => write!(f, "{:?}", self),
        }
    }
}
