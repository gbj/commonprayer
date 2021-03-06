use crate::{
    components::*,
    preferences::{self, StorageError},
    utils::preferences::*,
};
use futures::StreamExt;
use language::Language;
use leptos::*;
use library::bcp1979::{COMPLINE, NOONDAY_PRAYER};
use library::rite2::{EVENING_PRAYER_II, MORNING_PRAYER_II};
use liturgy::{
    GlobalPref, Lectionaries, LiturgyPreferences, PreferenceKey, PreferenceValue, Slug, SlugPath,
    Version,
};
use rust_i18n::t;
use serde::{Deserialize, Serialize};

pub fn settings() -> Page<SettingsPageProps, (), ()> {
    Page::new("settings")
        .head_fn(head)
        .body_fn(body)
        .hydration_state(static_props)
        .incremental_generation()
}

fn static_props(_locale: &str, _path: &str, _params: &()) -> Option<SettingsPageProps> {
    Some(SettingsPageProps {
        mp_1_prefs: liturgy_to_preferences(&MORNING_PRAYER_II),
        mp_2_prefs: liturgy_to_preferences(&MORNING_PRAYER_II),
        np_prefs: liturgy_to_preferences(&NOONDAY_PRAYER),
        ep_1_prefs: None, // TODO
        ep_2_prefs: liturgy_to_preferences(&EVENING_PRAYER_II),
        cp_prefs: liturgy_to_preferences(&COMPLINE),
        eucharist_prefs: None, // TODO
    })
}

fn head(_locale: &str, _props: &SettingsPageProps, _render_state: &()) -> View {
    view! {
        <>
            <title>{t!("settings.title")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/vars.css"/>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/settings.css"/>
            <link rel="stylesheet" href="/static/display-settings.css"/>
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

fn body(locale: &str, props: &SettingsPageProps, _render_state: &()) -> View {
    let status = Behavior::new(Status::Idle);

    let display_settings = DisplaySettingsComponent::new();
    display_settings.current_settings.stream().create_effect({
        let status = status.clone();
        move |display_settings| {
            preference_effect(&status, || {
                preferences::set_display_settings(&display_settings)
            })
        }
    });

    let dark_mode_setting = SegmentButton::new_with_default_value(
        "dark_mode",
        Some(t!("settings.dark_mode.label")),
        [
            (
                None,
                t!("settings.dark_mode.auto"),
                Some(t!("settings.dark_mode.auto_explanation")),
            ),
            (
                Some("light".to_string()),
                t!("settings.dark_mode.light"),
                Some(t!("settings.dark_mode.light_explanation")),
            ),
            (
                Some("dark".to_string()),
                t!("settings.dark_mode.dark"),
                Some(t!("settings.dark_mode.dark_explanation")),
            ),
        ],
        preferences::get_raw(DARK_MODE_KEY),
    );
    dark_mode_setting.value.stream().skip(1).create_effect({
        let status = status.clone();

        move |new_value| {
            let body_class_list = leptos::body().unwrap().class_list();
            match new_value {
                Some(value) => {
                    preference_effect(&status, || {
                        preferences::set_raw(DARK_MODE_KEY, &value)?;
                        body_class_list
                            .remove_1("dark-mode-light")
                            .map_err(|_| StorageError::SettingStorage)?;
                        body_class_list
                            .remove_1("dark-mode-dark")
                            .map_err(|_| StorageError::SettingStorage)?;
                        body_class_list
                            .add_1(&format!("dark-mode-{}", value))
                            .map_err(|_| StorageError::SettingStorage)?;
                        Ok(())
                    });
                }
                None => {
                    preference_effect(&status, || {
                        preferences::clear_raw(DARK_MODE_KEY)?;
                        body_class_list
                            .remove_1("dark-mode-light")
                            .map_err(|_| StorageError::SettingStorage)?;
                        body_class_list
                            .remove_1("dark-mode-dark")
                            .map_err(|_| StorageError::SettingStorage)?;
                        Ok(())
                    });
                }
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
        (t!("bcp_1979"), PreferenceValue::Local("bcp1979".into())),
        (t!("lff_2018"), PreferenceValue::Local("lff2018".into())),
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
        }),
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
            (
                SlugPath::from([Slug::Office, Slug::MorningPrayer]),
                t!("toc.morning_prayer"),
                None,
            ),
            (
                SlugPath::from([Slug::Office, Slug::NoondayPrayer]),
                t!("toc.noonday_prayer"),
                None,
            ),
            (
                SlugPath::from([Slug::Office, Slug::EveningPrayer]),
                t!("toc.evening_prayer"),
                None,
            ),
            (
                SlugPath::from([Slug::Office, Slug::Compline]),
                t!("toc.compline"),
                None,
            ),
            (
                SlugPath::from([Slug::Office, Slug::Eucharist]),
                t!("toc.holy_eucharist"),
                None,
            ),
        ],
        SlugPath::from([Slug::Office, Slug::MorningPrayer]),
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
            <main class="settings">
                <h2>{t!("settings.display_settings.title")}</h2>
                <dyn:view view={dark_mode_setting.view()} />
                <dyn:view view={display_settings.view()} />

                <h2>{t!("settings.general")}</h2>
                <dyn:view view={version_setting.view()} />
                <dyn:view view={calendar_setting.view()} />
                <dyn:view view={psalm_cycle_setting.view()} />
                <dyn:view view={bible_version_setting.view()} />

                <h2>{t!("settings.liturgy")}</h2>
                <dyn:view view={liturgy_picker.view()} />
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != SlugPath::from([Slug::Office, Slug::MorningPrayer]) || contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, &SlugPath::from([Slug::Office, Slug::MorningPrayer]), Language::En, Version::RiteI, &props.mp_1_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != SlugPath::from([Slug::Office, Slug::MorningPrayer]) || !contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, &SlugPath::from([Slug::Office, Slug::MorningPrayer]), Language::En, Version::RiteII, &props.mp_2_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, _)| liturgy != SlugPath::from([Slug::Office, Slug::NoondayPrayer])).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, &SlugPath::from([Slug::Office, Slug::NoondayPrayer]), Language::En, Version::BCP1979, &props.np_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != SlugPath::from([Slug::Office, Slug::EveningPrayer]) || contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, &SlugPath::from([Slug::Office, Slug::EveningPrayer]), Language::En, Version::RiteI, &props.ep_1_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, contemporary)| liturgy != SlugPath::from([Slug::Office, Slug::EveningPrayer]) || !contemporary).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, &SlugPath::from([Slug::Office, Slug::EveningPrayer]), Language::En, Version::RiteII, &props.ep_2_prefs)} />
                </dyn:section>
                <dyn:section
                    class:hidden={liturgy_and_version.stream().map(|(liturgy, _)| liturgy != SlugPath::from([Slug::Office, Slug::Compline])).boxed_local()}
                >
                    <dyn:view view={liturgy_preferences_view(&status, &SlugPath::from([Slug::Office, Slug::Compline]), Language::En, Version::BCP1979, &props.cp_prefs)} />
                </dyn:section>

                <h2>{t!("settings.advanced")}</h2>
                <dyn:view view={gloria_patri_setting.view()} />
                <dyn:view view={black_letter_collect_setting.view()} />
            </main>
            {preference_status_footer(&status)}
        </>
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
