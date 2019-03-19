use log::trace;

use crate::{Info, Type, Version};

// TODO: Somehow get the real OS version?
pub fn current_platform() -> Info {
    trace!("emscripten::current_platform is called");

    let info = Info {
        os_type: Type::Emscripten,
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
        assert_eq!(Type::Emscripten, version.os_type());
    }
}
