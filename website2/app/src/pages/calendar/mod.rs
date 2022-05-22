mod controller;
pub use controller::CalendarController;

use crate::{
    preferences,
    utils::language::locale_to_language,
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

pub fn calendar() -> Page<CalendarPageProps, ()> {
    Page::new("calendar")
        .head_fn(head)
        .body_fn(body)
        .state(hydration_state)
        .build_paths_fn(get_static_paths)
        .incremental_generation()
}

pub fn head(_locale: &str, _props: &CalendarPageProps) -> Vec<Node> {
    view! {
        <>
            <title>{t!("menu.calendar")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/vars.css"/>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/calendar.css"/>
        </>
    }
}

pub fn get_static_paths() -> Vec<String> {
    vec!["".into(), "bcp1979".into(), "lff2018".into()]
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
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
    let language = locale_to_language(locale);

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

pub fn body(locale: &str, props: &CalendarPageProps) -> Vec<Node> {
    // Render BCP and LFF calendars and choose between them
    let bcp = calendar_view(CalendarChoice::BCP1979, locale, &props.bcp1979);
    let lff = calendar_view(CalendarChoice::LFF2018, locale, &props.lff2018);

    let initial_toggle_value = match props.default_calendar {
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
                    lff={props.default_calendar == CalendarChoice::LFF2018}
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

pub fn root_id(use_lff: bool) -> &'static str {
    if use_lff {
        "lff"
    } else {
        "bcp"
    }
}

fn calendar_view(calendar: CalendarChoice, locale: &str, listing: &CalendarListing) -> Vec<Node> {
    let language = locale_to_language(locale);
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
