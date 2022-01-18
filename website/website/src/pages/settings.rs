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

fn body(locale: &str, _props: &()) -> View {
    let status = Behavior::new(Status::Idle);

    // Calendar preference
    let calendar_toggle = Toggle::new(
        if is_server!() {
            false
        } else {
            preferences::get(&PreferenceKey::from(GlobalPref::Calendar))
                == Some(PreferenceValue::from("lff2018"))
        },
        "calendar",
        t!("bcp_1979"),
        t!("lff_2018"),
    );

    // On initial load from localStorage or default, don't display message/set preference redundantly
    calendar_toggle.toggled.stream().skip(1).create_effect({
        let status = status.clone();
        move |use_lff| {
            if use_lff {
                set_preference(
                    &status,
                    PreferenceKey::from(GlobalPref::Calendar),
                    PreferenceValue::from("lff2018"),
                );
            } else {
                set_preference(
                    &status,
                    PreferenceKey::from(GlobalPref::Calendar),
                    PreferenceValue::from("bcp1979"),
                );
            }
        }
    });

    // Psalm cycle preference
    let psalm_cycle_toggle = Toggle::new(
        if is_server!() {
            false
        } else {
            preferences::get(&PreferenceKey::from(GlobalPref::PsalmCycle))
                == Some(PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms))
        },
        "psalm_cycle",
        t!("daily_readings.daily_office_psalms"),
        t!("daily_readings.thirty_day_psalms"),
    );

    // On initial load from localStorage or default, don't display message/set preference redundantly
    psalm_cycle_toggle.toggled.stream().skip(1).create_effect({
        let status = status.clone();
        move |use_30| {
            if use_30 {
                set_preference(
                    &status,
                    PreferenceKey::from(GlobalPref::PsalmCycle),
                    PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms),
                );
            } else {
                set_preference(
                    &status,
                    PreferenceKey::from(GlobalPref::Calendar),
                    PreferenceValue::from(Lectionaries::BCP1979DailyOfficePsalms),
                );
            }
        }
    });

    view! {
        <>
            {header(locale, &t!("settings.title"))}
            <main>
                <dyn:view view={calendar_toggle.view()} />
                <dyn:view view={psalm_cycle_toggle.view()} />
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

#[derive(Copy, Clone, PartialEq, Eq)]
enum Status {
    Idle,
    Success,
    Error,
    NotAvailable,
}

fn set_preference(status: &Behavior<Status>, key: PreferenceKey, value: PreferenceValue) {
    if !is_server!() {
        leptos::log(&format!("setting preference {:#?} to {:#?}", key, value));
        let (new_status, delay_before_clearing) = match preferences::set(&key, &value) {
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
