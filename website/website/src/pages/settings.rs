use std::{collections::HashMap, time::Duration};

use crate::{
    components::{header, SegmentButton, Toggle},
    preferences::{self, StorageError},
    table_of_contents::TOCLiturgy,
};
use episcopal_api::{
    language::Language,
    library::rite2::{COMPLINE, MORNING_PRAYER_II, NOONDAY_PRAYER},
    liturgy::{
        Content, Document, GlobalPref, Lectionaries, LiturgyPreferences, PreferenceKey,
        PreferenceValue, Version,
    },
};
use futures::StreamExt;
use itertools::Itertools;
use leptos::*;
use rust_i18n::t;
use serde::{Deserialize, Serialize};

pub fn settings() -> Page<SettingsPageProps, ()> {
    Page::new("settings")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(static_props)
}

fn static_props(_locale: &str, _path: &str, _params: ()) -> Option<SettingsPageProps> {
    Some(SettingsPageProps {
        mp_1_prefs: liturgy_to_preferences(&MORNING_PRAYER_II),
        mp_2_prefs: liturgy_to_preferences(&MORNING_PRAYER_II),
        np_prefs: liturgy_to_preferences(&NOONDAY_PRAYER),
        ep_1_prefs: None, // TODO
        ep_2_prefs: None, // TODO
        cp_prefs: liturgy_to_preferences(&COMPLINE),
        eucharist_prefs: None, // TODO
    })
}

fn liturgy_to_preferences(document: &Document) -> Option<(String, LiturgyPreferences)> {
    if let Content::Liturgy(liturgy) = &document.content {
        Some((
            document.label.clone().unwrap_or_default(),
            liturgy.preferences.clone(),
        ))
    } else {
        None
    }
}

