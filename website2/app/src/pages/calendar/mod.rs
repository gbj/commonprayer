mod controller;

pub use controller::CalendarController;

use crate::views::{Header, Icon};
use calendar::{
    feasts::KalendarEntry, Calendar, Feast, HolyDayId, Rank, Time, BCP1979_CALENDAR,
    LFF2018_CALENDAR,
};
use language::Language;
use leptos2::*;
use rust_i18n::t;

pub struct CalendarPage {
    lff: bool,
    data: CalendarListing,
}

type CalendarListing = Vec<(HolyDayId, Feast, String)>;

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

impl Page for CalendarPage {
    type Params = ();
    type Query = ();

    fn name() -> &'static str {
        "calendar"
    }

    fn paths() -> Vec<String> {
        vec!["".into(), "lff2018".into()]
    }

    fn build_state(
        locale: &str,
        path: &str,
        _params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        let language = Language::from_locale(locale);

        let lff = path.ends_with("lff2018");
        let data = if lff {
            summarize_calendar(
                language,
                &LFF2018_CALENDAR,
                LFF2018_CALENDAR.holy_days.iter().cloned(),
            )
        } else {
            summarize_calendar(
                language,
                &BCP1979_CALENDAR,
                BCP1979_CALENDAR.holy_days.iter().cloned(),
            )
        };

        Some(CalendarPage { lff, data })
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        view! {
            <>
                <title>{t!("menu.calendar")} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/calendar.css"/>
                <link rel="stylesheet" href="/static/toggle-links.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        let button = view! {
            <button id="modalOpen">
                <img src={Icon::Calendar} alt={t!("calendar.settings")}/>
            </button>
        };

        // Main view
        view! {
            <>
                {Header::new_with_additional_buttons(locale, &t!("menu.calendar"), vec![button]).to_node()}
                <CalendarController
                    locale={locale}
                    prop:lff={self.lff}
                />
                <main>
                    {calendar_toggle_links(locale, self.lff)}

                    {if self.lff {
                        view! {  <h2>{t!("lff_2018")}</h2> }
                    } else {
                        view! { <h2>{t!("bcp_1979")}</h2> }
                    }}

                    {self.calendar_view(locale)}
                </main>
            </>
        }
    }
}

pub fn calendar_toggle_links(locale: &str, use_lff: bool) -> Node {
    view! {
        <div class="toggle-links">
            <a href={format!("/{}/calendar", locale)} class:current={!use_lff}>{{t!("bcp_1979")}}</a>
            <a href={format!("/{}/calendar/lff2018", locale)} class:current={use_lff}>{{t!("lff_2018")}}</a>
        </div>
    }
}

impl CalendarPage {
    fn calendar_view(&self, locale: &str) -> Vec<Node> {
        let language = Language::from_locale(locale);

        MONTHS
            .iter()
            .flat_map(move |(month, days)| {
                let name = language.month_name(*month);

                let rows = (1..=*days)
                    .map(|day_of_month| {
                        let feast = self
                            .data
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
                        let id = format!("{}-{}", month, day_of_month);
                        view! {
                            <tr id={id}>
                                <td>{day_of_month.to_string()}</td>
                                <td>{link}</td>
                            </tr>
                        }
                    })
                    .collect::<Vec<_>>();

                view! {
                    <>
                        <h3>{name}</h3>
                        <table id={month}>{rows}</table>
                    </>
                }
            })
            .collect()
    }
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
