use episcopal_api::{
    language::Language,
    liturgy::{GlobalPref, PreferenceKey, PreferenceValue, Version},
};
use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

use crate::{
    preferences,
    utils::time::{current_preferred_liturgy, today, DEFAULT_OFFICE_TIMES},
};

pub fn daily_office() -> Page<(), ()> {
    Page::new("daily-office")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(|_, _, _| ())
        .build_paths_fn(build_paths_fn)
}

fn build_paths_fn() -> Vec<String> {
    vec!["".into()]
}

fn head(_locale: &str, _props: &()) -> View {
    let title = format!("{} – {}", t!("toc.daily_office"), t!("common_prayer"));

    view! {
        <>
            <title>{title}</title>
        </>
    }
}

fn body(locale: &str, _props: &()) -> View {
    // Daily Office page exists simply to redirect based on client date/time and preferences
    if !is_server!() {
        let date = today();

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

        let prefs = preferences::get_prefs_for_liturgy(office, language, version);

        // convert HashMap<K, V> to Vec<(K, V)> because serde_json can't serialize a HashMap with enum keys to a JSON map
        let serialized_prefs =
            serde_json::to_string(&prefs.iter().collect::<Vec<_>>()).unwrap_or_default();

        let url = format!(
            "/{}/document/office/{}/{:#?}/{}/{}/{}",
            locale, office, version, date, calendar, serialized_prefs,
        );
        location().set_href(&url).unwrap_throw();
    }

    // give an empty view until redirect
    View::Empty
}
