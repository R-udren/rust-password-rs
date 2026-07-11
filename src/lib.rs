//! Read Rust game data from the current user's Windows registry.

#[cfg(windows)]
mod windows;

use thiserror::Error;

/// Registry data displayed by the application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    /// Saved console commands in registry order.
    pub history: Vec<String>,
    /// Most recently entered server code.
    pub last_code: String,
    /// Current Steam user information.
    pub steam: Steam,
    /// Rust's Steam application state.
    pub rust: Game,
}

/// Steam account information.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Steam {
    /// Name saved as Steam's last used game name.
    pub last_game: String,
    /// Active account converted to a `SteamID64`, when logged in.
    pub steam_id: Option<u64>,
}

/// Rust's Steam application state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    /// Registered application name.
    pub name: String,
    /// Whether Steam reports Rust as installed.
    pub installed: bool,
    /// Whether Steam reports Rust as running.
    pub running: bool,
}

/// Failure reading or decoding supported registry values.
#[derive(Debug, Error)]
pub enum Error {
    /// Registry scanning was requested on a non-Windows platform.
    #[error("Rust registry data is only available on Windows")]
    Unsupported,
    /// The primary Rust registry key could not be opened.
    #[error("could not open the Rust registry key: {0}")]
    Open(String),
    /// The primary Rust registry key could not be enumerated.
    #[error("could not enumerate the Rust registry key: {0}")]
    Read(String),
    /// A supporting registry key could not be opened.
    #[error("could not open registry key '{path}': {reason}")]
    OpenKey {
        /// Registry key path.
        path: &'static str,
        /// Operating-system error description.
        reason: String,
    },
    /// A supporting registry value could not be read.
    #[error("could not read registry value '{name}' from '{path}': {reason}")]
    ReadValue {
        /// Registry key path.
        path: &'static str,
        /// Registry value name.
        name: &'static str,
        /// Operating-system error description.
        reason: String,
    },
    /// A registry boolean contained a value other than zero or one.
    #[error("registry value '{name}' from '{path}' must be 0 or 1, got {value}")]
    InvalidFlag {
        /// Registry key path.
        path: &'static str,
        /// Registry value name.
        name: &'static str,
        /// Unexpected numeric value.
        value: u32,
    },
    /// A required registry value was absent.
    #[error("registry value '{0}' was not found")]
    Missing(&'static str),
    /// More than one dynamic registry value matched a required field.
    #[error("multiple registry values matched '{0}'")]
    Duplicate(&'static str),
    /// A Rust registry value used an unexpected registry type.
    #[error("registry value '{name}' is not binary data")]
    WrongType {
        /// Registry value name.
        name: String,
    },
    /// A Rust registry value was not valid UTF-8.
    #[error("registry value '{name}' is not valid UTF-8")]
    InvalidText {
        /// Registry value name.
        name: String,
        #[source]
        /// UTF-8 decoding failure.
        source: std::str::Utf8Error,
    },
}

/// Read the latest console history and entered code.
///
/// # Errors
///
/// Returns an [`Error`] when a required registry key or value is missing,
/// malformed, or inaccessible.
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

const fn flag(path: &'static str, name: &'static str, value: u32) -> Result<bool, Error> {
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
        assert!(matches!(text("code", b"7412\0"), Ok(value) if value == "7412"));
        assert!(matches!(text("code", b"7412"), Ok(value) if value == "7412"));
        assert!(text("code", &[0xff, 0]).is_err());
    }

    #[test]
    fn converts_account_id_to_steam_id_64() {
        assert_eq!(steam_id(1_025_200_509), Some(76_561_198_985_466_237));
        assert_eq!(steam_id(0), None);
    }

    #[test]
    fn accepts_binary_registry_flags_only() {
        assert!(matches!(flag("key", "flag", 0), Ok(false)));
        assert!(matches!(flag("key", "flag", 1), Ok(true)));
        assert!(flag("key", "flag", 2).is_err());
    }
}
