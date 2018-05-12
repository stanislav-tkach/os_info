mod lsb_release;
mod file_release;

use {Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("linux::current_platform is called");

    let info = lsb_release::get()
        .or_else(file_release::get)
        .unwrap_or_else(|| Info::new(Type::Linux, Version::unknown()));
    trace!("Returning {:?}", info);
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        match version.os_type() {
            Type::Linux
            | Type::Redhat
            | Type::Ubuntu
            | Type::Debian
            | Type::Arch
            | Type::Centos
            | Type::Fedora
            | Type::Alpine => (),
            os_type => {
                panic!("Unexpected OS type: {}", os_type);
            }
        }
    }
}
