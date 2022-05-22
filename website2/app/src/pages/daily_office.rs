use language::Language;
use leptos2::*;
use liturgy::{GlobalPref, PreferenceKey, PreferenceValue, Version};
use wasm_bindgen::UnwrapThrowExt;

use crate::{
    preferences,
    utils::time::{current_preferred_liturgy, today, DEFAULT_OFFICE_TIMES},
};

pub fn daily_office() -> Page<(), ()> {
    Page::new("daily-office")
        .head_fn(head)
        .body_fn(body)
        .state(|_, _, _| Some(()))
        .build_paths_fn(build_paths_fn)
        .incremental_generation()
}

fn build_paths_fn() -> Vec<String> {
    vec!["".into()]
}

fn head(_locale: &str, _props: &()) -> Vec<Node> {
    let title = format!("{} – {}", t!("toc.daily_office"), t!("common_prayer"));

    view! {
        <>
            <title>{title}</title>
        </>
    }
}

fn body(_locale: &str, _props: &()) -> Vec<Node> {
    vec![]
}

pub fn daily_office_redirect() {
    // Daily Office page exists simply to redirect based on client date/time and preferences
    if !is_server!() {
        let date = today();

        let locale = location_pathname()
            .and_then(|path| {
                let mut parts = path.split('/');
                parts.next();
                parts.next().map(String::from)
            })
            .unwrap_or_else(|| "en".to_string());

        // TODO load time ranges from preferences
        let office = current_preferred_liturgy(&DEFAULT_OFFICE_TIMES);

        let language = preferences::get(&PreferenceKey::Global(GlobalPref::Language))
            .and_then(|value| match value {
                PreferenceValue::Language(version) => Some(version),
                _ => None,
            })
            .unwrap_or(Language::En);

        let version = preferences::get(&PreferenceKey::Global(GlobalPref::Version))
            .and_then(|value| match value {
                PreferenceValue::Version(version) => Some(version),
                _ => None,
            })
            .unwrap_or(Version::RiteII);

        let calendar = preferences::get(&PreferenceKey::Global(GlobalPref::Calendar))
            .and_then(|value| match value {
                PreferenceValue::Local(calendar_slug) => Some(calendar_slug),
                _ => None,
            })
            .unwrap_or_else(|| "bcp1979".to_string());

        let prefs = preferences::get_prefs_for_liturgy(&office, language, version);

        // convert HashMap<K, V> to Vec<(K, V)> because serde_json can't serialize a HashMap with enum keys to a JSON map
        let serialized_prefs =
            serde_json::to_string(&prefs.iter().collect::<Vec<_>>()).unwrap_or_default();

        let url = format!(
            "/{}/document/{}/{:#?}/{}/{}/{}",
            locale, office, version, date, calendar, serialized_prefs,
        );
        location().set_href(&url).unwrap_throw();
    }
}
