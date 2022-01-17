use episcopal_api::liturgy::{GlobalPref, PreferenceKey, PreferenceValue, Version};
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

        let version = preferences::get(PreferenceKey::Global(GlobalPref::Version))
            .and_then(|value| match value {
                PreferenceValue::Version(version) => Some(version),
                _ => None,
            })
            .unwrap_or(Version::RiteII);

        let calendar = preferences::get(PreferenceKey::Global(GlobalPref::Calendar))
            .and_then(|value| match value {
                PreferenceValue::Local(calendar_slug) => Some(calendar_slug),
                _ => None,
            })
            .unwrap_or_else(|| "bcp1979".to_string());

        // TODO load prefs for liturgy from stored preferences
        let prefs = preferences::get_prefs_for_office(office);

        let url = format!(
            "/{}/document/office/{}/{:#?}/{}/{}/{}",
            locale,
            office,
            version,
            date,
            calendar,
            serde_json::to_string(&prefs).unwrap_or_else(|_| "{}".to_string()),
        );
        location().set_href(&url).unwrap_throw();
    }

    // give an empty view until redirect
    View::Empty
}
