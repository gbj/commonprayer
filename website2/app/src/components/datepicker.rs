use leptos2::*;

use calendar::Date;

use crate::utils::time::{today, TimezoneOffset};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct DatePicker {
    pub label: String,
    pub todaylabel: String,
    #[prop]
    pub date: Option<Date>,
}

impl DatePicker {
    pub fn new(label: String, todaylabel: String, date: Option<Date>) -> Self {
        Self {
            label,
            todaylabel,
            date,
        }
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
        let tzoffset = TimezoneOffset(js_sys::Date::new_0().get_timezone_offset() as i32);

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
                    on:click=move |_| Some(today(&tzoffset))
                >
                    {if self.todaylabel.is_empty() {
                        "Today"
                    } else {
                        &self.todaylabel
                    }}
                </button>
            </Host>
        }
    }
}
