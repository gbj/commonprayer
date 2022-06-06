mod controller;
use std::str::FromStr;

pub use controller::CalendarController;
use strum_macros::{Display, EnumString};

use crate::{
    preferences,
    views::{Header, Icon},
};
use calendar::{
    feasts::KalendarEntry, Calendar, Feast, HolyDayId, Rank, Time, BCP1979_CALENDAR,
    LFF2018_CALENDAR,
};
use language::Language;
use leptos2::*;
use liturgy::{GlobalPref, PreferenceKey, PreferenceValue};
use rust_i18n::t;
use serde_derive::{Deserialize, Serialize};

pub struct CalendarPage {
    default_calendar: CalendarChoice,
    bcp1979: CalendarListing,
    lff2018: CalendarListing,
}

impl Page for CalendarPage {
    type Params = ();
    type Query = ();

    fn name() -> &'static str {
        "calendar"
    }

    fn paths() -> Vec<String> {
        vec!["".into(), "bcp1979".into(), "lff2018".into()]
    }

    fn build_state(
        locale: &str,
        path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let language = Language::from_locale(locale);

        let default_calendar = if path.ends_with("lff2018") {
            CalendarChoice::LFF2018
        } else if path.ends_with("bcp1979") {
            CalendarChoice::BCP1979
        } else {
            CalendarChoice::None
        };
        let bcp1979 = summarize_calendar(
            language,
            &BCP1979_CALENDAR,
            BCP1979_CALENDAR.holy_days.iter().cloned(),
        );
        let lff2018 = summarize_calendar(
            language,
            &LFF2018_CALENDAR,
            LFF2018_CALENDAR.holy_days.iter().cloned(),
        );

        Some(CalendarPage {
            default_calendar,
            bcp1979,
            lff2018,
        })
    }

    fn head(&self, locale: &str) -> Vec<Node> {
        view! {
            <>
                <title>{t!("menu.calendar")} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/calendar.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        // Render BCP and LFF calendars and choose between them
        let bcp = calendar_view(CalendarChoice::BCP1979, locale, &self.bcp1979);
        let lff = calendar_view(CalendarChoice::LFF2018, locale, &self.lff2018);

        let initial_toggle_value = match self.default_calendar {
            CalendarChoice::BCP1979 => false,
            CalendarChoice::LFF2018 => true,
            CalendarChoice::None => preferences::is(
                &PreferenceKey::from(GlobalPref::Calendar),
                &PreferenceValue::Local("lff2018".into()),
            ),
        };

        let button = view! {
            <button id="modalOpen">
                <img src={Icon::Calendar} alt={t!("calendar.settings")}/>
            </button>
        };

        // Main view
        view! {
            <>
                {Header::new_with_additional_buttons(locale, &t!("menu.calendar"), vec![button]).to_node()}
                <main>
                    <CalendarController
                        lff={self.default_calendar == CalendarChoice::LFF2018}
                    >
                        <h2 slot="bcp-title">{t!("bcp_1979")}</h2>
                        <h2 slot="lff-title">{t!("lff_2018")}</h2>
                        <section slot="bcp-content">{bcp}</section>
                        <section slot="lff-content">{lff}</section>
                    </CalendarController>
                </main>
            </>
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, EnumString, Display)]
pub enum CalendarChoice {
    None,
    BCP1979,
    LFF2018,
}

impl Default for CalendarChoice {
    fn default() -> Self {
        Self::BCP1979
    }
}

type CalendarListing = Vec<(HolyDayId, Feast, String)>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CalendarPageProps {
    default_calendar: CalendarChoice,
    bcp1979: CalendarListing,
    lff2018: CalendarListing,
}

fn summarize_calendar(
    language: Language,
    calendar: &Calendar,
    holy_days: impl Iterator<Item = KalendarEntry>,
) -> CalendarListing {
    holy_days
        .filter_map(|(id, feast, time, _)| {
            if matches!(id, HolyDayId::Date(_, _)) {
                let rank = calendar.feast_day_rank(&feast);
                // include black-letter and red-letter days, but not weird Daily Office lectionary days like December 29
                // and don't include the Eve of ___ days
                if (rank == Rank::OptionalObservance || rank >= Rank::HolyDay)
                    && !matches!(time, Time::EveningOnly(_))
                {
                    let name = calendar.feast_name(feast, language);
                    Some((id, feast, name.map(String::from).unwrap_or_default()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

pub fn hydration_state(locale: &str, path: &str, _params: &()) -> Option<CalendarPageProps> {
    let language = Language::from_locale(locale);

    let default_calendar = if path.ends_with("lff2018") {
        CalendarChoice::LFF2018
    } else if path.ends_with("bcp1979") {
        CalendarChoice::BCP1979
    } else {
        CalendarChoice::None
    };
    let bcp1979 = summarize_calendar(
        language,
        &BCP1979_CALENDAR,
        BCP1979_CALENDAR.holy_days.iter().cloned(),
    );
    let lff2018 = summarize_calendar(
        language,
        &LFF2018_CALENDAR,
        LFF2018_CALENDAR.holy_days.iter().cloned(),
    );

    Some(CalendarPageProps {
        default_calendar,
        bcp1979,
        lff2018,
    })
}

const MONTHS: [(u8, u8); 12] = [
    (1, 31),
    (2, 28),
    (3, 31),
    (4, 30),
    (5, 31),
    (6, 30),
    (7, 31),
    (8, 31),
    (9, 30),
    (10, 31),
    (11, 30),
    (12, 31),
];

pub fn root_id(use_lff: bool) -> &'static str {
    if use_lff {
        "lff"
    } else {
        "bcp"
    }
}

fn calendar_view(calendar: CalendarChoice, locale: &str, listing: &CalendarListing) -> Vec<Node> {
    let language = Language::from_locale(locale);
    let root_id = root_id(calendar == CalendarChoice::LFF2018);

    MONTHS
        .iter()
        .flat_map(move |(month, days)| {
            let name = language.month_name(*month);
            // TODO yuck
            let bcp = listing.clone();

            let rows = (1..=*days)
                .map(|day_of_month| {
                    let feast = bcp
                        .iter()
                        .find(|(id, _, _)| *id == HolyDayId::Date(*month, day_of_month))
                        .map(|(_, feast, name)| (feast, name.clone()));
                    let link = feast
                        .and_then(|(feast, name)| {
                            serde_json::to_string(&feast).ok().map(|feast| {
                                let link =
                                    format!("/{}/holy-day/{}", locale, feast.replace('"', ""));
                                view! {
                                    <a href={link}>{name}</a>
                                }
                            })
                        })
                        .unwrap_or_default();
                    let id = format!("{}-{}-{}", root_id, month, day_of_month);
                    view! {
                        <tr id={id}>
                            <td>{day_of_month.to_string()}</td>
                            <td>{link}</td>
                        </tr>
                    }
                })
                .collect::<Vec<_>>();

            let id = format!("{}-{}", root_id, month);

            view! {
                <>
                    <h3>{name}</h3>
                    <table id={id}>{rows}</table>
                </>
            }
        })
        .collect()
}
