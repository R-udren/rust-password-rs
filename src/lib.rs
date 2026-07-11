//! Read Rust game data from the current user's Windows registry.

#[cfg(windows)]
mod windows;

use thiserror::Error;

/// Registry data displayed by the application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub history: Vec<String>,
    pub last_code: String,
    pub steam: Steam,
    pub rust: Game,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Steam {
    pub last_game: String,
    pub steam_id: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub name: String,
    pub installed: bool,
    pub running: bool,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Rust registry data is only available on Windows")]
    Unsupported,
    #[error("could not open the Rust registry key: {0}")]
    Open(String),
    #[error("could not enumerate the Rust registry key: {0}")]
    Read(String),
    #[error("could not open registry key '{path}': {reason}")]
    OpenKey { path: &'static str, reason: String },
    #[error("could not read registry value '{name}' from '{path}': {reason}")]
    ReadValue {
        path: &'static str,
        name: &'static str,
        reason: String,
    },
    #[error("registry value '{name}' from '{path}' must be 0 or 1, got {value}")]
    InvalidFlag {
        path: &'static str,
        name: &'static str,
        value: u32,
    },
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

fn steam_id(account_id: u32) -> Option<u64> {
    const INDIVIDUAL_ACCOUNT_BASE: u64 = 76_561_197_960_265_728;
    (account_id != 0).then(|| INDIVIDUAL_ACCOUNT_BASE + u64::from(account_id))
}

fn flag(path: &'static str, name: &'static str, value: u32) -> Result<bool, Error> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        value => Err(Error::InvalidFlag { path, name, value }),
    }
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

    #[test]
    fn converts_account_id_to_steam_id_64() {
        assert_eq!(steam_id(1_025_200_509), Some(76_561_198_985_466_237));
        assert_eq!(steam_id(0), None);
    }

    #[test]
    fn accepts_binary_registry_flags_only() {
        assert!(!flag("key", "flag", 0).unwrap());
        assert!(flag("key", "flag", 1).unwrap());
        assert!(flag("key", "flag", 2).is_err());
    }
}
