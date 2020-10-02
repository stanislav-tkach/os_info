// spell-checker:ignore dword, minwindef, ntdef, ntdll, ntstatus, osversioninfoex, osversioninfoexa
// spell-checker:ignore osversioninfoexw, serverr, sysinfoapi, winnt, winuser, pbool, libloaderapi
// spell-checker:ignore lpcstr, processthreadsapi, farproc, hkey, lpbyte, winerror, wchar, winreg
// spell-checker:ignore lstatus, osstr

#![allow(unsafe_code)]

use std::ffi::{OsStr, OsString};
use std::mem;
use std::os::windows::ffi::{OsStrExt, OsStringExt};

use winapi::{
    shared::{
        minwindef::{DWORD, FARPROC, HKEY, LPBYTE},
        ntdef::{LPCSTR, NTSTATUS},
        ntstatus::STATUS_SUCCESS,
        winerror::ERROR_SUCCESS,
    },
    um::{
        libloaderapi::{GetModuleHandleA, GetProcAddress},
        sysinfoapi::{GetSystemInfo, SYSTEM_INFO},
        winnt::{
            KEY_READ, PROCESSOR_ARCHITECTURE_AMD64, REG_SZ, VER_NT_WORKSTATION,
            VER_SUITE_WH_SERVER, WCHAR,
        },
        winreg::{RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE, LSTATUS},
        winuser::{GetSystemMetrics, SM_SERVERR2},
    },
};

use crate::{Bitness, Info, Type, Version};

#[cfg(target_arch = "x86")]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXW;

pub fn get() -> Info {
    let (version, edition) = version();
    Info {
        os_type: Type::Windows,
        version,
        edition,
        bitness: bitness(),
        ..Default::default()
    }
}

fn version() -> (Version, Option<String>) {
    match version_info() {
        None => (Version::Unknown, None),
        Some(v) => (
            Version::Semantic(
                v.dwMajorVersion as u64,
                v.dwMinorVersion as u64,
                v.dwBuildNumber as u64,
            ),
            edition(&v, release_id()),
        ),
    }
}

#[cfg(target_pointer_width = "64")]
fn bitness() -> Bitness {
    // x64 program can only run on x64 Windows.
    Bitness::X64
}

#[cfg(target_pointer_width = "32")]
fn bitness() -> Bitness {
    use winapi::{
        shared::{
            minwindef::{BOOL, FALSE, PBOOL},
            ntdef::HANDLE,
        },
        um::processthreadsapi::GetCurrentProcess,
    };

    // IsWow64Process is not available on all supported versions of Windows. Use GetModuleHandle to
    // get a handle to the DLL that contains the function and GetProcAddress to get a pointer to the
    // function if available.
    let is_wow_64 = match get_proc_address(b"kernel32\0", b"IsWow64Process\0") {
        None => return Bitness::Unknown,
        Some(val) => val,
    };

    type IsWow64 = unsafe extern "system" fn(HANDLE, PBOOL) -> BOOL;
    let is_wow_64: IsWow64 = unsafe { mem::transmute(is_wow_64) };

    let mut result = FALSE;
    if unsafe { is_wow_64(GetCurrentProcess(), &mut result) } == 0 {
        log::error!("IsWow64Process failed");
        return Bitness::Unknown;
    }

    if result == FALSE {
        Bitness::X32
    } else {
        Bitness::X64
    }
}

// Calls the Win32 API function RtlGetVersion to get the OS version information:
// https://msdn.microsoft.com/en-us/library/mt723418(v=vs.85).aspx
fn version_info() -> Option<OSVERSIONINFOEX> {
    let rtl_get_version = match get_proc_address(b"ntdll\0", b"RtlGetVersion\0") {
        None => return None,
        Some(val) => val,
    };

    type RtlGetVersion = unsafe extern "system" fn(&mut OSVERSIONINFOEX) -> NTSTATUS;
    let rtl_get_version: RtlGetVersion = unsafe { mem::transmute(rtl_get_version) };

    let mut info: OSVERSIONINFOEX = unsafe { mem::zeroed() };
    info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEX>() as DWORD;

    if unsafe { rtl_get_version(&mut info) } == STATUS_SUCCESS {
        Some(info)
    } else {
        None
    }
}

fn str_to_wide(value: &str) -> Vec<WCHAR> {
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}

