extern crate regex;

use self::regex::Regex;
use std::process::Command;

pub struct WindowsVer {
    pub version: Option<String>
}

pub fn parse(output: String) -> WindowsVer {
    let version_regex = Regex::new(r"^Microsoft Windows \[Version\s(\d+\.\d+\.\d+)\]$").unwrap();

    let version = match version_regex.captures_iter(&output).next() {
        Some(m) => {
            match m.at(1) {
                Some(version) => Some(version.to_string()),
                None => None
            }
        },
        None => None
    };
    WindowsVer { version: version }
}
