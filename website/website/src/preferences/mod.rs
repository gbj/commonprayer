use std::collections::HashMap;

use episcopal_api::liturgy::{GlobalPref, PreferenceKey, PreferenceValue, Version};
use leptos::{is_server, window};
use serde::Serialize;

pub enum StorageError {
    StorageNotAvailable,
    SettingStorage,
    SerializeKey,
    SerializeValue,
}

fn set_localstorage(key: &str, value: impl Serialize) -> Result<(), StorageError> {
    let storage = window()
        .local_storage()
        .map_err(|_| StorageError::StorageNotAvailable)?
        .ok_or(StorageError::StorageNotAvailable)?;

    let value = serde_json::to_string(&value).map_err(|_| StorageError::SerializeValue)?;

    storage
        .set_item(key, &value)
        .map_err(|_| StorageError::SettingStorage)
}

pub fn get(key: &PreferenceKey) -> Option<PreferenceValue> {
    let key = serde_json::to_string(&key).ok()?;
    let storage = window().local_storage().ok()??;
    storage
        .get_item(&key)
        .ok()
        .flatten()
        .and_then(|value| serde_json::from_str(&value).ok())
}

pub fn set(key: &PreferenceKey, value: &PreferenceValue) -> Result<(), StorageError> {
    let key = serde_json::to_string(&key).map_err(|_| StorageError::SerializeKey)?;
    set_localstorage(&key, value)
}

pub fn set_prefs_for_office(
    office: &str,
    prefs: HashMap<PreferenceKey, PreferenceValue>,
) -> Result<(), StorageError> {
    set_localstorage(office, prefs)
}

pub fn get_prefs_for_office(
    office: &str,
    version: Version,
) -> HashMap<PreferenceKey, PreferenceValue> {
    let mut prefs: HashMap<PreferenceKey, PreferenceValue> = window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| {
            storage
                .get_item(&format!("{}-{:#?}", office, version))
                .ok()
                .flatten()
                .and_then(|value| serde_json::from_str(&value).ok())
        })
        .unwrap_or_default();

    // Overwrite particular global prefs with stored prefs:
    // - UseBlackLetterCollects
    // - PsalmCycle
    if let Some(value) = get(&PreferenceKey::from(GlobalPref::PsalmCycle)) {
        prefs.insert(PreferenceKey::from(GlobalPref::PsalmCycle), value);
    }
    if let Some(value) = get(&PreferenceKey::from(GlobalPref::UseBlackLetterCollects)) {
        prefs.insert(
            PreferenceKey::from(GlobalPref::UseBlackLetterCollects),
            value,
        );
    }

    prefs
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
