use crate::{
    components::*,
    utils::{scroll_to_element_by_id_with_padding_for_header, time::today},
};
use calendar::Date;
use leptos2::*;

use super::root_id;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct CalendarController {
    pub lff: bool,
    date: String,
    menu_open: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CalendarPageMsg {
    HashChange,
    UseLff(bool),
    SetDate(Option<Date>),
    OpenMenu(bool),
    Noop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CalendarPageCmd {
    ScrollTo { lff: bool, date_hash: String },
    SetHash(String),
}

#[async_trait(?Send)]
impl Component for CalendarController {
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

    fn update(&mut self, msg: &Self::Msg) -> (bool, Option<Self::Cmd>) {
        let cmd = match msg {
            CalendarPageMsg::UseLff(lff) => {
                self.lff = *lff;
                Some(CalendarPageCmd::ScrollTo {
                    lff: *lff,
                    date_hash: self.date.clone(),
                })
            }
            CalendarPageMsg::SetDate(date) => {
                let date = date.unwrap_or_else(today);
                let date_hash = format!("{}-{}", date.month(), date.day());
                self.date = date_hash;
                Some(CalendarPageCmd::SetHash(self.date.clone()))
            }
            CalendarPageMsg::HashChange => {
                let hash = location_hash().unwrap_or_default();
                self.date = hash.clone();
                Some(CalendarPageCmd::ScrollTo {
                    lff: self.lff,
                    date_hash: hash,
                })
            }
            CalendarPageMsg::OpenMenu(open) => {
                self.menu_open = *open;
                None
            }
            CalendarPageMsg::Noop => None,
        };
        (true, cmd)
    }

    async fn cmd(cmd: Self::Cmd, _host: web_sys::HtmlElement) -> Option<Self::Msg> {
        match cmd {
            CalendarPageCmd::ScrollTo { lff, date_hash } => {
                scroll_to_row(lff, &date_hash);
            }
            CalendarPageCmd::SetHash(hash) => {
                location().set_hash(&hash);
            }
        }
        None
    }

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
                <style>".hidden { display: none; }"</style>
                <Modal
                    open={self.menu_open}
                    on:close=|_| CalendarPageMsg::OpenMenu(false)
                >
                    <Toggle
                        slot="content"
                        id="calendar-toggle"
                        toggled={self.lff}
                        name="calendar"
                        off-label=t!("bcp_1979_abbrev")
                        on-label=t!("lff_2018_abbrev")
                        legend=t!("settings.calendar")
                        on:change=|ev: Event| {
                            let lff: CustomEvent<ToggleEventDetail> = ev.into();
                            let lff = lff.detail.map(|detail| detail.toggled).unwrap_or_default();
                            CalendarPageMsg::UseLff(lff)
                        }
                    />
                    <DatePicker
                        slot="content"
                        id="calendar-date"
                        label=t!("date")
                        date={initial_date}
                        on:change=|ev: Event| {
                            let ev: CustomEvent<Option<Date>> = ev.into();
                            CalendarPageMsg::SetDate(ev.detail.flatten())
                        }
                    />
                </Modal>

                // Content
                <main>
                    <slot name="bcp-title" class:hidden={self.lff}></slot>
                    <slot name="lff-title" class:hidden={!self.lff}></slot>
                    <slot name="bcp-content" class:hidden={self.lff}></slot>
                    <slot name="lff-content" class:hidden={!self.lff}></slot>
                </main>
            </Host>
        }
    }
}

fn scroll_to_row(lff: bool, hash: &str) {
    let root_id = root_id(lff);
    scroll_to_element_by_id_with_padding_for_header(&format!("{}-{}", root_id, hash));
}
