mod lsb_release;
mod file_release;

use {Info, Type, Version};

pub fn current_platform() -> Info {
    if let Some(info) = lsb_release::get() {
        info
    } else {
        file_release::get()
    }
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
