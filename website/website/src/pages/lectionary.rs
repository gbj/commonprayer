use crate::{
    components::{header, DocumentController},
    utils::language::locale_to_language,
};
use chrono::{Datelike, Local};
use episcopal_api::{
    calendar::{Date, LiturgicalDay, LiturgicalDayId, Rank, Weekday, BCP1979_CALENDAR},
    lectionary::Reading,
    library::summary,
    liturgy::{BiblicalCitation, Document, DocumentError, Version},
};
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
        .static_page()
}

#[derive(Deserialize, Clone)]
pub struct LectionaryPageParams {
    year: Option<u16>,
    month: Option<u8>,
}

#[derive(Serialize, Clone, Default)]
pub struct LectionaryPageRenderState {
    year: u16,
    starting_month: Option<u8>,
    days: Vec<LectionaryDayEntry>,
}

#[derive(Serialize, Clone, Default)]
pub struct LectionaryDayEntry {
    month: u8,
    day: u8,
    listing: Option<(String, LiturgicalDay)>,
}

#[derive(Serialize, Clone)]
pub struct DayDetails {
    day: LiturgicalDay,
    name: String,
    collect: Document,
    readings: Vec<Reading>,
}

pub fn build_paths() -> Vec<String> {
    vec!["".into(), "{year}".into(), "{year}/{month}".into()]
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
                let marked_on_calendar = if rank >= Rank::HolyDay {
                    let localized_day_name = summary::localize_day_name(
                        &liturgical_day,
                        &liturgical_day.observed,
                        &BCP1979_CALENDAR,
                        locale_to_language(locale),
                    );
                    Some((localized_day_name, liturgical_day))
                } else {
                    None
                };
                Some(LectionaryDayEntry {
                    month: current_date.month(),
                    day: current_date.day(),
                    listing: marked_on_calendar,
                })
            } else {
                None
            }
        })
        .collect();
    Some(LectionaryPageRenderState {
        year,
        starting_month: params.month,
        days,
    })
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
    calendar_body(
        locale,
        render_state.year,
        &render_state.starting_month,
        &render_state.days,
    )
}

fn calendar_body(
    locale: &str,
    year: u16,
    starting_month: &Option<u8>,
    days: &[LectionaryDayEntry],
) -> View {
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
                        .map(|LectionaryDayEntry { day, listing, .. }| {
                            let listing = if let Some((day_name, liturgical_day)) = listing {
                                let transferred = if matches!(
                                    liturgical_day.observed,
                                    LiturgicalDayId::TransferredFeast(_)
                                ) {
                                    View::StaticText(t!("daily_readings.transferred"))
                                } else {
                                    View::Empty
                                };

                                view! {
                                    <>
                                        <a href={format!("/{}/readings/lectionary/{}-{}-{}", locale, year, month, day)}>{day_name}</a>
                                        {transferred}
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
