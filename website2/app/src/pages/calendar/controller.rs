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

#[async_trait(?Send)]
impl State for CalendarController {
    type Msg = CalendarPageMsg;
    type Cmd = CalendarPageCmd;

    fn init(&self) -> Option<Self::Cmd> {
        match location_hash() {
            None => {
                let date = today();
                let date_hash = format!("{}-{}", date.month(), date.day());
                Some(CalendarPageCmd::SetHash(date_hash))
            }
            Some(hash) if hash.is_empty() => {
                let date = today();
                let date_hash = format!("{}-{}", date.month(), date.day());
                Some(CalendarPageCmd::SetHash(date_hash))
            }
            Some(hash) => Some(CalendarPageCmd::SetHash(hash)),
        }
    }

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        let cmd = match msg {
            CalendarPageMsg::SetDate(date) => {
                let date = date.unwrap_or_else(today);
                let date_hash = format!("{}-{}", date.month(), date.day());
                self.date = date_hash;
                Some(CalendarPageCmd::SetHash(self.date.clone()))
            }
            CalendarPageMsg::HashChange => {
                let hash = location_hash().unwrap_or_default();
                self.date = hash.clone();
                Some(CalendarPageCmd::ScrollTo(hash))
            }
            CalendarPageMsg::OpenMenu(open) => {
                self.menu_open = open;
                None
            }
            CalendarPageMsg::Noop => None,
        };
        cmd
    }

    async fn cmd(
        cmd: Self::Cmd,
        _host: web_sys::HtmlElement,
        _link: StateLink<Self>,
    ) -> Option<Self::Msg> {
        match cmd {
            CalendarPageCmd::ScrollTo(hash) => {
                scroll_to_element_by_id_with_padding_for_header(&hash);
            }
            CalendarPageCmd::SetHash(hash) => {
                location().set_hash(&hash);
            }
        }
        None
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
                    {include_str!("../../../static/toggle-links.css")}
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