fn release_id() -> Option<String> {
    const REG_SUCCESS: LSTATUS = ERROR_SUCCESS as LSTATUS;
    const MAX_CHARS: usize = 256;
    // Assume something will go wrong
    let mut rv = None;
    // Path as a wide string (array of WCHAR)
    let sub_key = str_to_wide("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
    // Registry key
    let mut key: HKEY = std::ptr::null_mut();
    // Try to open the path for reading (should never fail)
    if unsafe { RegOpenKeyExW(HKEY_LOCAL_MACHINE, sub_key.as_ptr(), 0, KEY_READ, &mut key) }
        == REG_SUCCESS
    {
        // Name as a wide string
        let name = str_to_wide("ReleaseId");
        // Where we'll be storing
        let mut data_type: DWORD = 0;
        let mut data = [0 as WCHAR; MAX_CHARS];
        let mut data_size: DWORD = std::mem::size_of::<[WCHAR; MAX_CHARS]>() as DWORD;
        // Try reading the value
        let status = unsafe {
            RegQueryValueExW(
                key,
                name.as_ptr(),
                std::ptr::null_mut(),
                &mut data_type,
                &mut data as *mut _ as LPBYTE,
                &mut data_size,
            )
        };
        // Success and the correct datatype?
        if status == REG_SUCCESS && data_type == REG_SZ {
            // RegQueryValueExW returns the number of bytes stored including the null terminator.
            // We want the string length without the terminator.
            let string_length = (data_size as usize / std::mem::size_of::<WCHAR>()) - 1;
            let as_slice =
                unsafe { std::slice::from_raw_parts(&data as *const WCHAR, string_length) };
            // Convert to a dirty UTF-8 string
            let osstr: OsString = OsStringExt::from_wide(as_slice);
            // Drop invalid characters and ensure we own it.  Finally.  A clean Rust string.
            let value = osstr.to_string_lossy().to_string();
            rv = Some(value);
        }
    }
    rv
}

// Examines data in the OSVERSIONINFOEX structure to determine the Windows edition:
// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
fn edition(version_info: &OSVERSIONINFOEX, release_id: Option<String>) -> Option<String> {
    match (
        version_info.dwMajorVersion,
        version_info.dwMinorVersion,
        version_info.wProductType,
    ) {
        // Windows 10.
        (10, 0, VER_NT_WORKSTATION) => Some("Windows 10"),
        (10, 0, _) => {
            let mut rv = Some("Windows Server 2016");
            if let Some(release_id) = release_id {
                if &release_id[..] >= "1809" {
                    rv = Some("Windows Server 2019");
                    // ...or later
                }
            }
            rv
        }
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

            if Into::<DWORD>::into(version_info.wSuiteMask) & VER_SUITE_WH_SERVER
                == VER_SUITE_WH_SERVER
            {
                Some("Windows Home Server")
            } else if version_info.wProductType == VER_NT_WORKSTATION
                && unsafe { info.u.s().wProcessorArchitecture } == PROCESSOR_ARCHITECTURE_AMD64
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

fn get_proc_address(module: &[u8], proc: &[u8]) -> Option<FARPROC> {
    assert!(
        *module.last().expect("Empty module name") == 0,
        "Module name should be zero-terminated"
    );
    assert!(
        *proc.last().expect("Empty procedure name") == 0,
        "Procedure name should be zero-terminated"
    );

    let handle = unsafe { GetModuleHandleA(module.as_ptr() as LPCSTR) };
    if handle.is_null() {
        log::error!(
            "GetModuleHandleA({}) failed",
            String::from_utf8_lossy(module)
        );
        return None;
    }

    unsafe { Some(GetProcAddress(handle, proc.as_ptr() as LPCSTR)) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn version() {
        let info = get();
        assert_eq!(Type::Windows, info.os_type());
    }

    #[test]
    fn get_version_info() {
        let version = version_info();
        assert!(version.is_some());
    }

    #[test]
    fn get_edition() {
        let test_data = [
            (10, 0, VER_NT_WORKSTATION, None, "Windows 10"),
            (10, 0, VER_NT_WORKSTATION, Some("1803"), "Windows 10"),
            (10, 0, VER_NT_WORKSTATION, Some("1809"), "Windows 10"),
            (10, 0, VER_NT_WORKSTATION, Some("2004"), "Windows 10"),
            (10, 0, 0, None, "Windows Server 2016"),
            (10, 0, 0, Some("1803"), "Windows Server 2016"),
            (10, 0, 0, Some("1809"), "Windows Server 2019"),
            (10, 0, 0, Some("2004"), "Windows Server 2019"),
            (6, 3, VER_NT_WORKSTATION, None, "Windows 8.1"),
            (6, 3, 0, None, "Windows Server 2012 R2"),
            (6, 2, VER_NT_WORKSTATION, None, "Windows 8"),
            (6, 2, 0, None, "Windows Server 2012"),
            (6, 1, VER_NT_WORKSTATION, None, "Windows 7"),
            (6, 1, 0, None, "Windows Server 2008 R2"),
            (6, 0, VER_NT_WORKSTATION, None, "Windows Vista"),
            (6, 0, 0, None, "Windows Server 2008"),
            (5, 1, 0, None, "Windows XP"),
            (5, 1, 1, None, "Windows XP"),
            (5, 1, 100, None, "Windows XP"),
            (5, 0, 0, None, "Windows 2000"),
            (5, 0, 1, None, "Windows 2000"),
            (5, 0, 100, None, "Windows 2000"),
        ];

        let mut info = version_info().unwrap();

        for (major, minor, product_type, release_id, expected_edition) in &test_data {
            info.dwMajorVersion = *major;
            info.dwMinorVersion = *minor;
            info.wProductType = *product_type;
            let release_id = release_id.map(|s| s.to_string());

            let edition = edition(&info, release_id).unwrap();
            assert_eq!(edition, *expected_edition);
        }
    }

    #[test]
    fn get_bitness() {
        let b = bitness();
        assert_ne!(b, Bitness::Unknown);
    }

    #[test]
    #[should_panic(expected = "Empty module name")]
    fn empty_module_name() {
        get_proc_address(b"", b"RtlGetVersion\0");
    }

    #[test]
    #[should_panic(expected = "Module name should be zero-terminated")]
    fn non_zero_terminated_module_name() {
        get_proc_address(b"ntdll", b"RtlGetVersion\0");
    }

    #[test]
    #[should_panic(expected = "Empty procedure name")]
    fn empty_proc_name() {
        get_proc_address(b"ntdll\0", b"");
    }

    #[test]
    #[should_panic(expected = "Procedure name should be zero-terminated")]
    fn non_zero_terminated_proc_name() {
        get_proc_address(b"ntdll\0", b"RtlGetVersion");
    }

    #[test]
    fn proc_address() {
        let address = get_proc_address(b"ntdll\0", b"RtlGetVersion\0");
        assert!(address.is_some());
    }
}
