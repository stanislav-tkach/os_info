// spell-checker:ignore dword, minwindef, ntdef, ntdll, ntstatus, osversioninfoex, osversioninfoexa
// spell-checker:ignore osversioninfoexw, serverr, sysinfoapi, winnt, winuser

#![allow(unsafe_code)]

use std::mem;

use kernel32::GetSystemInfo;
use user32::GetSystemMetrics;
#[cfg(target_arch = "x86")]
use winapi::winnt::OSVERSIONINFOEXA;
#[cfg(not(target_arch = "x86"))]
use winapi::winnt::OSVERSIONINFOEXW;
use winapi::{
    minwindef::DWORD, ntdef::NTSTATUS, ntstatus::STATUS_SUCCESS, sysinfoapi::SYSTEM_INFO,
    winuser::SM_SERVERR2,
};

use crate::{Info, Type, Version};

#[cfg(target_arch = "x86")]
type OSVERSIONINFOEX = OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
type OSVERSIONINFOEX = OSVERSIONINFOEXW;

/// Win32 Flag: VER_NT_WORKSTATION
/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
const VER_NT_WORKSTATION: u8 = 0x0000_0001;
/// Win32 Flag: VER_SUITE_WH_SERVER
/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
const VER_SUITE_WH_SERVER: u16 = 0x0000_8000;
/// Win32 Flag: PROCESSOR_ARCHITECTURE_AMD64
/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724958(v=vs.85).aspx
const PROCESSOR_ARCHITECTURE_AMD64: u16 = 9;

#[link(name = "ntdll")]
extern "system" {
    pub fn RtlGetVersion(lpVersionInformation: &mut OSVERSIONINFOEX) -> NTSTATUS;
}

pub fn get() -> Info {
    let mut info = Info::new(Type::Windows, Version::unknown());

    let version_info = match get_version_info() {
        None => {
            return info;
        }
        Some(val) => val,
    };

    info.version = Version::semantic(
        version_info.dwMajorVersion as u64,
        version_info.dwMinorVersion as u64,
        version_info.dwBuildNumber as u64,
        get_edition(&version_info),
    );

    info
}

// Calls the Win32 API function RtlGetVersion to get the OS version information:
// https://msdn.microsoft.com/en-us/library/mt723418(v=vs.85).aspx
fn get_version_info() -> Option<OSVERSIONINFOEX> {
    let mut info: OSVERSIONINFOEX = unsafe { mem::zeroed() };
    info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEX>() as DWORD;

    if unsafe { RtlGetVersion(&mut info) } == STATUS_SUCCESS {
        Some(info)
    } else {
        None
    }
}

// Examines data in the OSVERSIONINFOEX structure to determine the Windows edition:
// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
fn get_edition(version_info: &OSVERSIONINFOEX) -> Option<String> {
    match (
        version_info.dwMajorVersion,
        version_info.dwMinorVersion,
        version_info.wProductType,
    ) {
        // Windows 10.
        (10, 0, VER_NT_WORKSTATION) => Some("Windows 10"),
        (10, 0, _) => Some("Windows Server 2016"),
        // Windows Vista, 7, 8 and 8.1.
        (6, 3, VER_NT_WORKSTATION) => Some("Windows 8.1"),
        (6, 3, _) => Some("Windows Server 2012 R2"),
        (6, 2, VER_NT_WORKSTATION) => Some("Windows 8"),
        (6, 2, _) => Some("Windows Server 2012"),
        (6, 1, VER_NT_WORKSTATION) => Some("Windows 7"),
        (6, 1, _) => Some("Windows Server 2008 R2"),
        (6, 0, VER_NT_WORKSTATION) => Some("Windows Vista"),
        (6, 0, _) => Some("Windows Server 2008"),
        // Windows 2000, Home Server, 2003 Server, 2003 R2 Server, XP and XP Professional x64.
        (5, 1, _) => Some("Windows XP"),
        (5, 0, _) => Some("Windows 2000"),
        (5, 2, _) if unsafe { GetSystemMetrics(SM_SERVERR2) } == 0 => {
            let mut info: SYSTEM_INFO = unsafe { mem::zeroed() };
            unsafe { GetSystemInfo(&mut info) };

            if version_info.wSuiteMask & VER_SUITE_WH_SERVER == VER_SUITE_WH_SERVER {
                Some("Windows Home Server")
            } else if version_info.wProductType == VER_NT_WORKSTATION
                && info.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_AMD64
            {
                Some("Windows XP Professional x64 Edition")
            } else {
                Some("Windows Server 2003")
            }
        }
        _ => None,
    }
    .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let info = get();
        assert_eq!(Type::Windows, info.os_type());
    }

    #[test]
    fn version_info() {
        let version = get_version_info();
        assert!(version.is_some());
    }

    #[test]
    fn edition() {
        let test_data = [
            (10, 0, VER_NT_WORKSTATION, "Windows 10"),
            (10, 0, 0, "Windows Server 2016"),
            (6, 3, VER_NT_WORKSTATION, "Windows 8.1"),
            (6, 3, 0, "Windows Server 2012 R2"),
            (6, 2, VER_NT_WORKSTATION, "Windows 8"),
            (6, 2, 0, "Windows Server 2012"),
            (6, 1, VER_NT_WORKSTATION, "Windows 7"),
            (6, 1, 0, "Windows Server 2008 R2"),
            (6, 0, VER_NT_WORKSTATION, "Windows Vista"),
            (6, 0, 0, "Windows Server 2008"),
            (5, 1, 0, "Windows XP"),
            (5, 1, 1, "Windows XP"),
            (5, 1, 100, "Windows XP"),
            (5, 0, 0, "Windows 2000"),
            (5, 0, 1, "Windows 2000"),
            (5, 0, 100, "Windows 2000"),
        ];

        let mut info = get_version_info().unwrap();

        for &(major, minor, product_type, expected_edition) in &test_data {
            info.dwMajorVersion = major;
            info.dwMinorVersion = minor;
            info.wProductType = product_type;

            let edition = get_edition(&info).unwrap();
            assert_eq!(edition, expected_edition);
        }
    }
}
