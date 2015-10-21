extern crate regex;
use self::regex::Regex;

pub struct LsbRelease {
    pub distro: Option<String>,
    pub version: Option<String>
}

pub fn parse(file: String) -> LsbRelease {
    let distrib_regex = Regex::new(r"DISTRIB_ID=(\w+)").unwrap();
    let distrib_release_regex = Regex::new(r"DISTRIB_RELEASE=([\w\.]+)").unwrap();

    let distro = match distrib_regex.captures_iter(&file).next() {
        Some(m) => {
            match m.at(1) {
                Some(distro) => {
                    println!("Match {}", distro);
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
