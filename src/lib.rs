
pub mod OsType {
    use std::process::Command;
    use std::fs;

    pub enum OSType {
        Unknown,
        Redhat,
        OSX
    }

    fn file_exists(path: &String) -> bool {
        let metadata = fs::metadata(path);

        match metadata {
            Ok(md) => md.is_dir() || md.is_file(),
            Err(_) => false
        }
    }

    fn is_os_x() -> bool {
        let output = Command::new("sw_vers").output().unwrap();
        output.status.success()
    }

    pub fn current_platform() {
        fn os_type() -> OSType {
            if file_exists(&"/etc/redhat-release".to_string()) || file_exists(&"/etc/centos-release".to_string()) {
                OSType::Redhat
            } else if is_os_x() {
                OSType::OSX
            } else {
                OSType::Unknown
            }
        }
    }
}
