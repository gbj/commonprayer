use chrono::{Datelike, Local};
use itertools::Itertools;
use leptos2::*;
use serde::Deserialize;
use {
    calendar::{Date, Feast, LiturgicalDay, LiturgicalDayId, Rank, Weekday, BCP1979_CALENDAR},
    library::summary,
};
use crate::views::Header;

use crate::utils::{
    language::locale_to_language, scroll_to_element_by_id_with_padding_for_header, time::today,
};

#[derive(Deserialize)]
pub struct LectionaryPageParams {
    year: Option<u16>,
}

pub struct LectionaryPage {
    year: u16,
    days: Vec<LectionaryDayEntry>,
}
pub struct LectionaryDayEntry {
    month: u8,
    day: u8,
    listing: Option<(String, LiturgicalDay)>,
    alternatives: Vec<(String, Feast)>,
    other_notes: Vec<String>,
}

impl Page for LectionaryPage {
    type Params = LectionaryPageParams;

    fn name() -> &'static str {
        "lectionary"
    }

    fn paths() -> Vec<String> {
        vec!["".into(), "{year}".into()]
    }

    fn build_state(locale: &str, _path: &str, params: Self::Params) -> Option<Self> {
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

                    let other_notes = liturgical_day
                        .holy_days
                        .iter()
                        .filter(|feast| BCP1979_CALENDAR.feast_day_rank(feast) == Rank::EmberDay)
                        .map(|feast| {
                            summary::localize_day_name(
                                &liturgical_day,
                                &LiturgicalDayId::Feast(*feast),
                                &BCP1979_CALENDAR,
                                language,
                            )
                        })
                        .collect();

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
                        other_notes,
                    })
                } else {
                    None
                }
            })
            .collect();
        Some(LectionaryPage { year, days })
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        view! {
            <>
                <title>{t!("menu.lectionary")} " – " {t!("common_prayer")}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/lectionary.css"/>
                <link rel="stylesheet" href="/static/document.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        let grouped_by_month = self.days
        .iter()
        .group_by(|LectionaryDayEntry { month, .. }| month);

    let months = grouped_by_month
            .into_iter()
            .flat_map(|(month, group)| {
                // calendar days
                let days = group
                        .into_iter()
                        .map(|LectionaryDayEntry { day, listing, alternatives, other_notes, .. }| {
                            let listing = if let Some((day_name, liturgical_day)) = listing {
                                let transferred = if matches!(
                                    liturgical_day.observed,
                                    LiturgicalDayId::TransferredFeast(_)
                                ) {
                                    Some(text(t!("daily_readings.transferred")))
                                } else {
                                   None
                                };

                                let alternatives = if alternatives.is_empty() {
                                   vec![]
                                } else {
									alternatives
										.iter()
										.map(|(name, feast)| view! {
											<a 
												class="alternative" 
												href={format!("/{}/readings/lectionary/{}-{}-{}/{:?}", locale, self.year, month, day, feast)}
											>
												{name}
											</a>
										})
										.collect::<Vec<_>>()
                                };

								let mut frags = Vec::new();
								frags.push(view! { <a href={format!("/{}/readings/lectionary/{}-{}-{}", locale, self.year, month, day)}>{day_name}</a> });
								if let Some(transferred) = transferred {
									frags.push(transferred);
								}
								frags.extend(alternatives);
                                frags
                            } else {
                                vec![]
                            };

                            let date = Date::from_ymd(self.year, *month, *day);
                            let class = if date.weekday() == Weekday::Sun {
                                "day sunday"
                            } else {
                                "day"
                            };

                            let other_notes = if other_notes.is_empty() {
                                None
                            } else {
                                let others = other_notes.iter().map(|s| view! { <li>{s}</li>} ).collect::<Vec<_>>();
                                Some(view! {
                                    <ul class="other-notes">{others}</ul>
                                })
                            };

                            view! {
                                <div class={class}>
                                    <a id={format!("{}/{}", month, day)}></a>
                                    <div class="month-number">{day.to_string()}</div>
                                    {listing}
                                    {other_notes}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>();

                // padding so that day #1 falls on the correct column for its day of week
                let padding_days = Date::from_ymd(self.year, *month, 1)
                    .weekday()
                    .num_days_from_sunday();
                let padding = (1..=padding_days)
                        .map(|_| view! { <div class="padding"></div> })
                        .collect::<Vec<_>>();

                view! {
                    <>
                        <a id={month}></a>
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
            .collect::<Vec<_>>();

	view! {
		<>
			{Header::new(locale, &t!("menu.lectionary")).to_node()}
			<main class="lectionary calendar">
				{months}
			</main>
		</>
	}
    }

    fn on_load() {
        if location_hash().unwrap_or_default().trim().is_empty() {
            let date = today();
            let hash_result = location().set_hash(&format!("{}/{}", date.month(), date.day()));
            if let Err(e) = hash_result {
                leptos2::debug_warn(&format!(
                    "[error in LectionaryPage::on_load when calling location.setHash()]\n\n{:#?}",
                    e
                ));
            }
        }

        window_event_listener("hashchange", |_| {
            if let Some(mmdd) = location_hash() {
                scroll_to_element_by_id_with_padding_for_header(&mmdd);
            }
        });
    }
}
