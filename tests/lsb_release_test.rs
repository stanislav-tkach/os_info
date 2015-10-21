#[path="../src/lsb_release.rs"]
mod lsb_release;

fn file() -> String {
"
    DISTRIB_ID=Ubuntu
    DISTRIB_RELEASE=14.04
    DISTRIB_CODENAME=trusty
    DISTRIB_DESCRIPTION=\"Ubuntu 14.04.2 LTS\"
    ".to_string()
}

#[test]
pub fn test_parses_lsb_distro() {
    let parse_results = lsb_release::parse(file());
    assert_eq!(parse_results.distro, Some("Ubuntu".to_string()));
}

#[test]
pub fn test_parses_lsb_version() {
    let parse_results = lsb_release::parse(file());
    assert_eq!(parse_results.version, Some("14.04".to_string()));
}
