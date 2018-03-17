//! `os_info`
//!
//! Provides interfaces for getting information about the current operating system, such as type,
//! version and edition.

#![deny(missing_debug_implementations, missing_docs, unsafe_code)]

#[cfg(not(windows))]
extern crate regex;

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate winapi;

#[cfg(test)]
extern crate itertools;

#[cfg(target_os = "android")]
#[path = "android/mod.rs"]
mod imp;

#[cfg(target_os = "emscripten")]
#[path = "emscripten/mod.rs"]
mod imp;

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod imp;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod imp;

#[cfg(target_os = "redox")]
#[path = "redox/mod.rs"]
mod imp;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod imp;

mod info;
mod version;
mod os_type;

pub use info::Info;
pub use version::{Version, VersionType};
pub use os_type::Type;

/// Returns information about the current operating system (type, version, edition, etc.).
///
/// # Examples
///
/// ```
/// use os_info;
///
/// let info = os_info::get();
///
/// // Print full information:
/// println!("OS information: {}", info);
///
/// // Print information separately:
/// println!("Type: {}", info.os_type());
/// println!("Version: {}", info.version());
/// ```
pub fn get() -> Info {
    imp::current_platform()
}
