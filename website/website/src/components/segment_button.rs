use std::fmt::Debug;

use futures::StreamExt;
use leptos::*;

pub struct SegmentButton<T>
where
    T: Clone + PartialEq + 'static,
{
    pub value: Behavior<T>,
    options: Vec<(T, String, Option<String>)>,
    legend: Option<String>,
    name: &'static str,
}

impl<T> SegmentButton<T>
where
    T: Clone + PartialEq + 'static,
{
    #[allow(dead_code)]
    pub fn new(
        name: &'static str,
        legend: Option<String>,
        options: impl Into<Vec<(T, String, Option<String>)>>,
    ) -> Self
    where
        T: Default,
    {
        Self {
            name,
            value: Behavior::new(T::default()),
            legend,
            options: options.into(),
        }
    }

    pub fn new_with_default_value(
        name: &'static str,
        legend: Option<String>,
        options: impl Into<Vec<(T, String, Option<String>)>>,
        default_value: T,
    ) -> Self {
        Self {
            name,
            value: Behavior::new(default_value),
            legend,
            options: options.into(),
        }
    }

    pub fn view(&self) -> View
    where
        T: Debug,
    {
        let legend_view = match &self.legend {
            Some(legend) => view! { <legend>{legend}</legend> },
            None => View::Empty,
        };

        let options = View::Fragment(
            self.options
                .iter()
                .enumerate()
                .map(|(idx, (value, label, _))| {
                    let id = format!("{}-{}", self.name, idx);
                    let dom_value = idx.to_string();
                    let checked = self
                        .value
                        .stream()
                        .map({
                            let value = value.clone();
                            move |selected| {
                                if selected == value {
                                    Some("".to_string())
                                } else {
                                    None
                                }
                            }
                        })
                        .boxed_local();

                    view! {
                        <>
                            <dyn:input
                                type="radio"
                                id={&id}
                                name={self.name}
                                value={dom_value}
                                checked={checked}
                                on:change={on_change(&self.value, idx, value.clone())}
                            />
                            <label for={&id}>
                                {label}
                            </label>
                        </>
                    }
                })
                .collect(),
        );

        // Optional descriptions that show below the buttons
        let description_field = if self.options.iter().any(|(_, _, desc)| desc.is_some()) {
            let options = self.options.clone();
            let current_description = self
                .value
                .stream()
                .map(move |value| {
                    options
                        .iter()
                        .find(|(s_value, _, _)| s_value == &value)
                        .and_then(|(_, _, desc)| desc.as_ref().cloned())
                        .unwrap_or_default()
                })
                .boxed_local();
            view! {
                <dyn:p class="description">{current_description}</dyn:p>
            }
        } else {
            View::Empty
        };

        view! {
            <>
                <fieldset class="toggle segment-button">
                    {legend_view}
                    {options}
                </fieldset>
                {description_field}
            </>
        }
    }
}

fn on_change<T>(
    current_value: &Behavior<T>,
    idx: usize,
    value_for_option: T,
) -> impl Fn(web_sys::Event)
where
    T: Clone,
{
    let current_value = current_value.clone();
    move |ev: Event| {
        if let Ok(ev_idx) = event_target_value(ev).parse::<usize>() {
            if ev_idx == idx {
                current_value.set(value_for_option.clone())
            }
        }
    }
}
