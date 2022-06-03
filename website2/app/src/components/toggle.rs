use std::fmt::Display;

use leptos2::*;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct Toggle {
    pub name: String,
    pub on: String,
    pub off: String,
    pub legend: String,
    #[prop]
    pub toggled: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct ToggleEventDetail {
    pub name: String,
    pub toggled: bool,
}

impl Toggle {
    pub fn new(toggled: bool, name: impl Display, off: String, on: String, legend: String) -> Self {
        Self {
            name: name.to_string(),
            on,
            off,
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

    async fn cmd(
        cmd: Self::Cmd,
        host: web_sys::HtmlElement,
        _link: StateLink<Self>,
    ) -> Option<Self::Msg> {
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
                        checked={!self.toggled}
                        prop:checked={!self.toggled}
                        on:change=|ev| event_target_value(&ev) == "on"
                    />
                    <label for={&id_off}>
                        {&self.off}
                    </label>
                    <input
                        type="radio"
                        id={&id_on}
                        name={&self.name}
                        value="on"
                        checked={self.toggled}
                        prop:checked={self.toggled}
                        on:change=|ev| event_target_value(&ev) == "on"
                    />
                    <label for={&id_on}>
                        {&self.on}
                    </label>
                </fieldset>
            </Host>
        }
    }
}
