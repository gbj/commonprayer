use std::time::Duration;

use crate::{
    components::{header, Toggle},
    preferences::{self, StorageError},
};
use episcopal_api::liturgy::{GlobalPref, Lectionaries, PreferenceKey, PreferenceValue};
use futures::StreamExt;
use leptos::*;
use rust_i18n::t;

pub fn settings() -> Page<(), ()> {
    Page::new("settings")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(|_, _, _| ())
}

fn head(_locale: &str, _props: &()) -> View {
    view! {
        <>
            <title>{t!("settings.title")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/settings.css"/>
        </>
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Status {
    Idle,
    Success,
    Error,
    NotAvailable,
}

fn body(locale: &str, _props: &()) -> View {
    let status = Behavior::new(Status::Idle);

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

    view! {
        <>
            {header(locale, &t!("settings.title"))}
            <main>
                <dyn:view view={calendar_setting} />
                <dyn:view view={psalm_cycle_setting} />
                <dyn:view view={black_letter_collect_setting} />
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

fn setting_toggle(
    status: &Behavior<Status>,
    name: &'static str,
    key: PreferenceKey,
    legend: String,
    off: (String, PreferenceValue),
    on: (String, PreferenceValue),
    toggled_by_default: bool,
) -> View {
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

    toggle.view()
}

fn set_preference(status: &Behavior<Status>, key: &PreferenceKey, value: &PreferenceValue) {
    if !is_server!() {
        leptos::log(&format!("setting preference {:#?} to {:#?}", key, value));
        let (new_status, delay_before_clearing) = match preferences::set(key, value) {
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
