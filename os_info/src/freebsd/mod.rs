use crate::{bitness, Info, Type, Version};
use std::process::Command;

fn uname(arg: &str) -> Option<String> {
    Command::new("uname")
        .args(&[arg])
        .output()
        .ok()
        .and_then(|out| {
            if out.status.success() {
                String::from_utf8(out.stdout)
                    .ok()
                    .map(|sz| sz.trim_end().to_string())
            } else {
                None
            }
        })
}

pub fn current_platform() -> Info {
    let version = uname("-r")
        .map(Version::from_string)
        .unwrap_or_else(|| Version::Unknown);
    let info = Info {
        os_type: Type::FreeBSD,
        version,
        bitness: bitness::get(),
        ..Default::default()
    };
    info
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
