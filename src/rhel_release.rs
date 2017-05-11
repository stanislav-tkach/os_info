use regex::Regex;

pub struct RHELRelease {
    pub distro: Option<String>
}

pub fn parse(file: String) -> RHELRelease {
    let distrib_regex = Regex::new(r"(\w+) Linux release").unwrap();
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

    RHELRelease {
        distro: distro
    }
}