use std::process::Command;
use std::str;

use log::{error, trace};

use crate::{bitness, uname::uname, Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("aix::current_platform is called");

    let version = get_version()
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

fn get_version() -> Option<String> {
    fn parse_uname(arg: &str) -> Option<String> {
        Command::new("uname")
            .arg(arg)
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

    let major = parse_uname("-v")?;
    let minor = parse_uname("-r").unwrap_or(String::from("0"));
    Some(format!("{}.{}", major, minor))
}

fn get_os() -> Type {
    let os = Command::new("uname")
        .arg("-o")
        .output()
        .expect("Failed to get OS");

    match str::from_utf8(&os.stdout) {
        Ok("AIX\n") => Type::AIX,
        Ok(_) => Type::Unknown,
        Err(_) => Type::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::AIX, version.os_type());
    }
}
