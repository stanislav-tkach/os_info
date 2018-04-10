mod winapi;

use Info;

pub fn current_platform() -> Info {
    winapi::get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Type;

    #[test]
    fn os_type() {
        let version = current_platform();
        assert_eq!(Type::Windows, version.os_type());
    }
}
