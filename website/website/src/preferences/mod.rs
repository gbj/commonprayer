use std::collections::HashMap;

use episcopal_api::{
    language::Language,
    liturgy::{GlobalPref, PreferenceKey, PreferenceValue, Version},
};
use leptos::{is_server, log, window};
use serde::Serialize;

use crate::table_of_contents::TOCLiturgy;

pub enum StorageError {
    StorageNotAvailable,
    SettingStorage,
    SerializeKey,
    SerializeValue,
}

fn set_localstorage(key: &str, value: impl Serialize) -> Result<(), StorageError> {
    let value = serde_json::to_string(&value).map_err(|err| {
        log(&format!(
            "(set_localstorage) error for key {}\n\n{}",
            key, err
        ));
        StorageError::SerializeValue
    })?;

    set_raw(key, &value)
}

pub fn set_raw(key: &str, value: &str) -> Result<(), StorageError> {
    if is_server!() {
        Err(StorageError::StorageNotAvailable)
    } else {
        let storage = window()
            .local_storage()
            .map_err(|_| StorageError::StorageNotAvailable)?
            .ok_or(StorageError::StorageNotAvailable)?;

        storage
            .set_item(key, value)
            .map_err(|_| StorageError::SettingStorage)
    }
}

pub fn get_raw(key: &str) -> Option<String> {
    if is_server!() {
        None
    } else {
        let storage = window().local_storage().ok()??;
        storage.get_item(key).ok().flatten()
    }
}

pub fn clear_raw(key: &str) -> Result<(), StorageError> {
    if is_server!() {
        Err(StorageError::StorageNotAvailable)
    } else {
        let storage = window()
            .local_storage()
            .map_err(|_| StorageError::StorageNotAvailable)?
            .ok_or(StorageError::StorageNotAvailable)?;
        storage
            .delete(key)
            .map_err(|_| StorageError::SettingStorage)
    }
}

pub fn get(key: &PreferenceKey) -> Option<PreferenceValue> {
    let key = serde_json::to_string(&key).ok()?;
    get_raw(&key).and_then(|value| serde_json::from_str(&value).ok())
}

pub fn set(key: &PreferenceKey, value: &PreferenceValue) -> Result<(), StorageError> {
    let key = serde_json::to_string(&key).map_err(|_| StorageError::SerializeKey)?;
    set_localstorage(&key, value)
}

pub fn clear(key: &PreferenceKey) -> Result<(), StorageError> {
    let key = serde_json::to_string(&key).map_err(|_| StorageError::SerializeKey)?;
    clear_raw(&key)
}

fn liturgy_key(liturgy: TOCLiturgy, language: Language, version: Version) -> String {
    format!("{}-{:#?}-{:#?}", liturgy, language, version)
}

pub fn set_prefs_for_liturgy(
    liturgy: TOCLiturgy,
    language: Language,
    version: Version,
    prefs: HashMap<PreferenceKey, PreferenceValue>,
) -> Result<(), StorageError> {
    // serde_json can't handle HashMaps with non-String keys
    let vectorized = prefs.into_iter().collect::<Vec<_>>();
    set_localstorage(&liturgy_key(liturgy, language, version), vectorized)
}

pub fn get_prefs_for_liturgy(
    liturgy: TOCLiturgy,
    language: Language,
    version: Version,
) -> HashMap<PreferenceKey, PreferenceValue> {
    let mut prefs: HashMap<PreferenceKey, PreferenceValue> = window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| {
            storage
                .get_item(&liturgy_key(liturgy, language, version))
                .ok()
                .flatten()
                // serde_json can't handle HashMaps with non-String keys
                .and_then(|value| {
                    serde_json::from_str::<Vec<(PreferenceKey, PreferenceValue)>>(&value).ok()
                })
                .map(|vectorized| vectorized.into_iter().collect())
        })
        .unwrap_or_default();

    // Overwrite particular global prefs with stored prefs:
    insert_global_pref(&mut prefs, GlobalPref::BibleVersion);
    insert_global_pref(&mut prefs, GlobalPref::PsalmCycle);
    insert_global_pref(&mut prefs, GlobalPref::UseBlackLetterCollects);
    insert_global_pref(&mut prefs, GlobalPref::GloriaPatriTraditional);

    prefs
}

fn insert_global_pref(prefs: &mut HashMap<PreferenceKey, PreferenceValue>, pref: GlobalPref) {
    if let Some(value) = get(&PreferenceKey::from(pref)) {
        prefs.insert(PreferenceKey::from(pref), value);
    }
}

pub fn is(key: &PreferenceKey, value: &PreferenceValue) -> bool {
    if is_server!() {
        false
    } else {
        get(key).as_ref() == Some(value)
    }
}

pub fn is_with_default(key: &PreferenceKey, value: &PreferenceValue, default: bool) -> bool {
    if is_server!() {
        default
    } else if let Some(stored_value) = get(key).as_ref() {
        stored_value == value
    } else {
        default
    }
}
