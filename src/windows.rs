use windows_registry::{CURRENT_USER, Type, Value};

use crate::{Data, Error, matches, text};

const PATH: &str = r"Software\Facepunch Studios LTD\Rust";
const HISTORY: &str = "CONSOLE_HISTORY";
const LAST_CODE: &str = "lastCodeEntered";

pub(crate) fn scan() -> Result<Data, Error> {
    let key = CURRENT_USER
        .open(PATH)
        .map_err(|error| Error::Open(error.to_string()))?;
    let values = key
        .values()
        .map_err(|error| Error::Read(error.to_string()))?;

    let mut history = None;
    let mut last_code = None;
    for (name, value) in values {
        if matches(&name, HISTORY) {
            insert(&mut history, HISTORY, name, value)?;
        } else if matches(&name, LAST_CODE) {
            insert(&mut last_code, LAST_CODE, name, value)?;
        }
    }

    let (history_name, history) = history.ok_or(Error::Missing(HISTORY))?;
    let (code_name, code) = last_code.ok_or(Error::Missing(LAST_CODE))?;
    let history = text(&history_name, &history)?;
    let last_code = text(&code_name, &code)?;

    Ok(Data {
        history: history
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect(),
        last_code,
    })
}

fn insert(
    slot: &mut Option<(String, Vec<u8>)>,
    key: &'static str,
    name: String,
    value: Value,
) -> Result<(), Error> {
    if slot.is_some() {
        return Err(Error::Duplicate(key));
    }
    if value.ty() != Type::Bytes {
        return Err(Error::WrongType { name });
    }

    tracing::debug!(%name, bytes = value.len(), "found registry value");
    *slot = Some((name, value.to_vec()));
    Ok(())
}
