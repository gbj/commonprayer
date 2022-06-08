use leptos2::*;

use calendar::Date;

use crate::utils::time::today;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct DatePicker {
    pub label: String,
    #[prop]
    pub date: Option<Date>,
}

impl DatePicker {
    pub fn new(label: String, date: Option<Date>) -> Self {
        Self { label, date }
    }
}

impl State for DatePicker {
    type Msg = Option<Date>;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        self.date = msg;
        Some(Cmd::event(CustomEvent::new("change").detail(self.date)))
    }
}

impl Component for DatePicker {
    fn view(&self) -> Host {
        view! {
            <Host>
                <style>{include_str!("datepicker.css")}</style>
                <fieldset>
                    <label>
                        {&self.label}
                        <input
                            type="date"
                            value={self.date.map(|date| date.to_padded_string())}
                            prop:value={self.date.map(|date| date.to_padded_string())}
                            on:change=|ev: Event| (Date::parse_from_str(&event_target_value(&ev), "%Y-%m-%d").ok())
                        />
                    </label>
                </fieldset>
                <button
                    on:click=|_| Some(today())
                >
                    {t!("today")}
                </button>
            </Host>
        }
    }
}
