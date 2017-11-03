use std::fs::File;
use std::io::Read;

use {Type, Info, Version};

pub fn current_platform() -> Info {
    Info {
        os_type: Type::Redox,
        version: Version::unknown(),
    }
}

fn get_version() -> Option<String> {
    let mut file = match File::open("sys:uname") {
        Some(file) => file,
        _ => return None,
    };

    let mut version = String::new();
    file.read_to_string(&mut contents).ok().map(|_| version)
}