fn head(_locale: &str, _props: &SettingsPageProps) -> View {
    view! {
        <>
            <title>{t!("settings.title")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/settings.css"/>
        </>
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SettingsPageProps {
    mp_1_prefs: Option<(String, LiturgyPreferences)>,
    mp_2_prefs: Option<(String, LiturgyPreferences)>,
    np_prefs: Option<(String, LiturgyPreferences)>,
    ep_1_prefs: Option<(String, LiturgyPreferences)>,
    ep_2_prefs: Option<(String, LiturgyPreferences)>,
    cp_prefs: Option<(String, LiturgyPreferences)>,
    eucharist_prefs: Option<(String, LiturgyPreferences)>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Status {
    Idle,
    Success,
    Error,
    NotAvailable,
}

fn body(locale: &str, props: &SettingsPageProps) -> View {
    let status = Behavior::new(Status::Idle);

    let dark_mode_setting = SegmentButton::new_with_default_value(
        "dark_mode",
        Some(t!("settings.dark_mode.label")),
        [
            (None, t!("settings.dark_mode.auto"), Some(t!("settings.dark_mode.auto_explanation"))),
            (Some("light".to_string()), t!("settings.dark_mode.light"), Some(t!("settings.dark_mode.light_explanation"))),
            (Some("dark".to_string()), t!("settings.dark_mode.dark"), Some(t!("settings.dark_mode.dark_explanation")))
        ],
        preferences::get_raw(DARK_MODE_KEY)
    );
    dark_mode_setting.value.stream().skip(1).create_effect({
        let status = status.clone();
        
        move |new_value| {
            let body_class_list = leptos::body().unwrap().class_list();
            match new_value {
                Some(value) => {
                    preference_effect(&status, || {
                        preferences::set_raw(DARK_MODE_KEY, &value)?;
                        body_class_list.remove_1("dark-mode-light").map_err(|_| StorageError::SettingStorage)?;
                        body_class_list.remove_1("dark-mode-dark").map_err(|_| StorageError::SettingStorage)?;
                        body_class_list.add_1(&format!("dark-mode-{}", value)).map_err(|_| StorageError::SettingStorage)?;
                        Ok(())
                    });
                },
                None => {
                    preference_effect(&status, || {
                        preferences::clear_raw(DARK_MODE_KEY)?;
                        body_class_list.remove_1("dark-mode-light").map_err(|_| StorageError::SettingStorage)?;
                        body_class_list.remove_1("dark-mode-dark").map_err(|_| StorageError::SettingStorage)?;
                        Ok(())
                    });
                },
            }
        }
    });

    let version_setting = setting_toggle(
        &status,
        "version",
        PreferenceKey::from(GlobalPref::Version),
        t!("settings.version"),
        (t!("settings.rite_i"), PreferenceValue::from(Version::RiteI)),
        (
            t!("settings.rite_ii"),
            PreferenceValue::from(Version::RiteII),
        ),
        true,
    );

    let calendar_setting = setting_toggle(
        &status,
        "calendar",
        PreferenceKey::from(GlobalPref::Calendar),
        t!("settings.calendar"),
        (t!("bcp_1979"), PreferenceValue::from("bcp1979")),
        (t!("lff_2018"), PreferenceValue::from("lff2018")),
        false,
    );

    let psalm_cycle_setting = setting_toggle(
        &status,
        "psalm_cycle",
        PreferenceKey::from(GlobalPref::PsalmCycle),
        t!("settings.psalm_cycle"),
        (
            t!("daily_readings.daily_office_psalms"),
            PreferenceValue::from(Lectionaries::BCP1979DailyOfficePsalms),
        ),
        (
            t!("daily_readings.thirty_day_psalms"),
            PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms),
        ),
        false,
    );

    let black_letter_collect_setting = setting_toggle(
        &status,
        "ublc",
        PreferenceKey::from(GlobalPref::UseBlackLetterCollects),
        t!("settings.use_black_letter_collects"),
        (t!("settings.no"), PreferenceValue::Bool(false)),
        (t!("settings.yes"), PreferenceValue::Bool(true)),
        true,
    );

    let gloria_patri_setting = setting_toggle(
        &status,
        "gloria-patri",
        PreferenceKey::from(GlobalPref::GloriaPatriTraditional),
        t!("settings.gloria_patri"),
        (t!("settings.gloria_patri_2"), PreferenceValue::Bool(false)),
        (t!("settings.gloria_patri_1"), PreferenceValue::Bool(true)),
        false,
    );


    let bible_version_setting = SegmentButton::new_with_default_value(
        "bible_version",
        Some(t!("settings.bible_version")),
        [
            (None, "—".into(), Some(t!("settings.no_bible_version"))),
            (
                Some(Version::NRSV),
                t!("bible_version.NRSV"),
                Some(t!("bible_version.NRSV_full")),
            ),
            (
                Some(Version::ESV),
                t!("bible_version.ESV"),
                Some(t!("bible_version.ESV_full")),
            ),
            (
                Some(Version::CEB),
                t!("bible_version.CEB"),
                Some(t!("bible_version.CEB_full")),
            ),
            (
                Some(Version::KJV),
                t!("bible_version.KJV"),
                Some(t!("bible_version.KJV_full")),
            ),
        ],
        
            preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion)).and_then(|value| {
                match value {
                    PreferenceValue::Version(version) => Some(version),
                    _ => None,
                }
            })
    );
    bible_version_setting.value.stream().skip(1).create_effect({
        let status = status.clone();
        move |new_value| match new_value {
            Some(value) => set_preference(
                &status,
                &PreferenceKey::from(GlobalPref::BibleVersion),
                &PreferenceValue::from(value),
            ),
            None => clear_preference(&status, &PreferenceKey::from(GlobalPref::BibleVersion)),
        }
    });

    let liturgy_picker = SegmentButton::new_with_default_value(
        "liturgy",
        None,
        [
            (TOCLiturgy::MP, t!("toc.morning_prayer"), None),
            (TOCLiturgy::NP, t!("toc.noonday_prayer"), None),
            (TOCLiturgy::EP, t!("toc.evening_prayer"), None),
            (TOCLiturgy::Compline, t!("toc.compline"), None),
            (TOCLiturgy::Eucharist, t!("toc.holy_eucharist"), None),
        ],
        TOCLiturgy::MP,
    );

    // hold combined liturgy/version in a new behavior
    let liturgy_and_version =
        Behavior::new((liturgy_picker.value.get(), version_setting.toggled.get()));
    (
        liturgy_picker.value.stream(),
        version_setting.toggled.stream(),
    )
        .lift()
        .skip(2) // skip initial value for each, which have already been put into the Behavior
        .create_effect({
            let liturgy_and_version = liturgy_and_version.clone();
            move |value| {
                if let (Some(liturgy), Some(contemporary)) = value {
                    liturgy_and_version.set((liturgy, contemporary));
                }
            }
        });

    view! {
        <>
            {header(locale, &t!("settings.title"))}
            <main>
                <h2>{t!("settings.general")}</h2>
                <dyn:view view={dark_mode_setting.view()} />
                <dyn:view view={version_setting.view()} />
                <dyn:view view={calendar_setting.view()} />
                <dyn:view view={psalm_cycle_setting.view()} />
                <dyn:view view={bible_version_setting.view()} />

                <h2>{t!("settings.liturgy")}</h2>
                <dyn:view view={liturgy_picker.view()} />
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != TOCLiturgy::MP || contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, TOCLiturgy::MP, Language::En, Version::RiteI, &props.mp_1_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != TOCLiturgy::MP || !contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, TOCLiturgy::MP, Language::En, Version::RiteII, &props.mp_2_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, _)| liturgy != TOCLiturgy::NP).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, TOCLiturgy::NP, Language::En, Version::BCP1979, &props.np_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != TOCLiturgy::EP || contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, TOCLiturgy::EP, Language::En, Version::RiteI, &props.ep_1_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != TOCLiturgy::EP || !contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, TOCLiturgy::EP, Language::En, Version::RiteII, &props.ep_2_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, _)| liturgy != TOCLiturgy::Compline).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, TOCLiturgy::Compline, Language::En, Version::BCP1979, &props.cp_prefs)} />
                </dyn:section>

                <h2>{t!("settings.advanced")}</h2>
                <dyn:view view={gloria_patri_setting.view()} />
                <dyn:view view={black_letter_collect_setting.view()} />
            </main>
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
        </>
    }
}

