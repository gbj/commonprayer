use std::collections::HashMap;

use episcopal_api::liturgy::{PreferenceKey, PreferenceValue};
use futures::TryFutureExt;
use leptos::window;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

pub enum StorageError {
    StorageNotAvailable,
    SettingStorage,
    SerializeKey,
    SerializeValue,
}

pub fn get(key: PreferenceKey) -> Option<PreferenceValue> {
    let key = serde_json::to_string(&key).ok()?;
    let storage = window().local_storage().ok()??;
    storage
        .get_item(&key)
        .ok()
        .flatten()
        .and_then(|value| serde_json::from_str(&value).ok())
}

pub fn set(key: PreferenceKey, value: PreferenceValue) -> Result<(), StorageError> {
    let key = serde_json::to_string(&key).map_err(|_| StorageError::SerializeKey)?;
    let storage = window()
        .local_storage()
        .map_err(|_| StorageError::StorageNotAvailable)?
        .ok_or(StorageError::StorageNotAvailable)?;
    let value = serde_json::to_string(&value).map_err(|_| StorageError::SerializeValue)?;
    storage
        .set_item(&key, &value)
        .map_err(|_| StorageError::SettingStorage)
}

pub fn get_prefs_for_office(office: &str) -> HashMap<PreferenceKey, PreferenceValue> {
    HashMap::new()
}
