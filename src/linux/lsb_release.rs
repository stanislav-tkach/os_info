// spell-checker:ignore codename, noarch

use std::process::Command;

use log::{trace, warn};
use regex::Regex;

use crate::{Info, Type, Version};

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
        Some("RedHatEnterprise") | Some("RedHatEnterpriseServer") => Info::new(Type::RedHatEnterprise, version),
        Some("Fedora") => Info::new(Type::Fedora, version),
        Some("Amazon") | Some("AmazonAMI") => Info::new(Type::Amazon, version),
        _ => Info::new(Type::Linux, Version::unknown()),
    })
}

struct LsbRelease {
    pub distribution: Option<String>,
    pub version: Option<String>,
}

fn retrieve() -> Option<LsbRelease> {
    match Command::new("lsb_release").arg("-a").output() {
        Ok(output) => {
            trace!("lsb_release command returned {:?}", output);
            Some(parse(&String::from_utf8_lossy(&output.stdout)))
        }
        Err(e) => {
            warn!("lsb_release command failed with {:?}", e);
            None
        }
    }
}

fn parse(output: &str) -> LsbRelease {
    trace!("Trying to parse {:?}", output);

    let distribution_regex = Regex::new(r"Distributor ID:\s(\w+)").unwrap();
    let distribution = distribution_regex
        .captures_iter(output)
        .next()
        .and_then(|c| c.get(1))
        .map(|d| d.as_str().to_owned());

    let version_regex = Regex::new(r"Release:\s+([\w]+[.]?[\w]*)?").unwrap();
    let version = version_regex
        .captures_iter(output)
        .next()
        .and_then(|c| c.get(1))
        .map(|v| v.as_str().to_owned());

    trace!(
        "Parsed as '{:?}' distribution and '{:?}' version",
        distribution,
        version
    );

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

    #[test]
    pub fn ubuntu() {
        let parse_results = parse(ubuntu_file());
        assert_eq!(parse_results.distribution, Some("Ubuntu".to_string()));
        assert_eq!(parse_results.version, Some("16.04".to_string()));
    }

    #[test]
    pub fn amazon1() {
        let parse_results = parse(amazon1_file());
        assert_eq!(parse_results.distribution, Some("AmazonAMI".to_string()));
        assert_eq!(parse_results.version, Some("2018.03".to_string()));
    }

    #[test]
    pub fn amazon2() {
        let parse_results = parse(amazon2_file());
        assert_eq!(parse_results.distribution, Some("Amazon".to_string()));
        assert_eq!(parse_results.version, Some("2".to_string()));
    }

    #[test]
    pub fn redhat_enterprise_8() {
        let parse_results = parse(rhel8_file());
        assert_eq!(parse_results.distribution, Some("RedHatEnterprise".to_string()));
        assert_eq!(parse_results.version, Some("8.1".to_string()));
    }

    #[test]
    pub fn redhat_enterprise_7() {
        let parse_results = parse(rhel7_file());
        assert_eq!(parse_results.distribution, Some("RedHatEnterpriseServer".to_string()));
        assert_eq!(parse_results.version, Some("7.7".to_string()));
    }

    #[test]
    pub fn redhat_enterprise_6() {
        let parse_results = parse(rhel7_file());
        assert_eq!(parse_results.distribution, Some("RedHatEnterpriseServer".to_string()));
        assert_eq!(parse_results.version, Some("6.10".to_string()));
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

    fn ubuntu_file() -> &'static str {
        "Distributor ID: Ubuntu\n\
         Description:    Ubuntu 16.04.5 LTS\n\
         Release:        16.04\n\
         Codename:       xenial"
    }

    // Amazon Linux 1 uses a separate Distributor ID and Release format from Amazon Linux 2
    fn amazon1_file() -> &'static str {
        "LSB Version:	:base-4.0-amd64:base-4.0-noarch:core-4.0-amd64:core-4.0-noarch\n\
         Distributor ID:	AmazonAMI\n\
         Description:	Amazon Linux AMI release 2018.03\n\
         Release:	2018.03\n\
         Codename:	n/a\n\
         "
    }

    // Amazon Linux 2 uses a separate Distributor ID and Release format from Amazon Linux 1
    fn amazon2_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
         Distributor ID:	Amazon\n\
         Description:	Amazon Linux release 2 (Karoo)\n\
         Release:	2\n\
         Codename:	Karoo\n\
         "
    }

    fn rhel8_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
         Distributor ID:	RedHatEnterprise\n\
         Description:	Red Hat Enterprise Linux release 8.1 (Ootpa)\n\
         Release:	8.1\n\
         Codename:	Ootpa\n\
         "
    }

    fn rhel7_file() -> &'static str {
        "LSB Version:	:core-4.1-amd64:core-4.1-noarch\n\
        Distributor ID:	RedHatEnterpriseServer\n\
        Description:	Red Hat Enterprise Linux Server release 7.7 (Maipo)\n\
        Release:	7.7\n\
        Codename:	Maipo\n\
        "
    }

    fn rhel6_file() -> &'static str {
        "LSB Version:	:base-4.0-amd64:base-4.0-noarch:core-4.0-amd64:core-4.0-noarch:graphics-4.0-amd64:graphics-4.0-noarch:printing-4.0-amd64:printing-4.0-noarch\n\
        Distributor ID:	RedHatEnterpriseServer\n\
        Description:	Red Hat Enterprise Linux Server release 6.10 (Santiago)\n\
        Release:	6.10\n\
        Codename:	Santiago\n\
        "
    }
}

