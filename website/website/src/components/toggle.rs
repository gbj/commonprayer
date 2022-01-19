use futures::StreamExt;
use leptos::{event_target_value, view, Behavior, Event, View};

pub struct Toggle {
    name: &'static str,
    on_label: String,
    off_label: String,
    legend: Option<String>,
    pub toggled: Behavior<bool>,
}

impl Toggle {
    pub fn new(
        is_toggled: bool,
        name: &'static str,
        off_label: String,
        on_label: String,
        legend: Option<String>,
    ) -> Self {
        Self {
            name,
            on_label,
            off_label,
            legend,
            toggled: Behavior::new(is_toggled),
        }
    }

    pub fn view(&self) -> View {
        let id_off = format!("{}-off", self.name);
        let id_on = format!("{}-on", self.name);

        let on_change = {
            let toggled = self.toggled.clone();
            move |ev: Event| toggled.set(event_target_value(ev) == "on")
        };

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

        let legend_view = match &self.legend {
            Some(legend) => view! { <legend>{legend}</legend> },
            None => View::Empty,
        };

        view! {
            <fieldset class="toggle">
                {legend_view}
                <dyn:input
                    type="radio"
                    id={&id_off}
                    name={self.name}
                    value="off"
                    checked={if self.toggled.get() { None } else { Some(String::default()) }}
                    checked={off_checked}
                    on:change={on_change.clone()}
                />
                <label for={&id_off}>
                    {&self.off_label}
                </label>
                <dyn:input
                    type="radio"
                    id={&id_on}
                    name={self.name}
                    value="on"
                    checked={if self.toggled.get() { Some(String::default()) } else { None }}
                    checked={on_checked}
                    on:change={on_change}
                />
                <label for={&id_on}>
                    {&self.on_label}
                </label>
            </fieldset>
        }
    }
}
