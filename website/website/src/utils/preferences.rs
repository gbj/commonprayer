use std::{time::Duration, collections::HashMap};

use {liturgy::{PreferenceKey, PreferenceValue, Version, LiturgyPreferences, Content, Document}, language::Language};
use futures::StreamExt;
use leptos::*;
use itertools::Itertools;
use liturgy::SlugPath;

use crate::{preferences::{self, StorageError}};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Status {
    Idle,
    Success,
    Error,
    NotAvailable,
}

pub fn preference_status_footer(status: &Behavior<Status>) -> View {
    view! {
        <footer>
            <dyn:p class="success hidden" class:hidden={status.stream().map(|status| status != Status::Success).boxed_local()}>
                {t!("settings.success")}
            </dyn:p>
            <dyn:p class="error hidden" class:hidden={status.stream().map(|status| status != Status::Error).boxed_local()}>
                {t!("settings.error")}
            </dyn:p>
            <dyn:p class="error hidden" class:hidden={status.stream().map(|status| status != Status::NotAvailable).boxed_local()}>
                {t!("settings.not_available")}
            </dyn:p>
        </footer>
    }
}

pub fn set_preference(status: &Behavior<Status>, key: &PreferenceKey, value: &PreferenceValue) {
    preference_effect(status, || preferences::set(key, value));
}

pub fn clear_preference(status: &Behavior<Status>, key: &PreferenceKey) {
    preference_effect(status, || preferences::clear(key));
}

pub fn preference_effect(
    status: &Behavior<Status>,
    effect: impl FnOnce() -> Result<(), StorageError>,
) {
    if !is_server!() {
        let (new_status, delay_before_clearing) = match (effect)() {
            Ok(_) => (Status::Success, Duration::from_secs(3)),
            Err(StorageError::StorageNotAvailable) => {
                (Status::NotAvailable, Duration::from_secs(8))
            }
            Err(_) => (Status::Error, Duration::from_secs(8)),
        };
        status.set(new_status);
        set_timeout(
            {
                let status = status.clone();
                move || status.set(Status::Idle)
            },
            delay_before_clearing,
        );
    }
}

pub fn liturgy_to_preferences(document: &Document) -> Option<(String, LiturgyPreferences)> {
    if let Content::Liturgy(liturgy) = &document.content {
        Some((
            document.label.clone().unwrap_or_default(),
            liturgy.preferences.clone(),
        ))
    } else {
        None
    }
}

pub fn liturgy_preferences_view(
    status: &Behavior<Status>,
    liturgy: &SlugPath,
    language: Language,
    version: Version,
    prefs: &Option<(String, LiturgyPreferences)>,
) -> View {
    let client_prefs = if is_server!() {
        HashMap::new()
    } else {
        preferences::get_prefs_for_liturgy(liturgy, language, version)
    };

    if let Some((label, prefs)) = prefs.as_ref() {
        let categories = prefs
                .iter()
                .group_by(|pref| pref.category.as_ref())
                .into_iter()
                .map(|(label, group)| {
                    let prefs = View::Fragment(
                        group
                            .filter_map(|pref| {
                                if pref.only_one_choice() {
                                    None
                                } else {
                                    let choices = View::Fragment(
                                        pref.choices()
                                            .enumerate()
                                            .map(|(choice_idx, choice)| {
                                                view! {
                                                    <option value={choice_idx.to_string()}>{&choice.label}</option>
                                                }
                                            })
                                            .collect()
                                    );

                                    let on_change = {
                                        let pref = pref.clone();
                                        let status = status.clone();
                                        let liturgy = liturgy.clone();
                                        move |ev: Event| {
                                            let value = event_target_value(ev);
                                            log(&format!("on_change called for {:#?}", pref.key));
                                            if let Ok(selected_idx) = value.parse::<usize>() {
                                                if let Some(option) =
                                                    pref.choices().nth(selected_idx)
                                                {
                                                    let mut current_prefs = preferences::get_prefs_for_liturgy(&liturgy, language, version);
                                                    current_prefs.insert(pref.key.clone(), option.value.clone());
                                                    preference_effect(&status, {
                                                        let liturgy = liturgy.clone();
                                                        move || 
                                                            preferences::set_prefs_for_liturgy(liturgy, language, version, current_prefs)
                                                        }
                                                    );
                                                }
                                            }
                                        }
                                    };

                                    // load the initial value from client preferences set in localStorage
                                    // because this is not known on the server side, needs to be a Behavior => Stream, not a static value
                                    let initial_value = Behavior::new(0);
                                    if let Some(idx) = client_prefs.get(&pref.key).and_then(|value| pref.choices().enumerate().find(|(_, choice)| &choice.value == value).map(|(idx, _)| idx)) {
                                        // if the selected pref is idx 0, we don't need to do anything anyway
                                        if idx != 0 {
                                            initial_value.set(idx);
                                        }
                                    }

                                    Some(view! {
                                        <label class="preference">
                                            {&pref.label}
                                            <dyn:select
                                                prop:value={initial_value.stream().map(|n| Some(n.to_string())).boxed_local()}
                                                on:change=on_change
                                            >
                                                {choices}
                                            </dyn:select>
                                        </label>
                                    })
                                }
                            })
                            .collect(),
                    );

                    let label = if let Some(label) = label {
                        view! { <h4>{label}</h4> }
                    } else {
                        View::Empty
                    };

                    view! {
                        <>
                            {label}
                            {prefs}
                        </>
                    }
                })
                .collect::<Vec<_>>();

        if categories.is_empty() {
            View::Empty
        } else {
            view! {
                <>
                    <h3>{label}</h3>
                    {View::Fragment(categories)}
                </>
            }
        }
    } else {
        View::Empty
    }
}