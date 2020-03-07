use std::fmt::{self, Display, Formatter};

/// Operating system architecture in terms of how many bits compose the basic values it can deal with.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Bitness {
    /// Unknown bitness (unable to determine).
    Unknown,
    /// 32-bit.
    X32,
    /// 64-bit.
    X64,
}

impl Display for Bitness {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Bitness::Unknown => write!(f, "unknown"),
            Bitness::X32 => write!(f, "32-bit"),
            Bitness::X64 => write!(f, "64-bit"),
        }
    }
}
