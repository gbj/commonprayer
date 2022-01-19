use crate::{
    components::*,
    utils::{language::locale_to_language, time::today},
};
use episcopal_api::{
    calendar::{
        feasts::KalendarEntry, Calendar, Date, Feast, HolyDayId, Rank, Time, BCP1979_CALENDAR,
        LFF2018_CALENDAR,
    },
    language::Language,
};
use futures::StreamExt;
use leptos::LiftStreamExt;
use leptos::*;
use rust_i18n::t;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::ScrollToOptions;

pub fn calendar() -> Page<CalendarPageProps, ()> {
    Page::new("calendar")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(get_static_props)
        .build_paths_fn(get_static_paths)
}

pub fn head(_locale: &str, _props: &CalendarPageProps) -> View {
    view! {
        <>
            <title>{t!("menu.calendar")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/calendar.css"/>
        </>
    }
}

pub fn get_static_paths() -> Vec<String> {
    vec!["".into(), "bcp1979".into(), "lff2018".into()]
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
enum CalendarChoice {
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
                    && time != Time::EveningOnly
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

pub fn get_static_props(locale: &str, path: &str, _params: ()) -> CalendarPageProps {
    let language = locale_to_language(locale);

    let default_calendar = if path.ends_with("lff2018") {
        CalendarChoice::LFF2018
    } else {
        CalendarChoice::BCP1979
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

    CalendarPageProps {
        default_calendar,
        bcp1979,
        lff2018,
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

pub fn body(locale: &str, props: &CalendarPageProps) -> View {
    // Render BCP and LFF calendars and choose between them
    let bcp = calendar_view(CalendarChoice::BCP1979, locale, &props.bcp1979);
    let lff = calendar_view(CalendarChoice::LFF2018, locale, &props.lff2018);

    let use_lff_toggle = Toggle::new(
        props.default_calendar == CalendarChoice::LFF2018,
        "calendar",
        t!("bcp_1979"),
        t!("lff_2018"),
        None,
    );

    let initial_date = if is_server!() { None } else { location_hash() }.and_then(|hash| {
        let today = today();
        let year = today.year();
        Date::parse_from_str(&format!("{}-{}", year, hash), "%Y-%m-%d")
            .ok()
            .or(Some(today))
    });
    let date_picker = DatePicker::new(t!("date"), initial_date);

    // auto-scroll either to current day or to the day selected in the date picker
    if !is_server!() {
        log("setting async handlers");

        let mut date = date_picker.date.stream();
        spawn_local(async move {
            while let Some(date) = date.next().await {
                if let Some(date) = date {
                    location()
                        .set_hash(&format!("{}-{}", date.month(), date.day()))
                        .unwrap_throw();
                }
            }
        });

        let hashchange = event_stream(window().unchecked_ref(), "hashchange");
        let calendar_choice = use_lff_toggle.toggled.stream().map(|toggled| {
            if toggled {
                CalendarChoice::LFF2018
            } else {
                CalendarChoice::BCP1979
            }
        });
        let mut lifted = (hashchange, calendar_choice).lift();

        spawn_local({
            let default_calendar = props.default_calendar;
            async move {
                while let Some((_, calendar_choice)) = lifted.next().await {
                    // use requestAnimationFrame here so that the attempt to scroll happens on the next tick
                    // otherwise, if this is firing because we've switched calendars,
                    // the other calendar will not yet be visible so the coordinates will be off
                    request_animation_frame(move || {
                        scroll_to_row(
                            calendar_choice.unwrap_or(default_calendar),
                            &location_hash().unwrap_or_default(),
                        )
                    });
                }
            }
        });
    }

    let bcp_class = use_lff_toggle
        .toggled
        .stream()
        .map(|using_lff| {
            if using_lff {
                String::from("hidden")
            } else {
                String::from("visible")
            }
        })
        .boxed_local();

    let lff_class = use_lff_toggle
        .toggled
        .stream()
        .map(|using_lff| {
            if using_lff {
                String::from("visible")
            } else {
                String::from("hidden")
            }
        })
        .boxed_local();

    // Main view
    view! {
        <>
            {header(locale, &t!("menu.calendar"))}
            <main>
                <section class="selection-menu">
                    <dyn:view view={use_lff_toggle.view()} />
                    <dyn:view view={date_picker.view()} />
                </section>
                <dyn:section
                    class={if props.default_calendar == CalendarChoice::BCP1979 { "visible" } else { "hidden" }}
                    class={bcp_class}
                >
                    {bcp}
                </dyn:section>
                <dyn:section
                    class={if props.default_calendar == CalendarChoice::LFF2018 { "visible" } else { "hidden" }}
                    class={lff_class}
                >
                    {lff}
                </dyn:section>
            </main>
        </>
    }
}

fn scroll_to_row(calendar: CalendarChoice, hash: &str) {
    let root_id = root_id(calendar);
    let el = document().get_element_by_id(&format!("{}-{}", root_id, hash));
    if let Some(el) = el {
        // scroll into view, with some padding at the top for the menu
        // uses scroll_by rather than scroll_to because the DomRect is apparently relative to the current position
        let rect = el.get_bounding_client_rect();
        window().scroll_by_with_scroll_to_options(ScrollToOptions::new().top(rect.y() - 150.0));
    }
}

fn root_id(calendar: CalendarChoice) -> &'static str {
    match calendar {
        CalendarChoice::BCP1979 => "bcp",
        CalendarChoice::LFF2018 => "lff",
    }
}

fn calendar_view(calendar: CalendarChoice, locale: &str, listing: &CalendarListing) -> View {
    let language = locale_to_language(locale);
    let root_id = root_id(calendar);

    View::Fragment(
        MONTHS
            .iter()
            .map(move |(month, days)| {
                let name = language.month_name(*month);
                // TODO yuck
                let bcp = listing.clone();

                let rows = View::Fragment(
                    (1..=*days)
                        .map(|day_of_month| {
                            let feast = bcp
                                .iter()
                                .find(|(id, _, _)| *id == HolyDayId::Date(*month, day_of_month))
                                .map(|(_, feast, name)| (feast, name.clone()));
                            let link = feast
                                .and_then(|(feast, name)| {
                                    serde_json::to_string(&feast).ok().map(|feast| {
                                        let link = format!(
                                            "/{}/holy-day/{}",
                                            locale,
                                            feast.replace('"', "")
                                        );
                                        view! {
                                            <a href={link}>{name}</a>
                                        }
                                    })
                                })
                                .unwrap_or(View::Empty);
                            let id = format!("{}-{}-{}", root_id, month, day_of_month);
                            view! {
                                <tr id={id}>
                                    <td>{day_of_month.to_string()}</td>
                                    <td>{link}</td>
                                </tr>
                            }
                        })
                        .collect(),
                );

                let id = format!("{}-{}", root_id, month);

                view! {
                    <>
                        <h2>{name}</h2>
                        <table id={id}>{rows}</table>
                    </>
                }
            })
            .collect(),
    )
}
