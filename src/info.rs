use std::fmt::{self, Display, Formatter};

use super::{Type, Version};

/// Holds information about operating system (type, version, etc.).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Info {
    /// Operating system type. See `Type` for details.
    pub(crate) os_type: Type,
    /// Operating system version. See `Version` for details.
    pub(crate) version: Version,
}

impl Info {
    /// Constructs a new `Info` instance with unknown type and version.
    pub fn unknown() -> Self {
        Self {
            os_type: Type::Unknown,
            version: Version::unknown(),
        }
    }

    /// Constructs a new `Info` instance with the given type and version.
    pub fn new(os_type: Type, version: Version) -> Self {
        Self { os_type, version }
    }

    /// Returns operating system type. See `Type` for details.
    pub fn os_type(&self) -> &Type {
        &self.os_type
    }

    /// Returns operating system version. See `Version` for details.
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
        write!(f, "{}", self.version)
    }
}
