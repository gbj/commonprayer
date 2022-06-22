use calendar::{
    Date, Feast, LiturgicalDay, LiturgicalDayId, Rank, BCP1979_CALENDAR, LFF2018_CALENDAR, Weekday,
};
use itertools::Itertools;
use language::Language;
use leptos2::*;
use library::summary;

use crate::utils::time::today;
pub struct CalendarView {
	locale: String,
    year: u16,
    month: u8,
    days: Vec<CalendarDayEntry>,
    using_lff: bool,
    show_black_letter: bool,
}

pub struct CalendarDayEntry {
    month: u8,
    day: u8,
    black_letter_days: Vec<(Feast, String)>,
    listing: Option<(String, LiturgicalDay)>,
    alternatives: Vec<(String, Feast)>,
    other_notes: Vec<String>,
}

#[derive(Params)]
pub struct CalendarDayQuery {
    year: Option<u16>,
    month: Option<u8>,
    calendar: Option<String>,
    blackletter: Option<String>,
}

#[async_trait(?Send)]
impl Loader for CalendarView {
    type Params = ();
    type Query = CalendarDayQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let today = today();
        let year = query.year.unwrap_or_else(|| today.year());
        let month = query.month.unwrap_or_else(|| today.month());

        let day_1 = Date::from_ymd(year, month, 1);

        let using_lff = query
            .calendar
            .map(|calendar| calendar == "lff" || calendar == "lff2018")
            .unwrap_or(false);
        let show_black_letter = query.blackletter.map(|v| v != "false").unwrap_or(true);

        let calendar = if using_lff {
            LFF2018_CALENDAR
        } else {
            BCP1979_CALENDAR
        };

        let language = Language::from_locale(locale);

        let days = (0..=31)
            .filter_map(|offset| {
                let current_date = day_1.add_days(offset);
                if current_date.year() == year {
                    let liturgical_day = calendar.liturgical_day(current_date, false);
                    let rank = calendar.rank(&liturgical_day);

                    let other_notes = liturgical_day
                        .holy_days
                        .iter()
                        .filter(|feast| calendar.feast_day_rank(feast) == Rank::EmberDay)
                        .map(|feast| {
                            summary::localize_day_name(
                                &liturgical_day,
                                &LiturgicalDayId::Feast(*feast),
                                &calendar,
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
                                    &calendar,
                                    language,
                                ),
                                *feast,
                            )
                        })
                        .collect();

                    let black_letter_days = liturgical_day.holy_days.iter().filter(|feast| show_black_letter && calendar.feast_day_rank(feast) == Rank::OptionalObservance).map(|feast| (*feast, calendar.feast_name(*feast, language).unwrap_or_else(|| feast.to_string()))).collect();

                    let marked_on_calendar = if rank >= Rank::HolyDay {
                        let localized_day_name = summary::localize_day_name(
                            &liturgical_day,
                            &liturgical_day.observed,
                            &calendar,
                            language,
                        );
                        Some((localized_day_name, liturgical_day))
                    } else {
                        None
                    };

                    Some(CalendarDayEntry {
                        month: current_date.month(),
                        day: current_date.day(),
                        black_letter_days,
                        listing: marked_on_calendar,
                        alternatives,
                        other_notes,
                    })
                } else {
                    None
                }
            })
            .collect();

        Some(Self {
			locale: locale.to_string(),
            year,
            month,
            days,
            using_lff,
            show_black_letter,
        })
    }
}

impl View for CalendarView {
    fn title(&self) -> String {
        format!(
            "{} {} – {}",
            t!(&format!("lectionary.month_{}", self.month)),
            self.year,
            t!("common_prayer")
        )
    }

    fn styles(&self) -> Styles {
        vec![include_str!("calendar.css").into()]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        // calendar days
		let days = self.days
				.into_iter()
				.map(|CalendarDayEntry { day, listing, alternatives, other_notes, black_letter_days, month, .. }| {
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
											href={format!("/{}/readings/lectionary/{}-{}-{}/{:?}", self.locale, self.year, self.month, day, feast)}
										>
											{name}
										</a>
									})
									.collect()
						};

						Some(view! {
							<div>
								<a href={format!("/{}/readings/lectionary/{}-{}-{}", self.locale, self.year, self.month, day)}>{day_name}</a>
								{transferred}
								{alternatives}
							</div>
						})
					} else {
						None
					};

					let date = Date::from_ymd(self.year, self.month, day);
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

                    let black_letter_days = if black_letter_days.is_empty() {
                        None
                    } else {
                        let days = black_letter_days.iter().map(|(feast, name)| {
                            let href = format!("/{}/readings/holy-day/?date={}-{}-{}&id={}", self.locale, self.year, month, day, feast);
                            view! {
                                <li><a href={href}>{name}</a></li>
                            }
                        }).collect::<Vec<_>>();
                        Some(view! {
                            <ul class="black-letter-days">{days}</ul>
                        })
                    };

					view! {
						<time datetime={format!("{}-{:02}-{:02}", self.year, self.month, day)} class={class}>
							<a id={format!("{}/{}", self.month, day)}></a>
							<div class="month-number">{day.to_string()}</div>
							{listing}
                            {black_letter_days}
                            {other_notes}
						</time>
					}
				});

		// padding so that day #1 falls on the correct column for its day of week
		let padding_days = Date::from_ymd(self.year, self.month, 1)
			.weekday()
			.num_days_from_sunday();
		let padding = (1..=padding_days)
				.map(|_| view! { <div class="padding"></div> });;

        let weeks = padding.chain(days).chunks(7).into_iter().map(|chunk| {
            view! {
                <div class="week">{chunk.collect::<Vec<_>>()}</div>
            }
        }).collect::<Vec<_>>();

		view! {
			<div>
				<header><h1>{t!("menu.calendar")}</h1></header>
				<main>
					<h2>{t!(&format!("lectionary.month_{}", self.month))}</h2>
					<time class="month" datetime={format!("{}-{:02}", self.year, self.month)}>
                        <div class="weekday-labels">
                            <div class="weekday-label">{t!("canticle_table.sunday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.monday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.tuesday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.wednesday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.thursday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.friday_abbrev")}</div>
                            <div class="weekday-label">{t!("canticle_table.saturday_abbrev")}</div>
                        </div>
						{weeks}
					</time>
				</main>
			</div>
		}
    }
}
