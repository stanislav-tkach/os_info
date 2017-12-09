use std::mem::zeroed;
use std::mem::size_of;

extern crate winapi;
use self::winapi::{ NTSTATUS, DWORD, STATUS_SUCCESS };
use self::winapi::winnt::{ OSVERSIONINFOW };

#[link(name = "ntdll")]
extern {
    pub fn RtlGetVersion(lpVersionInformation: &mut OSVERSIONINFOW) -> NTSTATUS;
}

pub fn retrieve() -> Option<OSVERSIONINFOW> {
    unsafe {
        let mut osvi: OSVERSIONINFOW = { zeroed() };
        osvi.dwOSVersionInfoSize     = size_of::<OSVERSIONINFOW>() as DWORD;
        if RtlGetVersion(&mut osvi) == STATUS_SUCCESS {
            Some(osvi)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_version() {
        let info = match retrieve() {
            Some(v) => format!("{}.{}.{}", v.dwMajorVersion, v.dwMinorVersion, v.dwBuildNumber),
            None    => String::from("Unknown")
        };

        assert_ne!(info, "Unknown".to_string());
        assert_ne!(info, "0.0.0".to_string());
        println!("Windows Version: {}", info);
    }
}
