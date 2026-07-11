//! Read Rust game data from the current user's Windows registry.

#[cfg(windows)]
mod windows;

use thiserror::Error;

/// Registry data displayed by the application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub history: Vec<String>,
    pub last_code: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Rust registry data is only available on Windows")]
    Unsupported,
    #[error("could not open the Rust registry key: {0}")]
    Open(String),
    #[error("could not enumerate the Rust registry key: {0}")]
    Read(String),
    #[error("registry value '{0}' was not found")]
    Missing(&'static str),
    #[error("multiple registry values matched '{0}'")]
    Duplicate(&'static str),
    #[error("registry value '{name}' is not binary data")]
    WrongType { name: String },
    #[error("registry value '{name}' is not valid UTF-8")]
    InvalidText {
        name: String,
        #[source]
        source: std::str::Utf8Error,
    },
}

/// Read the latest console history and entered code.
pub fn scan() -> Result<Data, Error> {
    #[cfg(windows)]
    return windows::scan();

    #[cfg(not(windows))]
    Err(Error::Unsupported)
}

fn matches(name: &str, key: &str) -> bool {
    if name == key {
        return true;
    }

    let Some(hash) = name
        .strip_prefix(key)
        .and_then(|suffix| suffix.strip_prefix("_h"))
    else {
        return false;
    };

    !hash.is_empty() && hash.bytes().all(|byte| byte.is_ascii_digit())
}

fn text(name: &str, bytes: &[u8]) -> Result<String, Error> {
    let bytes = bytes.strip_suffix(&[0]).unwrap_or(bytes);
    std::str::from_utf8(bytes)
        .map(str::to_owned)
        .map_err(|source| Error::InvalidText {
            name: name.to_owned(),
            source,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_plain_and_hashed_names() {
        assert!(matches("lastCodeEntered", "lastCodeEntered"));
        assert!(matches("lastCodeEntered_h92736747", "lastCodeEntered"));
        assert!(!matches("lastCodeEntered_h", "lastCodeEntered"));
        assert!(!matches("lastCodeEntered_hash", "lastCodeEntered"));
    }

    #[test]
    fn decodes_null_terminated_utf8() {
        assert_eq!(text("code", b"7412\0").unwrap(), "7412");
        assert_eq!(text("code", b"7412").unwrap(), "7412");
        assert!(text("code", &[0xff, 0]).is_err());
    }
}
