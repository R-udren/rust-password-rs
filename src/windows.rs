use windows_registry::{CURRENT_USER, Type, Value};

use windows_registry::Key;

use crate::{Data, Error, Game, Steam, flag, matches, steam_id, text};

const RUST_PATH: &str = r"Software\Facepunch Studios LTD\Rust";
const STEAM_PATH: &str = r"Software\Valve\Steam";
const ACTIVE_PROCESS_PATH: &str = r"Software\Valve\Steam\ActiveProcess";
const RUST_APP_PATH: &str = r"Software\Valve\Steam\Apps\252490";
const HISTORY: &str = "CONSOLE_HISTORY";
const LAST_CODE: &str = "lastCodeEntered";

pub fn scan() -> Result<Data, Error> {
    let key = CURRENT_USER
        .open(RUST_PATH)
        .map_err(|error| Error::Open(error.to_string()))?;
    let values = key
        .values()
        .map_err(|error| Error::Read(error.to_string()))?;

    let mut history = None;
    let mut last_code = None;
    for (name, value) in values {
        if matches(&name, HISTORY) {
            insert(&mut history, HISTORY, name, &value)?;
        } else if matches(&name, LAST_CODE) {
            insert(&mut last_code, LAST_CODE, name, &value)?;
        }
    }

    let (history_name, history) = history.ok_or(Error::Missing(HISTORY))?;
    let (code_name, code) = last_code.ok_or(Error::Missing(LAST_CODE))?;
    let history = text(&history_name, &history)?;
    let last_code = text(&code_name, &code)?;
    let steam = read_steam()?;
    let rust = read_rust_app()?;

    Ok(Data {
        history: history
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect(),
        last_code,
        steam,
        rust,
    })
}

fn read_steam() -> Result<Steam, Error> {
    let steam = open(STEAM_PATH)?;
    let active_process = open(ACTIVE_PROCESS_PATH)?;

    Ok(Steam {
        last_game: string(&steam, STEAM_PATH, "LastGameNameUsed")?,
        steam_id: steam_id(u32(&active_process, ACTIVE_PROCESS_PATH, "ActiveUser")?),
    })
}

fn read_rust_app() -> Result<Game, Error> {
    let key = open(RUST_APP_PATH)?;
    Ok(Game {
        name: string(&key, RUST_APP_PATH, "Name")?,
        installed: flag(
            RUST_APP_PATH,
            "Installed",
            u32(&key, RUST_APP_PATH, "Installed")?,
        )?,
        running: flag(
            RUST_APP_PATH,
            "Running",
            u32(&key, RUST_APP_PATH, "Running")?,
        )?,
    })
}

fn open(path: &'static str) -> Result<Key, Error> {
    CURRENT_USER.open(path).map_err(|error| Error::OpenKey {
        path,
        reason: error.to_string(),
    })
}

fn string(key: &Key, path: &'static str, name: &'static str) -> Result<String, Error> {
    key.get_string(name).map_err(|error| Error::ReadValue {
        path,
        name,
        reason: error.to_string(),
    })
}

fn u32(key: &Key, path: &'static str, name: &'static str) -> Result<u32, Error> {
    key.get_u32(name).map_err(|error| Error::ReadValue {
        path,
        name,
        reason: error.to_string(),
    })
}

fn insert(
    slot: &mut Option<(String, Vec<u8>)>,
    key: &'static str,
    name: String,
    value: &Value,
) -> Result<(), Error> {
    if slot.is_some() {
        return Err(Error::Duplicate(key));
    }
    if value.ty() != Type::Bytes {
        return Err(Error::WrongType { name });
    }

    #[cfg(debug_assertions)]
    eprintln!("found registry value: {name} ({} bytes)", value.len());
    *slot = Some((name, value.to_vec()));
    Ok(())
}
