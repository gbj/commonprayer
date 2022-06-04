use calendar::{Date, TimeOfDay};
use leptos2::*;
use liturgy::{GlobalPref, Lectionaries, PreferenceKey, PreferenceValue};
use wasm_bindgen::UnwrapThrowExt;

use crate::components::{DatePicker, Toggle, ToggleEventDetail};
use crate::pages::CalendarChoice;
use crate::preferences;
use crate::utils::time::current_hour;

use super::redirect::RedirectMode;

#[derive(Clone, Debug, Default, PartialEq, WebComponent)]
pub struct DailyOfficeView {
    pub locale: String,
    #[prop]
    pub date: Option<Date>,
    pub calendar: CalendarChoice,
    pub time: TimeOfDay,
    pub psalms: Lectionaries,
    pub morning_observance: String,
    pub morning_alternate: String,
    pub evening_observance: String,
    pub evening_alternate: String,
    use_alternate: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DailyOfficeViewMsg {
    Noop,
    SetDate(Date),
    SetCalendar(CalendarChoice),
    SetTime(TimeOfDay),
    SetPsalmCycle(Lectionaries),
    UseAlternate(bool),
}

#[derive(Clone, PartialEq, Debug)]
pub enum DailyOfficeViewCmd {
    SetDate(String, Date),
}

#[async_trait(?Send)]
impl State for DailyOfficeView {
    type Msg = DailyOfficeViewMsg;
    type Cmd = DailyOfficeViewCmd;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        match msg {
            DailyOfficeViewMsg::Noop => {}
            DailyOfficeViewMsg::SetDate(date) => {
                return Some(Self::Cmd::SetDate(self.locale.clone(), date))
            }
            DailyOfficeViewMsg::SetCalendar(calendar) => self.calendar = calendar,
            DailyOfficeViewMsg::SetTime(time) => self.time = time,
            DailyOfficeViewMsg::SetPsalmCycle(cycle) => self.psalms = cycle,
            DailyOfficeViewMsg::UseAlternate(use_alternate) => self.use_alternate = use_alternate,
        }
        None
    }

    async fn cmd(
        cmd: Self::Cmd,
        _host: web_sys::HtmlElement,
        _link: StateLink<Self>,
    ) -> Option<Self::Msg> {
        match cmd {
            DailyOfficeViewCmd::SetDate(locale, date) => {
                let mode = RedirectMode::DailyOffice;
                location()
                    .set_href(&format!("/{locale}/readings/{mode}/{date}"))
                    .unwrap_throw();
            }
        }
        None
    }
}

