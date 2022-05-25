use std::fmt::Display;

use leptos2::*;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct Toggle {
    pub name: String,
    pub on_label: String,
    pub off_label: String,
    pub legend: String,
    pub toggled: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ToggleEventDetail {
    pub name: String,
    pub toggled: bool,
}

impl Toggle {
    pub fn new(
        toggled: bool,
        name: impl Display,
        off_label: String,
        on_label: String,
        legend: String,
    ) -> Self {
        Self {
            name: name.to_string(),
            on_label,
            off_label,
            legend,
            toggled,
        }
    }
}

#[async_trait(?Send)]
impl State for Toggle {
    type Msg = bool;
    type Cmd = ToggleEventDetail;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        self.toggled = msg;
        Some(ToggleEventDetail {
            name: self.name.to_string(),
            toggled: msg,
        })
    }

    async fn cmd(cmd: Self::Cmd, host: web_sys::HtmlElement) -> Option<Self::Msg> {
        let event_emitter = EventEmitter::new(&host);
        event_emitter.emit(CustomEvent::new("change").detail(cmd));
        None
    }
}

impl Component for Toggle {
    fn view(&self) -> Host {
        let id_off = format!("{}-off", self.name);
        let id_on = format!("{}-on", self.name);

        let legend_view = if self.legend.is_empty() {
            text("")
        } else {
            view! { <legend>{&self.legend}</legend> }
        };

        view! {
            <Host>
                <style>{include_str!("./toggle.css")}</style>
                <fieldset class="toggle">
                    {legend_view}
                    <input
                        type="radio"
                        id={&id_off}
                        name={&self.name}
                        value="off"
                        prop:checked={!self.toggled}
                        on:change=|ev| event_target_value(&ev) == "on"
                    />
                    <label for={&id_off}>
                        {&self.off_label}
                    </label>
                    <input
                        type="radio"
                        id={&id_on}
                        name={&self.name}
                        value="on"
                        prop:checked={self.toggled}
                        on:change=|ev| event_target_value(&ev) == "on"
                    />
                    <label for={&id_on}>
                        {&self.on_label}
                    </label>
                </fieldset>
            </Host>
        }
    }
}
