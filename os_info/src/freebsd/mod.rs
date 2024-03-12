use std::process::Command;
use std::str;

use log::{error, trace};

use crate::{bitness, uname::uname, Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("freebsd::current_platform is called");

    let version = uname()
        .map(Version::from_string)
        .unwrap_or_else(|| Version::Unknown);

    let info = Info {
        os_type: get_os(),
        version,
        bitness: bitness::get(),
        ..Default::default()
    };

    trace!("Returning {:?}", info);
    info
}

fn get_os() -> Type {
    let os = match uname("-s") {
        Some(o) => o,
        None => return Type::Unknown,
    };

    let os = match Command::new("uname").arg("-s").output() {
        Ok(o) => o,
        Err(e) => {
            error!("Failed to invoke 'uname': {:?}", e);
            return Type::Unknown;
        }
    };

    match uname("-s") {
        None => Type::Unknown,
        Some("MidnightBSD") => Type::MidnightBSD,
        Some("FreeBSD") => {
            let check_hardening = match Command::new("/sbin/sysctl")
                .arg("hardening.version")
                .output()
            {
                Ok(o) => o,
                Err(e) => {
                    error!("Failed to invoke '/sbin/sysctl': {:?}", e);
                    return Type::FreeBSD;
                }
            };
            match str::from_utf8(&check_hardening.stderr) {
                Ok("0\n") => Type::HardenedBSD,
                Ok(_) => Type::FreeBSD,
                Err(_) => Type::FreeBSD,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::FreeBSD, version.os_type());
    }
}
