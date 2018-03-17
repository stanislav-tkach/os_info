// spell-checker:ignore dword, minwindef, ntdef, ntdll, ntstatus, osversioninfoex, osversioninfoexa
// spell-checker:ignore osversioninfoexw, osvi, serverr, sysinfo, sysinfoapi, winnt, winuser

#![allow(unsafe_code)]

use std::mem::zeroed;
use std::mem::size_of;
use winapi::ntdef::NTSTATUS;
use winapi::minwindef::DWORD;
use winapi::ntstatus::STATUS_SUCCESS;
use winapi::sysinfoapi::SYSTEM_INFO;
use winapi::winuser::SM_SERVERR2;
use user32::GetSystemMetrics;
use kernel32::GetSystemInfo;

#[cfg(target_arch = "x86")]
use winapi::winnt::OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
use winapi::winnt::OSVERSIONINFOEXW;

#[cfg(target_arch = "x86")]
type OSVERSIONINFOEX = OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
type OSVERSIONINFOEX = OSVERSIONINFOEXW;

/// Win32 Flag: VER_NT_WORKSTATION
///  https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
const VER_NT_WORKSTATION: u8 = 0x0000001;
/// Win32 Flag: VER_SUITE_WH_SERVER
///  https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
const VER_SUITE_WH_SERVER: u16 = 0x00008000;
/// Win32 Flag: PROCESSOR_ARCHITECTURE_AMD64
/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724958(v=vs.85).aspx
const PROCESSOR_ARCHITECTURE_AMD64: u16 = 9;

#[link(name = "ntdll")]
extern "C" {
    pub fn RtlGetVersion(lpVersionInformation: &mut OSVERSIONINFOEX) -> NTSTATUS;
}

/// Win32Version Structure
/// Holds information about the current systems version data and edition
pub struct Win32Version {
    pub osvi: Option<OSVERSIONINFOEX>,
    pub edition: Option<String>,
}

/// Win32Version Structure Implementation
/// A set of functions that will set the data in the Win32Version struct
/// osvi() will call edition() each time the function is called because a new
/// structure is returned. However, edition() will only call osvi() if the
/// osvi data has not been previously set.
impl Win32Version {
    /// Win32Version::osvi()
    /// Call the Win32 API function RtlGetVersion to get the OS version information
    /// https://msdn.microsoft.com/en-us/library/mt723418(v=vs.85).aspx
    pub fn osvi() -> Self {
        unsafe {
            let mut info: OSVERSIONINFOEX = {
                zeroed()
            };
            info.dwOSVersionInfoSize = size_of::<OSVERSIONINFOEX>() as DWORD;

            if RtlGetVersion(&mut info) == STATUS_SUCCESS {
                Self {
                    osvi: Some(info),
                    ..Default::default()
                }.edition()
            } else {
                Self {
                    osvi: None,
                    ..Default::default()
                }.edition()
            }
        }
    }

    /// Win32Version::edition()
    /// Examine data in an OSVERSIONINFOEX structure to determine the Windows edition
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
    pub fn edition(self) -> Self {
        let mut info = match self.osvi {
            Some(_) => self,
            None => Self::osvi(),
        };

        if info.osvi.is_none() {
            return info;
        }

        let osvi = info.osvi.unwrap();
        match osvi.dwMajorVersion {
            // Windows 10
            10 => {
                match osvi.dwMinorVersion {
                    0 => {
                        match osvi.wProductType {
                            VER_NT_WORKSTATION => info.edition = Some("Windows 10".to_string()),
                            _ => info.edition = Some("Windows Server 2016".to_string()),
                        }
                    }
                    _ => info.edition = None,
                }
            }
            // Windows Vista, 7, 8 && 8.1
            6 => {
                match osvi.dwMinorVersion {
                    3 => {
                        match osvi.wProductType {
                            VER_NT_WORKSTATION => info.edition = Some("Windows 8.1".to_string()),
                            _ => info.edition = Some("Windows Server 2012 R2".to_string()),
                        }
                    }
                    2 => {
                        match osvi.wProductType {
                            VER_NT_WORKSTATION => info.edition = Some("Windows 8".to_string()),
                            _ => info.edition = Some("Windows Server 2012".to_string()),
                        }
                    }
                    1 => {
                        match osvi.wProductType {
                            VER_NT_WORKSTATION => info.edition = Some("Windows 7".to_string()),
                            _ => info.edition = Some("Windows Server 2008 R2".to_string()),
                        }
                    }
                    0 => {
                        match osvi.wProductType {
                            VER_NT_WORKSTATION => info.edition = Some("Windows Vista".to_string()),
                            _ => info.edition = Some("Windows Server 2008".to_string()),
                        }
                    }
                    _ => info.edition = None,
                }
            }
            // Windows 2000 and XP
            // Windows Home Server, 2003 Server, 2003 R2 Server and XP Professional x64
            5 => {
                match osvi.dwMinorVersion {
                    // Windows Home Server, 2003 Server, 2003 R2 Server and XP Professional x64
                    2 => {
                        match unsafe { GetSystemMetrics(SM_SERVERR2) } {
                            0 => {
                                let mut sysinfo: SYSTEM_INFO = unsafe { zeroed() };
                                unsafe { GetSystemInfo(&mut sysinfo) };

                                if osvi.wSuiteMask & VER_SUITE_WH_SERVER == VER_SUITE_WH_SERVER {
                                    info.edition = Some("Windows Home Server".to_string())
                                } else if osvi.wProductType == VER_NT_WORKSTATION &&
                                           sysinfo.wProcessorArchitecture ==
                                               PROCESSOR_ARCHITECTURE_AMD64
                                {
                                    info.edition =
                                        Some("Windows XP Professional x64 Edition".to_string())
                                } else {
                                    info.edition = Some("Windows Server 2003".to_string())
                                }
                            }
                            _ => info.edition = Some("Windows Server 2003 R2".to_string()),
                        }
                    }
                    // Windows 2000 and XP
                    1 => info.edition = Some("Windows XP".to_string()),
                    0 => info.edition = Some("Windows 2000".to_string()),
                    _ => info.edition = None,
                }
            }
            _ => info.edition = None,
        }

        return info;
    }
}

/// Win32Version Structure Default Settings
/// Both osvi and edition default to None/
impl Default for Win32Version {
    fn default() -> Self {
        Self {
            osvi: None,
            edition: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_version() {
        let version = Win32Version::osvi();
        let info = match version.osvi {
            Some(v) => {
                format!(
                    "{}.{}.{}",
                    v.dwMajorVersion,
                    v.dwMinorVersion,
                    v.dwBuildNumber
                )
            }
            None => String::from("Unknown"),
        };

        let edition = match version.edition {
            Some(v) => format!("{}", v),
            None => String::from("Unknown"),
        };

        assert_ne!(info, "Unknown".to_string());
        assert_ne!(info, "0.0.0".to_string());
        assert_ne!(edition, "Unknown".to_string());
        println!("Windows Version: {}", info);
        println!("Windows Edition: {}", edition);
    }
}
