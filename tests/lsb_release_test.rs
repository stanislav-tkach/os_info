#![allow(dead_code)]

extern crate regex;

#[path = "../src/lsb_release.rs"]
mod lsb_release;
#[path = "../src/utils.rs"]
mod utils;

fn file() -> &'static str {
    "
Distributor ID:	Debian
Description:	Debian GNU/Linux 7.8 (wheezy)
Release:	7.8
Codename:	wheezy
"
}

fn arch_file() -> &'static str {
    "
LSB Version:	1.4
Distributor ID:	Arch
Description:	Arch Linux
Release:	rolling
Codename:	n/a
"
}

#[test]
pub fn test_parses_lsb_distro() {
    let parse_results = lsb_release::parse(file());
    assert_eq!(parse_results.distro, Some("Debian".to_string()));
}

#[test]
pub fn test_parses_lsb_version() {
    let parse_results = lsb_release::parse(file());
    assert_eq!(parse_results.version, Some("7.8".to_string()));
}

#[test]
pub fn test_parses_arch_lsb_distro() {
    let parse_results = lsb_release::parse(arch_file());
    assert_eq!(parse_results.distro, Some("Arch".to_string()));
}

#[test]
pub fn test_parses_arch_lsb_version() {
    let parse_results = lsb_release::parse(arch_file());
    assert_eq!(parse_results.version, Some("rolling".to_string()));
}
