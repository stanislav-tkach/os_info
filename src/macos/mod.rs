/*
 * Mac OS X related checks
 */

use regex::Regex;

use std::process::Command;

use {Type, Info, Version};

pub fn current_platform() -> Info {
    let version = retrieve().map(|x| x.product_version).unwrap_or_else(
        Version::unknown,
    );
    Info {
        os_type: Type::Macos,
        version,
    }
}

struct SwVers {
    pub product_name: Option<String>,
    pub product_version: Option<String>,
    pub build_version: Option<String>,
}

fn extract_from_regex(stdout: &str, regex: &Regex) -> Option<String> {
    match regex.captures_iter(stdout).next() {
        Some(m) => {
            match m.get(1) {
                Some(s) => Some(s.as_str().to_owned()),
                None => None,
            }
        }
        None => None,
    }
}

fn retrieve() -> Option<SwVers> {
    let output = match Command::new("sw_vers").output() {
        Ok(output) => output,
        Err(_) => return None,
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    Some(parse(&stdout))
}

fn parse(version_str: &str) -> SwVers {
    let product_name_regex = Regex::new(r"ProductName:\s([\w\s]+)\n").unwrap();
    let product_version_regex = Regex::new(r"ProductVersion:\s(\w+\.\w+\.\w+)").unwrap();
    let build_number_regex = Regex::new(r"BuildVersion:\s(\w+)").unwrap();

    SwVers {
        product_name: extract_from_regex(version_str, &product_name_regex),
        product_version: extract_from_regex(version_str, &product_version_regex),
        build_version: extract_from_regex(version_str, &build_number_regex),
    }
}