fn liturgy_preferences_view(
    status: &Behavior<Status>,
    liturgy: TOCLiturgy,
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
                                        move |ev: Event| {
                                            let value = event_target_value(ev);
                                            log(&format!("on_change called for {:#?}", pref.key));
                                            if let Ok(selected_idx) = value.parse::<usize>() {
                                                if let Some(option) =
                                                    pref.choices().nth(selected_idx)
                                                {
                                                    let mut current_prefs = preferences::get_prefs_for_liturgy(liturgy, language, version);
                                                    current_prefs.insert(pref.key.clone(), option.value.clone());
                                                    preference_effect(&status, move || 
                                                        preferences::set_prefs_for_liturgy(liturgy, language, version, current_prefs)
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

fn setting_toggle(
    status: &Behavior<Status>,
    name: &'static str,
    key: PreferenceKey,
    legend: String,
    off: (String, PreferenceValue),
    on: (String, PreferenceValue),
    toggled_by_default: bool,
) -> Toggle {
    let (off_label, off_value) = off;
    let (on_label, on_value) = on;
    let toggle = Toggle::new(
        preferences::is_with_default(&key, &on_value, toggled_by_default),
        name,
        off_label,
        on_label,
        Some(legend),
    );

    // On initial load from localStorage or default, don't display message/set preference redundantly
    toggle.toggled.stream().skip(1).create_effect({
        let status = status.clone();
        move |toggled| {
            if toggled {
                set_preference(&status, &key, &on_value);
            } else {
                set_preference(&status, &key, &off_value);
            }
        }
    });

    toggle
}

fn set_preference(status: &Behavior<Status>, key: &PreferenceKey, value: &PreferenceValue) {
    preference_effect(status, || preferences::set(key, value));
}

fn clear_preference(status: &Behavior<Status>, key: &PreferenceKey) {
    preference_effect(status, || preferences::clear(key));
}

fn preference_effect(status: &Behavior<Status>, effect: impl FnOnce() -> Result<(), StorageError>) {
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
