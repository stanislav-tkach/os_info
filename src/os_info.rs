use std::fmt::{self, Display, Formatter};

///A list of supported operating system types
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

/// Holds information about Operating System type and its version
/// If the version could not be fetched it defaults to `0.0.0`
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OSInformation {
    pub os_type: OSType,
    pub version: String,
}

pub fn unknown_version() -> String {
    "Unknown".into()
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
