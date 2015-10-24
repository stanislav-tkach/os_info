#[path="../src/lsb_release.rs"]
mod lsb_release;

fn file() -> String {
"
Distributor ID:	Debian
Description:	Debian GNU/Linux 7.8 (wheezy)
Release:	7.8
Codename:	wheezy
".to_string()
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
