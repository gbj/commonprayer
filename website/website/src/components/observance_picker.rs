use std::rc::Rc;

use api::summary::{DailySummary, PartialDailySummary};
use futures::stream::{Stream, StreamExt};
use leptos::*;

pub struct ObservancePicker {
    pub toggled: Behavior<bool>,
    summary: Rc<DailySummary>,
    is_evening: Behavior<bool>,
}

impl ObservancePicker {
    pub fn new(is_evening: Behavior<bool>, summary: Rc<DailySummary>) -> Self {
        Self {
            toggled: Behavior::new(false),
            is_evening,
            summary,
        }
    }

    fn partial_daily_summary(&self) -> impl Stream<Item = PartialDailySummary> + 'static {
        let summary = self.summary.clone();
        self.is_evening.stream().map(move |evening| {
            if evening {
                summary.evening.clone()
            } else {
                summary.morning.clone()
            }
        })
    }

    pub fn view(&self) -> View {
        let primary_name = self
            .partial_daily_summary()
            .map(|summary| summary.observed.localized_name)
            .boxed_local();

        let alternate_name = self
            .partial_daily_summary()
            .map(|summary| {
                summary
                    .alternate
                    .as_ref()
                    .map(|alternate| alternate.localized_name.clone())
                    .unwrap_or_default()
            })
            .boxed_local();

        let initial_has_observance = if self.is_evening.get() {
            self.summary.evening.alternate.is_some()
        } else {
            self.summary.morning.alternate.is_some()
        };
        let has_observance = self
            .partial_daily_summary()
            .map(|summary| summary.alternate.is_some());

        let off_checked = self
            .toggled
            .stream()
            .map(|toggled| if !toggled { Some("".to_string()) } else { None })
            .boxed_local();

        let on_checked = self
            .toggled
            .stream()
            .map(|toggled| if toggled { Some("".to_string()) } else { None })
            .boxed_local();

        let on_change = {
            let toggled = self.toggled.clone();
            move |ev: Event| toggled.set(event_target_value(ev) == "on")
        };

        view! {
            <dyn:fieldset class={if initial_has_observance { "toggle" } else { "toggle no-observance"}}
                class:no_observance={has_observance.map(|has| !has).boxed_local()}
            >
                <dyn:input
                    type="radio"
                    id="observed"
                    name="alternate_observance"
                    value="off"
                    checked={if self.toggled.get() { None } else { Some(String::default()) }}
                    checked={off_checked}
                    on:change={on_change.clone()}
                />
                <dyn:label for="observed">
                    {primary_name}
                    " "
                    {t!("daily_readings.default")}
                </dyn:label>
                <dyn:input
                    type="radio"
                    id="alternate"
                    name="alternate_observance"
                    value="on"
                    checked={if self.toggled.get() { Some(String::default()) } else { None }}
                    checked={on_checked}
                    on:change={on_change}
                />
                <dyn:label for="alternate">
                    {alternate_name}
                    " "
                    {t!("daily_readings.alternate")}
                </dyn:label>
            </dyn:fieldset>
        }
    }
}
