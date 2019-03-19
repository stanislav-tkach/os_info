use std::fmt::{self, Display, Formatter, Write};

use serde_derive::{Deserialize, Serialize};

/// Operating system version including version number and optional edition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Version {
    pub(crate) version: VersionType,
    pub(crate) edition: Option<String>,
}

/// Operating system version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum VersionType {
    /// Unknown version.
    Unknown,
    /// Semantic version (major.minor.patch).
    Semantic(u64, u64, u64),
    /// Custom version format.
    Custom(String),
}

impl Version {
    /// Constructs a new `Version` instance with unknown version and `None` edition.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Version, VersionType};
    ///
    /// let version = Version::unknown();
    /// assert_eq!(VersionType::Unknown, *version.version());
    /// assert_eq!(None, version.edition());
    /// ```
    pub fn unknown() -> Self {
        Self {
            version: VersionType::Unknown,
            edition: None,
        }
    }

    /// Constructs a new `Version` instance with semantic version and given edition.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Version, VersionType};
    ///
    /// let version = Version::semantic(0, 1, 2, None);
    /// assert_eq!(VersionType::Semantic(0, 1, 2), *version.version());
    /// assert_eq!(None, version.edition());
    /// ```
    pub fn semantic(major: u64, minor: u64, patch: u64, edition: Option<String>) -> Self {
        Self {
            version: VersionType::Semantic(major, minor, patch),
            edition,
        }
    }

    /// Construct a new `Version` instance with "custom" (non semantic) version and given edition.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Version, VersionType};
    ///
    /// let ver = "version".to_string();
    /// let edition = "edition".to_string();
    /// let version = Version::custom(ver.clone(), Some(edition.clone()));
    /// assert_eq!(VersionType::Custom(ver), *version.version());
    /// assert_eq!(Some(edition.as_ref()), version.edition());
    /// ```
    pub fn custom<T: Into<String>>(version: T, edition: Option<String>) -> Self {
        Self {
            version: VersionType::Custom(version.into()),
            edition,
        }
    }

    /// Returns operating system version. See `VersionType` for details.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::{Version, VersionType};
    ///
    /// let version = Version::unknown();
    /// assert_eq!(VersionType::Unknown, *version.version());
    pub fn version(&self) -> &VersionType {
        &self.version
    }

    /// Returns optional (can be absent) operation system edition.
    ///
    /// # Examples
    ///
    /// ```
    /// use os_info::Version;
    ///
    /// let version = Version::unknown();
    /// assert_eq!(None, version.edition());
    pub fn edition(&self) -> Option<&str> {
        self.edition.as_ref().map(String::as_ref)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(ref edition) = self.edition {
            write!(f, "{}", edition)?;
        }
        write!(f, "{}", self.version)
    }
}

impl Display for VersionType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            VersionType::Unknown => f.write_char('?'),
            VersionType::Semantic(major, minor, patch) => {
                write!(f, "{}.{}.{}", major, minor, patch)
            }
            VersionType::Custom(ref version) => write!(f, "{}", version),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown() {
        let version = Version::unknown();
        assert_eq!(VersionType::Unknown, *version.version());
        assert_eq!(None, version.edition());
    }

    #[test]
    fn semantic() {
        let data = [
            ((0, 0, 0), None),
            ((10, 20, 30), Some("edition".to_string())),
            ((3, 2, 1), None),
            ((1, 0, 0), Some("different edition".to_string())),
        ];

        for &(v, ref edition) in &data {
            let version = Version::semantic(v.0, v.1, v.2, edition.clone());
            assert_eq!(VersionType::Semantic(v.0, v.1, v.2), *version.version());
            assert_eq!(edition.as_ref().map(String::as_ref), version.edition());
        }
    }

    #[test]
    fn custom() {
        let data = [
            ("OS", None),
            ("Another OS", Some("edition".to_string())),
            ("", None),
            ("Future OS", Some("e".to_string())),
        ];

        for &(ref v, ref edition) in &data {
            let version = Version::custom(*v, edition.clone());
            assert_eq!(VersionType::Custom(v.to_string()), *version.version());
            assert_eq!(edition.as_ref().map(String::as_ref), version.edition());
        }
    }
}
