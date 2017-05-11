extern crate regex;
#[path="../src/windows_ver.rs"]
mod windows_ver;
#[path="../src/utils.rs"]
mod utils;

fn output() -> String {
    "Microsoft Windows [Version 6.1.7601]".into()
}

#[test]
pub fn test_parses_version() {
    let parse_results = windows_ver::parse(output());
    assert_eq!(parse_results.version, Some("6.1.7601".into()));
}
