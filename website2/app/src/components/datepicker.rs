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

#[async_trait(?Send)]
impl Component for DatePicker {
    type Msg = Option<Date>;
    type Cmd = Option<Date>;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        self.date = msg;
        Some(msg)
    }

    async fn cmd(cmd: Self::Cmd, host: web_sys::HtmlElement) -> Option<Self::Msg> {
        let event_emitter = EventEmitter::new(&host);
        event_emitter.emit(CustomEvent::new("change").detail(cmd));
        None
    }

    fn view(&self) -> Host {
        view! {
            <Host>
                <fieldset class="centered stacked date-picker">
                    <label>
                        {&self.label}
                        <input
                            type="date"
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
