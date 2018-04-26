// spell-checker:ignore codename, noarch

use regex::Regex;

use std::process::Command;

use {Info, Type, Version};

pub fn get() -> Option<Info> {
    let release = retrieve()?;

    let version = release
        .version
        .map_or_else(Version::unknown, |v| Version::custom(v, None));

    Some(match release.distribution.as_ref().map(String::as_ref) {
        Some("Ubuntu") => Info::new(Type::Ubuntu, version),
        Some("Debian") => Info::new(Type::Debian, version),
        Some("Arch") => Info::new(Type::Arch, version),
        Some("CentOS") => Info::new(Type::Centos, version),
        Some("Fedora") => Info::new(Type::Fedora, version),
        _ => Info::new(Type::Linux, Version::unknown()),
    })
}

struct LsbRelease {
    pub distribution: Option<String>,
    pub version: Option<String>,
}

fn retrieve() -> Option<LsbRelease> {
    let output = Command::new("lsb_release").arg("-a").output().ok()?;
    Some(parse(&String::from_utf8_lossy(&output.stdout)))
}

fn parse(file: &str) -> LsbRelease {
    let distribution_regex = Regex::new(r"Distributor ID:\s(\w+)").unwrap();
    let distribution = distribution_regex
        .captures_iter(file)
        .next()
        .and_then(|c| c.get(1))
        .map(|d| d.as_str().to_owned());

    let version_regex = Regex::new(r"Release:\s+([\w]+[.]?[\w]+?)?").unwrap();
    let version = version_regex
        .captures_iter(file)
        .next()
        .and_then(|c| c.get(1))
        .map(|v| v.as_str().to_owned());

    LsbRelease {
        distribution,
        version,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn debian() {
        let parse_results = parse(file());
        assert_eq!(parse_results.distribution, Some("Debian".to_string()));
        assert_eq!(parse_results.version, Some("7.8".to_string()));
    }

    #[test]
    pub fn arch() {
        let parse_results = parse(arch_file());
        assert_eq!(parse_results.distribution, Some("Arch".to_string()));
        assert_eq!(parse_results.version, Some("rolling".to_string()));
    }

    #[test]
    pub fn fedora() {
        let parse_results = parse(fedora_file());
        assert_eq!(parse_results.distribution, Some("Fedora".to_string()));
        assert_eq!(parse_results.version, Some("26".to_string()));
    }

    fn file() -> &'static str {
        "\nDistributor ID:	Debian\n\
         Description:	Debian GNU/Linux 7.8 (wheezy)\n\
         Release:	7.8\n\
         Codename:	wheezy\n\
         "
    }

    fn arch_file() -> &'static str {
        "\nLSB Version:	1.4\n\
         Distributor ID:	Arch\n\
         Description:	Arch Linux\n\
         Release:	rolling\n\
         Codename:	n/a"
    }

    fn fedora_file() -> &'static str {
        "\nLSB Version:    :core-4.1-amd64:core-4.1-noarch:cxx-4.1-amd64:cxx-4.1-noarch\n\
         Distributor ID: Fedora\n\
         Description:    Fedora release 26 (Twenty Six)\n\
         Release:    26\n\
         Codename:   TwentySix\n\
         "
    }
}
