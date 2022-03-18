use calendar::Date;
use futures::StreamExt;
use leptos::*;

use crate::utils::time::today;

pub struct DatePicker {
    label: String,
    pub date: Behavior<Option<Date>>,
}

impl DatePicker {
    pub fn new(label: String, initial_date: Option<Date>) -> Self {
        Self {
            label,
            date: Behavior::new(initial_date),
        }
    }

    pub fn view(&self) -> View {
        let formatted_initial_date = self.date.get().unwrap_or_else(today).to_padded_string();
        let on_change = {
            let date = self.date.clone();
            move |ev: Event| {
                date.set(Date::parse_from_str(&event_target_value(ev), "%Y-%m-%d").ok())
            }
        };

        view! {
            <>
                <fieldset class="centered stacked date-picker">
                    <label>
                        {&self.label}
                        <dyn:input
                            type="date"
                            value={formatted_initial_date}
                            prop:value={self.date.stream().map(|date| date.map(|date| date.to_padded_string())).boxed_local()}
                            on:change={on_change}
                        />
                    </label>
                </fieldset>
                <dyn:button
                    on:click={
                        let date = self.date.clone();
                        move |_| date.set(Some(today()))
                    }
                >
                    {t!("today")}
                </dyn:button>
            </>
        }
    }
}
