use std::fmt::{self, Display, Formatter};

use serde_derive::{Deserialize, Serialize};

use super::{Type, Version};

/// Holds information about operating system (type, version, etc.).
///
/// The best way to get string representation of the operation system information is to use its
/// `Display` implementation.
///
/// # Examples
///
/// ```
/// use os_info;
///
/// let info = os_info::get();
/// println!("OS information: {}", info);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Info {
    /// Operating system type. See `Type` for details.
    pub(crate) os_type: Type,
    /// Operating system version. See `Version` for details.
    pub(crate) version: Version,
}

impl Info {
    /// Constructs a new `Info` instance with unknown type and version.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Info, Type, Version};
    ///
    /// let info = Info::unknown();
    /// assert_eq!(Type::Unknown, info.os_type());
    /// assert_eq!(Version::unknown(), *info.version());
    /// ```
    pub fn unknown() -> Self {
        Self {
            os_type: Type::Unknown,
            version: Version::unknown(),
        }
    }

    /// Constructs a new `Info` instance with the given type and version.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Info, Type, Version};
    ///
    /// let os_type = Type::Unknown;
    /// let version = Version::unknown();
    /// let info = Info::new(os_type, version.clone());
    /// assert_eq!(os_type, info.os_type());
    /// assert_eq!(version, *info.version());
    /// ```
    pub fn new(os_type: Type, version: Version) -> Self {
        Self { os_type, version }
    }

    /// Returns operating system type. See `Type` for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Info, Type, Version};
    ///
    /// let info = Info::unknown();
    /// assert_eq!(Type::Unknown, info.os_type());
    /// ```
    pub fn os_type(&self) -> Type {
        self.os_type
    }

    /// Returns operating system version. See `Version` for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Info, Type, Version};
    ///
    /// let info = Info::unknown();
    /// assert_eq!(Version::unknown(), *info.version());
    /// ```
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl Default for Info {
    fn default() -> Self {
        Self::unknown()
    }
}

impl Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.os_type)?;
        write!(f, " ({})", self.version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    #[test]
    fn unknown() {
        let info = Info::unknown();
        assert_eq!(Type::Unknown, info.os_type());
        assert_eq!(&Version::unknown(), info.version());
    }

    #[test]
    fn new() {
        let types = [
            Type::Unknown,
            Type::Android,
            Type::Emscripten,
            Type::Linux,
            Type::Redhat,
            Type::Ubuntu,
            Type::Debian,
            Type::Arch,
            Type::Centos,
            Type::Fedora,
            Type::Alpine,
            Type::Macos,
            Type::Redox,
            Type::Windows,
        ];

        let versions = [
            Version::unknown(),
            Version::semantic(0, 0, 0, None),
            Version::semantic(1, 2, 3, Some("e".to_owned())),
            Version::custom("version".to_owned(), None),
            Version::custom("different version".to_owned(), Some("edition".to_owned())),
        ];

        for (os_type, version) in types.iter().cartesian_product(versions.iter()) {
            let info = Info::new(*os_type, version.clone());
            assert_eq!(*os_type, info.os_type());
            assert_eq!(version, info.version());
        }
    }
}