impl Component for DailyOfficeView {
    fn view(&self) -> Host {
        let links_slot = match (self.use_alternate, self.psalms) {
            (true, Lectionaries::BCP1979ThirtyDayPsalms) => "alternate_links_30day_psalms",
            (true, _) => "alternate_links_daily_psalms",
            (false, Lectionaries::BCP1979ThirtyDayPsalms) => "primary_links_30day_psalms",
            (false, _) => "primary_links_daily_psalms",
        };

        let header_slot = match (self.use_alternate, self.time, self.calendar) {
            (true, TimeOfDay::Morning, CalendarChoice::LFF2018) => "alternate_morning_header_lff",
            (true, TimeOfDay::Morning, _) => "alternate_morning_header",
            (true, TimeOfDay::Evening, CalendarChoice::LFF2018) => "alternate_evening_header_lff",
            (true, TimeOfDay::Evening, _) => "alternate_evening_header",
            (false, TimeOfDay::Morning, CalendarChoice::LFF2018) => "primary_morning_header_lff",
            (false, TimeOfDay::Morning, _) => "primary_morning_header",
            (false, TimeOfDay::Evening, CalendarChoice::LFF2018) => "primary_evening_header_lff",
            (false, TimeOfDay::Evening, _) => "primary_evening_header",
        };

        let psalms_slot = match (self.use_alternate, self.time, self.psalms) {
            (_, TimeOfDay::Morning, Lectionaries::BCP1979ThirtyDayPsalms) => "morning_30",
            (_, TimeOfDay::Evening, Lectionaries::BCP1979ThirtyDayPsalms) => "evening_30",
            (false, TimeOfDay::Morning, _) => "primary_morning_daily",
            (false, TimeOfDay::Evening, _) => "primary_evening_daily",
            (true, TimeOfDay::Morning, _) => "alternate_morning_daily",
            (true, TimeOfDay::Evening, _) => "alternate_evening_daily",
        };

        let readings_slot = match (self.use_alternate, self.time) {
            (true, TimeOfDay::Morning) => "alternate_morning_readings",
            (true, TimeOfDay::Evening) => "alternate_evening_readings",
            (false, TimeOfDay::Morning) => "primary_morning_readings",
            (false, TimeOfDay::Evening) => "primary_evening_readings",
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
                        leptos2::warn(&format!("received datepicker change event {:#?}", ev.detail));
                        if let Some(Some(date)) = ev.detail {
                            Self::Msg::SetDate(date)
                        } else {
                            Self::Msg::Noop
                        }
                    }
                />
                <Toggle
                    name="calendar"
                    off={t!("bcp_1979")}
                    on={t!("lff_2018")}
                    toggled={preferences::is(
                        &PreferenceKey::from(GlobalPref::Calendar),
                        &PreferenceValue::Local("lff2018".into()),
                    )}
                    on:change=|ev: web_sys::Event| {
                        let ev: CustomEvent<ToggleEventDetail> = ev.into();
                        Self::Msg::SetCalendar(match ev.detail {
                            Some(ToggleEventDetail { toggled, .. }) if toggled => CalendarChoice::LFF2018,
                            _ => CalendarChoice::BCP1979
                        })
                    }
                />
                <Toggle
                    name="time"
                    off={t!("daily_readings.morning")}
                    on={t!("daily_readings.evening")}
                    toggled={current_hour() >= 13}
                    on:change=|ev: web_sys::Event| {
                        let ev: CustomEvent<ToggleEventDetail> = ev.into();
                        Self::Msg::SetTime(match ev.detail {
                            Some(ToggleEventDetail { toggled, .. }) if toggled => TimeOfDay::Evening,
                            _ => TimeOfDay::Morning
                        })
                    }
                />
                <Toggle
                    name="psalm_cycle"
                    off={t!("daily_readings.daily_office_psalms")}
                    on={t!("daily_readings.thirty_day_psalms")}
                    toggled={preferences::is(
                        &PreferenceKey::from(GlobalPref::PsalmCycle),
                        &PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms),
                    )}
                    on:change=|ev: web_sys::Event| {
                        let ev: CustomEvent<ToggleEventDetail> = ev.into();
                        Self::Msg::SetPsalmCycle(match ev.detail {
                            Some(ToggleEventDetail { toggled, .. }) if toggled => Lectionaries::BCP1979ThirtyDayPsalms,
                            _ => Lectionaries::BCP1979DailyOfficePsalms
                        })
                    }
                />
                <Toggle
                    name="morning_alternate"
                    off={&self.morning_observance}
                    on={&self.morning_alternate}
                    class:hidden={self.morning_alternate.is_empty() || self.time == TimeOfDay::Evening}
                    on:change=|ev: web_sys::Event| {
                        let ev: CustomEvent<ToggleEventDetail> = ev.into();
                        Self::Msg::UseAlternate(ev.detail.map(|detail| detail.toggled).unwrap_or(false))
                    }
                />
                <Toggle
                    name="evening_alternate"
                    off={&self.evening_observance}
                    on={&self.evening_alternate}
                    class:hidden={self.evening_alternate.is_empty() || self.time == TimeOfDay::Morning}
                    on:change=|ev: web_sys::Event| {
                        let ev: CustomEvent<ToggleEventDetail> = ev.into();
                        Self::Msg::UseAlternate(ev.detail.map(|detail| detail.toggled).unwrap_or(false))
                    }
                />

                // Slots for links down to readings
                <slot name={links_slot}></slot>

                // Day name, holy days w/ links, and collect
                <slot name={header_slot}></slot>

                // Slots for psalms and readings
                <slot name={psalms_slot}></slot>
                <slot name={readings_slot}></slot>
            </Host>
        }
    }

    fn should_render(&self, msg: &Self::Msg) -> bool {
        msg != &Self::Msg::Noop
    }
}
