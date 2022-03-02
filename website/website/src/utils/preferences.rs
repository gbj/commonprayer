use std::time::Duration;

use episcopal_api::liturgy::{PreferenceKey, PreferenceValue};
use futures::StreamExt;
use leptos::*;

use crate::preferences::{self, StorageError};

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
