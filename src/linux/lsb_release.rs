// spell-checker:ignore codename, noarch

use regex::Regex;

use std::process::Command;

use {Info, Type, Version};

pub fn get() -> Option<Info> {
    let lsb_release = retrieve()?;

    let version = release
        .version
        .map_or_else(Version::unknown, |v| Version::custom(v, None));

    match release.distro.as_ref() {
        "Ubuntu" => Info { os_type: Type::Ubuntu, version },
        "Debian" => Info { os_type: Type::Debian, version },
        "Arch" => Info { os_type: Type::Arch, version },
        "CentOS" => Info { os_type: Type::Centos, version },
        "Fedora" => Info { os_type: Type::Fedora, version },
        _ => Info::new(Type::Linux, Version::unknown())
    }
}

struct LsbRelease {
    pub distro: Option<String>,
    pub version: Option<String>,
}

fn retrieve() -> Option<LsbRelease> {
    let output = Command::new("lsb_release").arg("-a").output()?;
    Some(parse(&String::from_utf8_lossy(&output.stdout)))
}

fn parse(file: &str) -> LsbRelease {
    let distro_regex = Regex::new(r"Distributor ID:\s(\w+)").unwrap();
    let distro_release_regex = Regex::new(r"Release:\s+([\w]+[.]?[\w]+?)?").unwrap();

    let distro = distro_regex
        .captures_iter(file)
        .next()
        .map(|c| c.get(1))
        .map(|d| d.as_str().to_owned());

    let version = distro_release_regex
        .captures_iter(file)
        .next()
        .map(|c| c.get(1))
        .map(|v| v.as_str().to_owned());

    LsbRelease { distro, version }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parses_lsb_distro() {
        let parse_results = parse(file());
        assert_eq!(parse_results.distro, Some("Debian".to_string()));
        assert_eq!(parse_results.version, Some("7.8".to_string()));
    }

    #[test]
    pub fn test_parses_arch_lsb_distro() {
        let parse_results = parse(arch_file());
        assert_eq!(parse_results.distro, Some("Arch".to_string()));
        assert_eq!(parse_results.version, Some("rolling".to_string()));
    }

    #[test]
    pub fn test_parses_fedora_lsb_distro() {
        let parse_results = parse(fedora_file());
        assert_eq!(parse_results.distro, Some("Fedora".to_string()));
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
