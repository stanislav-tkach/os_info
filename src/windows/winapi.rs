// spell-checker:ignore dword, minwindef, ntdef, ntdll, ntstatus, osversioninfoex, osversioninfoexa
// spell-checker:ignore osversioninfoexw, serverr, sysinfoapi, winnt, winuser

#![allow(unsafe_code)]

use std::mem;

use winapi::{
    shared::{minwindef::DWORD, ntdef::NTSTATUS, ntstatus::STATUS_SUCCESS},
    um::{
        sysinfoapi::GetSystemInfo,
        sysinfoapi::SYSTEM_INFO,
        winnt::{PROCESSOR_ARCHITECTURE_AMD64, VER_NT_WORKSTATION, VER_SUITE_WH_SERVER},
        winuser::GetSystemMetrics,
        winuser::SM_SERVERR2,
    },
};
#[cfg(target_pointer_width = "32")]
use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleA};

use crate::{Bitness, Info, Type, Version};
use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleA};

#[cfg(target_arch = "x86")]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXW;

#[link(name = "ntdll")]
extern "system" {
    pub fn RtlGetVersion(lpVersionInformation: &mut OSVERSIONINFOEX) -> NTSTATUS;
}

pub fn get() -> Info {
    Info::new(Type::Windows, version(), bitness())
}

fn version() -> Version {
    match version_info() {
        None => Version::unknown(),
        Some(v) => Version::semantic(
            v.dwMajorVersion as u64,
            v.dwMinorVersion as u64,
            v.dwBuildNumber as u64,
            edition(&v),
        )
    }
}

fn bitness() -> Bitness {
    let my_directory = if cfg!(target_pointer_width = "64") {
        // x64 program can only run on x64 Windows.
        Bitness::X64,
    } else {
        match isWow64() {
            None => Bitness::Unknown,
            Some(true) => Bitness::X64,
            Some(false) => Bitness::X32,
        }
    }
}

fn isWow64() -> Option<bool> {
    // IsWow64Process is not available on all supported versions of Windows. Use GetModuleHandle to
    // get a handle to the DLL that contains the function and GetProcAddress to get a pointer to the
    // function if available.
    let kernel = unsafe { GetModuleHandleA("kernel32") };
    if kernel.is_null() {
        // TODO: Log error.
        return None;
    }

    let proc = unsafe { GetProcAddress(kernel, "IsWow64Process") };
    if proc.is_null() {
        // TODO: Log error.
        return None;
    }

    BOOL result = false;
    // TODO: FIXME: Handle errors (GetCurrentProcess).
    if !proc(GetCurrentProcess(), &result) {
        // TODO: Log error.
        return None;
    }

    Some(result)
}

/*
BOOL IsWow64()
{
    BOOL bIsWow64 = FALSE;

    //IsWow64Process is not available on all supported versions of Windows.
    //Use GetModuleHandle to get a handle to the DLL that contains the function
    //and GetProcAddress to get a pointer to the function if available.

    fnIsWow64Process = (LPFN_ISWOW64PROCESS) GetProcAddress(
        GetModuleHandle(TEXT("kernel32")),"IsWow64Process");

    if(NULL != fnIsWow64Process)
    {
        if (!fnIsWow64Process(GetCurrentProcess(),&bIsWow64))
        {
            //handle error
        }
    }
    return bIsWow64;
}
*/

// Calls the Win32 API function RtlGetVersion to get the OS version information:
// https://msdn.microsoft.com/en-us/library/mt723418(v=vs.85).aspx
fn version_info() -> Option<OSVERSIONINFOEX> {
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
fn edition(version_info: &OSVERSIONINFOEX) -> Option<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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

        let mut info = version_info().unwrap();

        for &(major, minor, product_type, expected_edition) in &test_data {
            info.dwMajorVersion = major;
            info.dwMinorVersion = minor;
            info.wProductType = product_type;

            let edition = edition(&info).unwrap();
            assert_eq!(edition, expected_edition);
        }
    }
}
