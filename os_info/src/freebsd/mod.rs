use std::process::Command;

use log::{error, trace};

use crate::{bitness, Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("freebsd::current_platform is called");

    let version = uname
        .map(Version::from_string)
        .unwrap_or_else(|| Version::Unknown);

    let info = Info {
        os_type: Type::FreeBSD,
        version,
        bitness: bitness::get(),
        ..Default::default()
    };

    trace!("Returning {:?}", info);
    info
}

fn uname() -> Option<String> {
    Command::new("uname")
        .arg("-r")
        .output()
        .map_err(|e| {
            error!("Failed to invoke 'uname': {:?}", e);
        })
        .ok()
        .and_then(|out| {
            if out.status.success() {
                Some(String::from_utf8_lossy(&out.stdout).trim_end().to_owned())
            } else {
                log::error!("'uname' invocation error: {:?}", out);
                None
            }
        })
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
