use std::collections::HashMap;

use leptos::*;
use liturgy::{
    Lectionaries, LiturgyPreferences, PreferenceKey, PreferenceValue, Slug, SlugPath, Version,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct GeneralSettings {
    pub liturgy_version: Version,
    pub use_lff: bool,
    pub psalm_cycle: Lectionaries,
    pub bible_version: Version,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            liturgy_version: Version::RiteII,
            use_lff: true,
            psalm_cycle: Lectionaries::BCP1979DailyOfficePsalms,
            bible_version: Version::NRSV,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneralSettingsContext(ReadSignal<GeneralSettings>, WriteSignal<GeneralSettings>);

pub fn use_settings(cx: Scope) -> (ReadSignal<GeneralSettings>, WriteSignal<GeneralSettings>) {
    match use_context::<GeneralSettingsContext>(cx) {
        Some(settings) => {
            let GeneralSettingsContext(read, write) = settings;
            (read, write)
        }
        None => {
            log::warn!("GeneralSettingsContext not provided");
            create_signal(cx, Default::default())
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct SettingsForLiturgy {
    pub liturgy: SlugPath,
    pub liturgy_prefs: LiturgyPreferences,
    pub client_prefs: HashMap<PreferenceKey, PreferenceValue>,
}

impl SettingsForLiturgy {
    pub fn serialize_non_default_prefs(&self) -> String {
        let filtered_as_vec = self
            .client_prefs
            .iter()
            .filter(|(k, v)| self.liturgy_prefs.default_value_for_key(k) != Some(v))
            .collect::<Vec<_>>();
        serde_json::to_string(&filtered_as_vec).unwrap()
    }
}

pub fn use_settings_for_liturgy(cx: Scope, slug: Slug) -> SettingsForLiturgy {
    log::warn!("TODO: implement use_settings_for_liturgy");
    Default::default()
}
