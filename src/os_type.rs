use std::fmt::{self, Display, Formatter};

/// A list of supported operating system types.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// Unknown operating system.
    Unknown,
    /// Android (https://en.wikipedia.org/wiki/Android_(operating_system)).
    Android,
    /// Emscripten (https://en.wikipedia.org/wiki/Emscripten).
    Emscripten,
    /// Linux based operating system (https://en.wikipedia.org/wiki/Linux).
    Linux,
    /// Red Hat Linux (https://en.wikipedia.org/wiki/Red_Hat_Linux).
    Redhat,
    /// Ubuntu (https://en.wikipedia.org/wiki/Ubuntu_(operating_system)).
    Ubuntu,
    /// Debian (https://en.wikipedia.org/wiki/Debian).
    Debian,
    /// Arch Linux (https://en.wikipedia.org/wiki/Arch_Linux).
    Arch,
    /// CentOS (https://en.wikipedia.org/wiki/CentOS).
    Centos,
    /// Mac OS X/OS X/macOS (https://en.wikipedia.org/wiki/MacOS).
    Macos,
    /// Redox (https://en.wikipedia.org/wiki/Redox_(operating_system)).
    Redox,
    /// Windows (https://en.wikipedia.org/wiki/Microsoft_Windows).
    Windows,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Type::Redhat => write!(f, "Red Hat Linux"),
            Type::Arch   => write!(f, "Arch Linux"),
            Type::Centos => write!(f, "CentOS"),
            Type::Macos  => write!(f, "Mac OS"),
            _ => write!(f, "{:?}", self),
        }
    }
}
