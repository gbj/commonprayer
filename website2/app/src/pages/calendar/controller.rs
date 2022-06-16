use super::calendar_toggle_links;
use crate::{
    components::*,
    utils::{scroll_to_element_by_id_with_padding_for_header, time::today},
};
use calendar::Date;
use leptos2::*;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct CalendarController {
    pub locale: String,
    #[prop]
    pub lff: bool,
    date: String,
    menu_open: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CalendarPageMsg {
    HashChange,
    SetDate(Option<Date>),
    OpenMenu(bool),
    Noop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CalendarPageCmd {
    ScrollTo(String),
    SetHash(String),
}

impl State for CalendarController {
    type Msg = CalendarPageMsg;

    fn init(&self) -> Option<Cmd<Self>> {
        let hash = match location_hash() {
            None => {
                let date = today();
                let date_hash = format!("{}-{}", date.month(), date.day());
                date_hash
            }
            Some(hash) if hash.is_empty() => {
                let date = today();
                let date_hash = format!("{}-{}", date.month(), date.day());
                date_hash
            }
            Some(hash) => hash,
        };
        Some(self.set_hash(hash))
    }

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        let cmd = match msg {
            CalendarPageMsg::SetDate(date) => {
                let date = date.unwrap_or_else(today);
                let date_hash = format!("{}-{}", date.month(), date.day());
                self.date = date_hash.clone();
                Some(self.set_hash(date_hash))
            }
            CalendarPageMsg::HashChange => {
                let hash = location_hash().unwrap_or_default();
                self.date = hash.clone();
                Some(self.scroll_to(hash))
            }
            CalendarPageMsg::OpenMenu(open) => {
                self.menu_open = open;
                None
            }
            CalendarPageMsg::Noop => None,
        };
        cmd
    }
}

impl Component for CalendarController {
    fn view(&self) -> Host {
        let initial_date = if is_server!() { None } else { location_hash() }.and_then(|hash| {
            let today = today();
            let year = today.year();
            Date::parse_from_str(&format!("{}-{}", year, hash), "%Y-%m-%d")
                .ok()
                .or(Some(today))
        });
        let initial_date = serde_json::to_string(&initial_date).unwrap();

        view! {
            <Host
                window:hashchange=|_| CalendarPageMsg::HashChange
                window:click=|ev: Event| {
                    if event_target_selector(&ev, "button#modalOpen") {
                        CalendarPageMsg::OpenMenu(true)
                    } else {
                        CalendarPageMsg::Noop
                    }
                }
            >
                <style>
                    ".hidden { display: none; }"
                    {include_str!("../../styles/toggle-links.css")}
                </style>
                <Modal
                    prop:open={self.menu_open}
                    on:close=|_| CalendarPageMsg::OpenMenu(false)
                >
                    <div slot="content">{calendar_toggle_links(&self.locale, self.lff)}</div>
                    <DatePicker
                        slot="content"
                        id="calendar-date"
                        label=t!("date")
                        date={&initial_date}
                        on:change=|ev: Event| {
                            let ev: CustomEvent<Option<Date>> = ev.into();
                            CalendarPageMsg::SetDate(ev.detail.flatten())
                        }
                    />
                </Modal>
            </Host>
        }
    }
}

impl CalendarController {
    fn set_hash(&self, hash: String) -> Cmd<Self> {
        Cmd::new(move |_, _| {
            location().set_hash(&hash);
        })
    }

    fn scroll_to(&self, hash: String) -> Cmd<Self> {
        Cmd::new(move |_, _| {
            scroll_to_element_by_id_with_padding_for_header(&hash);
        })
    }
}
