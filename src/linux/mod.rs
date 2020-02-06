// spell-checker:ignore getconf

mod file_release;
mod lsb_release;

use std::process::{Command, Output};

use log::trace;

use crate::{Bitness, Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("linux::current_platform is called");

    let mut info = lsb_release::get()
        .or_else(file_release::get)
        .unwrap_or_else(|| Info::new(Type::Linux, Version::unknown(), Bitness::Unknown));
    info.bitness = bitness();

    trace!("Returning {:?}", info);
    info
}

fn bitness() -> Bitness {
    match &Command::new("getconf").arg("LONG_BIT").output() {
        Ok(Output { stdout, .. }) if stdout == b"32\n" => Bitness::X32,
        Ok(Output { stdout, .. }) if stdout == b"64\n" => Bitness::X64,
        _ => Bitness::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_ne;

    #[test]
    fn os_type() {
        let version = current_platform();
        match version.os_type() {
            Type::Linux
            | Type::Redhat
            | Type::RedHatEnterprise
            | Type::Ubuntu
            | Type::Debian
            | Type::Arch
            | Type::Centos
            | Type::Fedora
            | Type::SUSE
            | Type::openSUSE
            | Type::Alpine => (),
            os_type => {
                panic!("Unexpected OS type: {}", os_type);
            }
        }
    }

    #[test]
    fn get_bitness() {
        let b = bitness();
        assert_ne!(b, Bitness::Unknown);
    }
}
