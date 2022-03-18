use crate::{components::header, utils::{language::locale_to_language, time::today, scroll_to_element_by_id_with_padding_for_header}};
use chrono::{Datelike, Local};
use {
    calendar::{Date, Feast, LiturgicalDay, LiturgicalDayId, Rank, Weekday, BCP1979_CALENDAR},
    lectionary::Reading,
    library::summary,
    liturgy::Document,
};
use futures::StreamExt;
use itertools::Itertools;
use leptos::*;
use serde::{Deserialize, Serialize};

pub fn lectionary() -> Page<(), LectionaryPageParams, LectionaryPageRenderState> {
    Page::new("lectionary")
        .head_fn(head)
        .body_fn(body)
        .build_paths_fn(build_paths)
        .hydration_state(|_, _, _| Some(()))
        .render_state(render_state)
        .incremental_generation()
}

#[derive(Deserialize, Clone)]
pub struct LectionaryPageParams {
    year: Option<u16>,
}

#[derive(Serialize, Clone, Default)]
pub struct LectionaryPageRenderState {
    year: u16,
    days: Vec<LectionaryDayEntry>,
}

#[derive(Serialize, Clone, Default)]
pub struct LectionaryDayEntry {
    month: u8,
    day: u8,
    listing: Option<(String, LiturgicalDay)>,
    alternatives: Vec<(String, Feast)>,
}

#[derive(Serialize, Clone)]
pub struct DayDetails {
    day: LiturgicalDay,
    name: String,
    collect: Document,
    readings: Vec<Reading>,
}

pub fn build_paths() -> Vec<String> {
    vec!["".into(), "{year}".into(), "{year}".into()]
}

pub fn render_state(
    locale: &str,
    _path: &str,
    params: &LectionaryPageParams,
) -> Option<LectionaryPageRenderState> {
    let year = params
        .year
        .unwrap_or_else(|| Local::now().date().year().try_into().unwrap());
    let january_1 = Date::from_ymd(year, 1, 1);
    let days = (0..=366)
        .filter_map(|offset| {
            let current_date = january_1.add_days(offset);
            if current_date.year() == year {
                let liturgical_day = BCP1979_CALENDAR.liturgical_day(current_date, false);
                let rank = BCP1979_CALENDAR.rank(&liturgical_day);
                let language = locale_to_language(locale);

                let alternatives = liturgical_day
                    .alternative_services
                    .iter()
                    .map(|feast| {
                        (
                            summary::localize_day_name(
                                &liturgical_day,
                                &LiturgicalDayId::Feast(*feast),
                                &BCP1979_CALENDAR,
                                language,
                            ),
                            *feast,
                        )
                    })
                    .collect();

                let marked_on_calendar = if rank >= Rank::HolyDay {
                    let localized_day_name = summary::localize_day_name(
                        &liturgical_day,
                        &liturgical_day.observed,
                        &BCP1979_CALENDAR,
                        language,
                    );
                    Some((localized_day_name, liturgical_day))
                } else {
                    None
                };

                Some(LectionaryDayEntry {
                    month: current_date.month(),
                    day: current_date.day(),
                    listing: marked_on_calendar,
                    alternatives,
                })
            } else {
                None
            }
        })
        .collect();
    Some(LectionaryPageRenderState { year, days })
}

pub fn head(_locale: &str, _props: &(), _render_state: &LectionaryPageRenderState) -> View {
    view! {
        <>
            <title>{t!("menu.lectionary")} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/lectionary.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
        </>
    }
}

pub fn body(locale: &str, _props: &(), render_state: &LectionaryPageRenderState) -> View {
    calendar_body(locale, render_state.year, &render_state.days)
}

fn calendar_body(locale: &str, year: u16, days: &[LectionaryDayEntry]) -> View {
    // on first load, scroll to today
    if !is_server!() {
        let date = today();
        let hash_result = location().set_hash(&format!("{}/{}", date.month(), date.day()));
        if let Err(e) = hash_result {
            warn(&format!("[error in calendar_body when calling location.setHash()]\n\n{:#?}", e));
        }
    }

    // listen for hash changes and scroll to month
    window_event_stream("hashchange").map(|_| ()).start_with(()).create_effect(|_| {
        if let Some(mmdd) = location_hash() {
            scroll_to_element_by_id_with_padding_for_header(&mmdd);
        }
    });


    // build calendar
    let grouped_by_month = days
        .iter()
        .group_by(|LectionaryDayEntry { month, .. }| month);
    let months = View::Fragment(
        grouped_by_month
            .into_iter()
            .map(|(month, group)| {
                // calendar days
                let days = View::Fragment(
                    group
                        .into_iter()
                        .map(|LectionaryDayEntry { day, listing, alternatives, .. }| {
                            let listing = if let Some((day_name, liturgical_day)) = listing {
                                let transferred = if matches!(
                                    liturgical_day.observed,
                                    LiturgicalDayId::TransferredFeast(_)
                                ) {
                                    View::StaticText(t!("daily_readings.transferred"))
                                } else {
                                    View::Empty
                                };

                                let alternatives = if alternatives.is_empty() {
                                    View::Empty
                                } else {
                                    View::Fragment(
                                        alternatives
                                            .iter()
                                            .map(|(name, feast)| view! {
                                                <a 
                                                    class="alternative" 
                                                    href={format!("/{}/readings/lectionary/{}-{}-{}/{:?}", locale, year, month, day, feast)}
                                                >
                                                    {name}
                                                </a>
                                            })
                                            .collect()
                                    )
                                };

                                view! {
                                    <>
                                        <a href={format!("/{}/readings/lectionary/{}-{}-{}", locale, year, month, day)}>{day_name}</a>
                                        {transferred}
                                        {alternatives}
                                    </>
                                }
                            } else {
                                View::Empty
                            };

                            let date = Date::from_ymd(year, *month, *day);
                            let class = if date.weekday() == Weekday::Sun {
                                "day sunday"
                            } else {
                                "day"
                            };

                            view! {
                                <div class={class}>
                                    <a id={format!("{}/{}", month, day)}></a>
                                    <div class="month-number">{day.to_string()}</div>
                                    {listing}
                                </div>
                            }
                        })
                        .collect(),
                );

                // padding so that day #1 falls on the correct column for its day of week
                let padding_days = Date::from_ymd(year, *month, 1)
                    .weekday()
                    .num_days_from_sunday();
                let padding = View::Fragment(
                    (1..=padding_days)
                        .map(|_| view! { <div class="padding"></div> })
                        .collect(),
                );

                view! {
                    <>
                        <h2>{t!(&format!("lectionary.month_{}", month))}</h2>
                        <div class="month">
                            <div class="weekday-label">{t!("canticle_table.sunday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.monday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.tuesday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.wednesday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.thursday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.friday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.saturday_abbrev")}</div>
                            {padding}
                            {days}
                        </div>
                    </>
                }
            })
            .collect(),
    );

    view! {
        <>
            {header(locale, &t!("menu.lectionary"))}
            <main class="lectionary calendar">
                {months}
            </main>
        </>
    }
}
