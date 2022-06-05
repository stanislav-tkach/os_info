use std::fmt::{self, Display, Formatter};

/// A general category for operating system to place them into 'families'
/// Example of use case is when program logic needs to perform an operation
/// on linux, but doesnt care which distro it is.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[non_exhaustive]
pub enum Family {
    /// Berkely Standard Distributions
    /// https://en.wikipedia.org/wiki/Berkeley_Software_Distribution
    BSD,
    /// Linux Operating systems of all type
    /// https://en.wikipedia.org/wiki/Linux
    Linux,
    /// Apple's MacOS
    /// https://en.wikipedia.org/wiki/Macintosh_operating_systems
    MacOS,
    /// SunOS and OSs derived from SunOS such as Illumos
    /// https://en.wikipedia.org/wiki/SunOS
    SunOS,
    /// Operating systems whose family is unknown
    Unknown,
    /// Operating systems that are DOS or derived from DOS
    /// https://en.wikipedia.org/wiki/Disk_operating_system
    DOS,
}

impl Default for Family {
    fn default() -> Self {
	Family::Unknown
    }
}

impl Display for Family {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
	match *self {
	    Family::BSD => write!(f, "BSD"),
	    Family::Linux => write!(f, "Linux"),
	    Family::MacOS => write!(f, "MacOS"),
	    Family::SunOS => write!(f, "SunOS"),
	    Family::DOS => write!(f, "DOS"),
	    _ => write!(f, "{:?}", self),
	}
    }
}
