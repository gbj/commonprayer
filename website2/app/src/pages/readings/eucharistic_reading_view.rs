use calendar::{Date, TimeOfDay};
use lectionary::RCLTrack;
use leptos2::*;
use liturgy::{GlobalPref, Lectionaries, PreferenceKey, PreferenceValue};
use wasm_bindgen::UnwrapThrowExt;

use crate::components::{DatePicker, Toggle, ToggleEventDetail};
use crate::pages::CalendarChoice;
use crate::preferences;
use crate::utils::time::current_hour;

use super::redirect::RedirectMode;

#[derive(Clone, Debug, Default, PartialEq, WebComponent)]
pub struct EucharisticReadingView {
    pub locale: String,
    #[prop]
    pub date: Option<Date>,
    #[prop]
    pub tracked: bool,
    track: Option<RCLTrack>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EucharisticReadingViewMsg {
    Noop,
    SetDate(Date),
    SetTrack(RCLTrack),
}

#[derive(Clone, PartialEq, Debug)]
pub enum EucharisticReadingViewCmd {
    SetDate(String, Date),
}

#[async_trait(?Send)]
impl State for EucharisticReadingView {
    type Msg = EucharisticReadingViewMsg;
    type Cmd = EucharisticReadingViewCmd;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        match msg {
            EucharisticReadingViewMsg::Noop => {}
            EucharisticReadingViewMsg::SetDate(date) => {
                return Some(Self::Cmd::SetDate(self.locale.clone(), date))
            }
            EucharisticReadingViewMsg::SetTrack(track) => self.track = Some(track),
        }
        None
    }

    async fn cmd(
        cmd: Self::Cmd,
        _host: web_sys::HtmlElement,
        _link: StateLink<Self>,
    ) -> Option<Self::Msg> {
        match cmd {
            EucharisticReadingViewCmd::SetDate(locale, date) => {
                let mode = RedirectMode::Eucharist;
                location()
                    .set_href(&format!("/{locale}/readings/{mode}/{date}"))
                    .unwrap_throw();
            }
        }
        None
    }
}

impl Component for EucharisticReadingView {
    fn view(&self) -> Host {
        let track_slot = match self.track {
            Some(RCLTrack::Two) if self.tracked => "track_two",
            Some(RCLTrack::One) | None if self.tracked => "track_one",
            _ => "untracked",
        };

        view! {
            <Host>
                <style>{include_str!("daily_office_view.css")}</style>

                // Controls
                <DatePicker
                    label={t!("date")}
                    date={self.date}
                    on:change=|ev: web_sys::Event| {
                        let ev: CustomEvent<Option<Date>> = ev.into();
                        if let Some(Some(date)) = ev.detail {
                            Self::Msg::SetDate(date)
                        } else {
                            Self::Msg::Noop
                        }
                    }
                />

                <slot name="header"></slot>

                <Toggle
                    name="track"
                    off={t!("lectionary.track_one")}
                    on={t!("lectionary.track_two")}
                    class:hidden={!self.tracked}
                    on:change=|ev: web_sys::Event| {
                        let ev: CustomEvent<ToggleEventDetail> = ev.into();
                        Self::Msg::SetTrack(match ev.detail {
                            Some(ToggleEventDetail { toggled, .. }) if toggled => RCLTrack::Two,
                            _ => RCLTrack::One
                        })
                    }
                />
                <slot name={track_slot}></slot>

                <slot name="readings"></slot>
            </Host>
        }
    }

    fn should_render(&self, msg: &Self::Msg) -> bool {
        msg != &Self::Msg::Noop
    }
}
