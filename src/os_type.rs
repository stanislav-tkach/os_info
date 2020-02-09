use std::fmt::{self, Display, Formatter};

#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

/// A list of supported operating system types.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    /// Unknown operating system.
    Unknown,
    /// Android (<https://en.wikipedia.org/wiki/Android_(operating_system)>).
    Android,
    /// Emscripten (<https://en.wikipedia.org/wiki/Emscripten>).
    Emscripten,
    /// Linux based operating system (<https://en.wikipedia.org/wiki/Linux>).
    Linux,
    /// Red Hat Linux (<https://en.wikipedia.org/wiki/Red_Hat_Linux>).
    Redhat,
    /// Red Hat Enterprise Linux (<https://en.wikipedia.org/wiki/Red_Hat_Enterprise_Linux>).
    RedHatEnterprise,
    /// Ubuntu (<https://en.wikipedia.org/wiki/Ubuntu_(operating_system)>).
    Ubuntu,
    /// Debian (<https://en.wikipedia.org/wiki/Debian>).
    Debian,
    /// Arch Linux (<https://en.wikipedia.org/wiki/Arch_Linux>).
    Arch,
    /// CentOS (<https://en.wikipedia.org/wiki/CentOS>).
    Centos,
    /// Fedora (<https://en.wikipedia.org/wiki/Fedora_(operating_system)>)
    Fedora,
    /// Amazon (<https://en.wikipedia.org/wiki/Amazon_Machine_Image#Amazon_Linux_AMI>)
    Amazon,
    /// SUSE Linux Enterprise (<https://en.wikipedia.org/wiki/SUSE_Linux_Enterprise>)
    SUSE,
    ///openSUSE Linux (<https://en.wikipedia.org/wiki/OpenSUSE>)
    openSUSE,
    /// Alpine Linux (<https://en.wikipedia.org/wiki/Alpine_Linux>)
    Alpine,
    /// Mac OS X/OS X/macOS (<https://en.wikipedia.org/wiki/MacOS>).
    Macos,
    /// Redox (<https://en.wikipedia.org/wiki/Redox_(operating_system)>).
    Redox,
    /// Windows (<https://en.wikipedia.org/wiki/Microsoft_Windows>).
    Windows,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Type::Redhat => write!(f, "Red Hat Linux"),
            Type::Arch => write!(f, "Arch Linux"),
            Type::Centos => write!(f, "CentOS"),
            Type::Macos => write!(f, "Mac OS"),
            _ => write!(f, "{:?}", self),
        }
    }
}
