///A list of supported operating system types
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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
