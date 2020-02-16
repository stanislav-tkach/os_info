/// An implementation to match on simple strings.
#[derive(Debug, Clone)]
pub(crate) enum Matcher {
    /// Considers the entire string (trimmed) to be the match.
    AllTrimmed,

    /// After finding the `prefix` followed by one or more spaces, returns the following word.
    #[cfg(not(target_os = "macos"))]
    PrefixedWord { prefix: &'static str },

    /// Similar to `PrefixedWord`, but only if the word is a valid version.
    PrefixedVersion { prefix: &'static str },
}

impl Matcher {
    /// Find the match on the input `string`.
    pub(crate) fn find(&self, string: &str) -> Option<String> {
        match *self {
            Self::AllTrimmed => Some(string.trim().to_string()),
            #[cfg(not(target_os = "macos"))]
            Self::PrefixedWord { prefix } => {
                find_prefixed_word(string, prefix).map(|v| v.to_owned())
            }
            Self::PrefixedVersion { prefix } => find_prefixed_word(string, prefix)
                .filter(|&v| is_valid_version(v))
                .map(|v| v.to_owned()),
        }
    }
}

fn find_prefixed_word<'a>(string: &'a str, prefix: &str) -> Option<&'a str> {
    if let Some(prefix_start) = string.find(prefix) {
        // Ignore prefix and leading whitespace
        let string = &string[prefix_start + prefix.len()..].trim_start();

        // Find where the word boundary ends
        let word_end = string
            .find(|c: char| c.is_whitespace())
            .unwrap_or_else(|| string.len());
        let string = &string[..word_end];

        Some(string)
    } else {
        None
    }
}

fn is_valid_version(word: &str) -> bool {
    !word.starts_with('.') && !word.ends_with('.')
}
