use log::trace;

use crate::{Info, Type, Version};

pub fn current_platform() -> Info {
    trace!("android::current_platform is called");

    let info = Info {
        os_type: Type::Android,
        version: Version::unknown(),
    };
    trace!("Returning {:?}", info);
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Android, version.os_type());
    }
}
