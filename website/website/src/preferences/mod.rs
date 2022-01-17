use std::collections::HashMap;

use episcopal_api::liturgy::{PreferenceKey, PreferenceValue};

pub mod keys;

pub fn get(key: PreferenceKey) -> Option<PreferenceValue> {
    None
}

pub fn set(key: PreferenceKey, value: PreferenceValue) {
    todo!()
}

pub fn get_prefs_for_office(office: &str) -> HashMap<PreferenceKey, PreferenceValue> {
    HashMap::new()
}
