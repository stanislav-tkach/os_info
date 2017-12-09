use std::mem::zeroed;
use std::mem::size_of;

extern crate winapi;
use self::winapi::{ NTSTATUS, DWORD, STATUS_SUCCESS };
use self::winapi::winnt::{ OSVERSIONINFOEXW };
use self::winapi::sysinfoapi::{ SYSTEM_INFO };

/// Win32 Flag: VER_NT_WORKSTATION
///  https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
const VER_NT_WORKSTATION: u8  = 0x0000001;
/// Win32 Flag: VER_SUITE_WH_SERVER
///  https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
const VER_SUITE_WH_SERVER: u16 = 0x00008000;
/// Win32 Flag: SM_SERVERR2
/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724385(v=vs.85).aspx
const SM_SERVERR2: u8  = 89;
/// Win32 Flag: PROCESSOR_ARCHITECTURE_AMD64
/// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724958(v=vs.85).aspx
const PROCESSOR_ARCHITECTURE_AMD64: u16 = 9;

#[link(name = "ntdll")]
extern {
    pub fn RtlGetVersion(lpVersionInformation: &mut OSVERSIONINFOEXW) -> NTSTATUS;
}

#[link(name = "user32")]
extern {
    pub fn GetSystemMetrics(nIndex: u8) -> u8;
}

#[link(name = "kernel32")]
extern {
    pub fn GetSystemInfo(lpSystemInfo: &mut SYSTEM_INFO);
}


/// Win32Version Structure
/// Holds information about the current systems version data and edition
pub struct Win32Version {
    pub osvi   : Option<OSVERSIONINFOEXW>,
    pub edition: Option<String>
}

/// Win32Version Structure Implementation
/// A set of functions that will set the data in the Win32Version struct
/// osvi() will call edition() each time the function is called because a new
/// structure is returned. However, edition() will only call osvi() if the
/// osvi data has not been previously set.
impl Win32Version {
    pub fn osvi() -> Self {
        unsafe {
            let mut info: OSVERSIONINFOEXW = { zeroed() };
            info.dwOSVersionInfoSize       = size_of::<OSVERSIONINFOEXW>() as DWORD;

            if RtlGetVersion(&mut info) == STATUS_SUCCESS {
                Self { osvi: Some(info), ..Default::default() }.edition()
            } else {
                Self { osvi: None, ..Default::default() }.edition()
            }
        }
    }

    pub fn edition(self) -> Self {
        let mut info = match self.osvi {
            Some(_) => self,
            None    => Self::osvi()
        };

        let osvi = info.osvi.unwrap();
        match osvi.dwMajorVersion {
            // Windows 10
            10 => {
                match osvi.dwMinorVersion {
                    0 => match osvi.wProductType {
                        VER_NT_WORKSTATION => info.edition = Some("Windows 10".to_string()),
                        _                  => info.edition = Some("Windows Server 2016".to_string())
                    },
                    _ => info.edition = None
                }
            },
            // Windows Vista, 7, 8 && 8.1
            6 => {
                match osvi.dwMinorVersion {
                    3 => match osvi.wProductType {
                        VER_NT_WORKSTATION => info.edition = Some("Windows 8.1".to_string()),
                        _                  => info.edition = Some("Windows Server 2012 R2".to_string())
                    },
                    2 => match osvi.wProductType {
                        VER_NT_WORKSTATION => info.edition = Some("Windows 8".to_string()),
                        _                  => info.edition = Some("Windows Server 2012".to_string())
                    },
                    1 => match osvi.wProductType {
                        VER_NT_WORKSTATION => info.edition = Some("Windows 7".to_string()),
                        _                  => info.edition = Some("Windows Server 2008 R2".to_string())
                    },
                    0 => match osvi.wProductType {
                        VER_NT_WORKSTATION => info.edition = Some("Windows Vista".to_string()),
                        _                  => info.edition = Some("Windows Server 2008".to_string())
                    },
                    _ => info.edition = None
                }
            },
            // Windows 2000 and XP
            // Windows Home Server, 2003 Server, 2003 R2 Server and XP Professional x64
            5 => {
                match osvi.dwMinorVersion {
                    // Windows Home Server, 2003 Server, 2003 R2 Server and XP Professional x64
                    2 => match unsafe { GetSystemMetrics(SM_SERVERR2) } {
                        0  => {
                            let mut sysinfo: SYSTEM_INFO = unsafe {
                                zeroed()
                            };
                            unsafe {
                                GetSystemInfo(&mut sysinfo)
                            };

                            if osvi.wSuiteMask & VER_SUITE_WH_SERVER == VER_SUITE_WH_SERVER {
                                info.edition = Some("Windows Home Server".to_string())
                            } else if osvi.wProductType == VER_NT_WORKSTATION &&
                                 sysinfo.wProcessorArchitecture == PROCESSOR_ARCHITECTURE_AMD64
                            {
                                 info.edition = Some("Windows XP Professional x64 Edition".to_string())
                            } else {
                                info.edition = Some("Windows Server 2003".to_string())
                            }
                        },
                        _  => info.edition = Some("Windows Server 2003 R2".to_string())
                    },
                    // Windows 2000 and XP
                    1 => info.edition = Some("Windows XP".to_string()),
                    0 => info.edition = Some("Windows 2000".to_string()),
                    _ => info.edition = None
                }
            },
            _  => info.edition = None
        }

        return info;
    }
}

/// Win32Version Structure Default Settings
/// Both osvi and edition default to None/
impl Default for Win32Version {
    fn default() -> Self {
        Self {
            osvi    : None,
            edition : None
        }
    }
}

/// retrieve()
/// Public function used to gather version information
pub fn retrieve() -> Win32Version {
    Win32Version::osvi()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_version() {
        let version = retrieve();
        let info = match version.osvi {
            Some(v) => format!("{}.{}.{}", v.dwMajorVersion, v.dwMinorVersion, v.dwBuildNumber),
            None    => String::from("Unknown")
        };

        let edition = match version.edition {
            Some(v) => format!("{}", v),
            None    => String::from("Unknown")
        };

        assert_ne!(info,    "Unknown".to_string());
        assert_ne!(info,    "0.0.0".to_string());
        assert_ne!(edition, "Unknown".to_string());
        println!("Windows Version: {}", info);
        println!("Windows Edition: {}", edition);
    }
}
