use std::fmt::Display;

use futures::StreamExt;
use leptos::*;
pub struct SearchBar {
    pub value: Behavior<String>,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            value: Behavior::new(String::default()),
        }
    }

    pub fn new_with_default_value(value: impl Display) -> Self {
        Self {
            value: Behavior::new(value.to_string()),
        }
    }

    pub fn view(&self) -> View {
        view! {
            <label class="stacked">
                {t!("search")}
                <dyn:input
                    type="search"
                    value={self.value.get()}
                    prop:value={self.value.stream().map(Some).boxed_local()}
                    on:input={
                        let value = self.value.clone();
                        move |ev: Event| value.set(event_target_value(ev))
                    }
                />
            </label>
        }
    }
}
