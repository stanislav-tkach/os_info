use std::fmt::{self, Display, Formatter, Write};

/// Operating system version including version number and optional edition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub(crate) version: VersionType,
    pub(crate) edition: Option<String>,
}

/// Operating system version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub fn unknown() -> Self {
        Self {
            version: VersionType::Unknown,
            edition: None,
        }
    }

    /// Constructs a new `Version` instance with semantic version and given edition.
    pub fn semantic(major: u64, minor: u64, patch: u64, edition: Option<String>) -> Self {
        Self {
            version: VersionType::Semantic(major, minor, patch),
            edition,
        }
    }

    /// Construct a new `Version` instance with "custom" (non semantic) version and given edition.
    pub fn custom(version: String, edition: Option<String>) -> Self {
        Self {
            version: VersionType::Custom(version),
            edition,
        }
    }

    /// Returns operating system version. See `VersionType` for details.
    pub fn version(&self) -> &VersionType {
        &self.version
    }

    /// Returns optional (can be absent) operation system edition.
    pub fn edition(&self) -> &Option<String> {
        &self.edition
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
