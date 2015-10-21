extern crate regex;
use self::regex::Regex;
use std::io::prelude::*;
use std::fs::File;
use std::convert::AsRef;
use std::path::Path;
use std::io::Error;

pub struct LsbRelease {
    pub distro: Option<String>,
    pub version: Option<String>
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<LsbRelease, Error> {
    let mut handle = match File::open(path) {
        Ok(h) => h,
        Err(err) => return Err(err)
    };

    let mut file_content = String::new();

    match handle.read_to_string(&mut file_content) {
        Ok(_) => {
            let release = parse(file_content);
            Ok(release)
        },
        Err(err) => {
            Err(err)
        }
    }
}

pub fn parse(file: String) -> LsbRelease {
    let distrib_regex = Regex::new(r"DISTRIB_ID=(\w+)").unwrap();
    let distrib_release_regex = Regex::new(r"DISTRIB_RELEASE=([\w\.]+)").unwrap();

    let distro = match distrib_regex.captures_iter(&file).next() {
        Some(m) => {
            match m.at(1) {
                Some(distro) => {
                    Some(distro.to_string())
                },
                None => None
            }
        },
        None => None
    };

    let version = match distrib_release_regex.captures_iter(&file).next() {
        Some(m) => {
            match m.at(1) {
                Some(version) => Some(version.to_string()),
                None => None
            }
        },
        None => None
    };

    LsbRelease {
        distro: distro,
        version: version
    }
}
