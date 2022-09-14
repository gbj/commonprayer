use calendar::{Calendar, Date, Feast, LiturgicalDay, LiturgicalDayId, Rank, Weekday};
use itertools::Itertools;
use language::Language;
use leptos::*;
use library::summary;

use crate::i18n::{use_i18n, use_locale};
use crate::icon::Icon;
use crate::modal::*;
use crate::time::{get_timezone_offset, today};

#[derive(Params, Debug, Clone, PartialEq, Eq, Default)]
pub struct CalendarDayQuery {
    year: Option<u16>,
    month: Option<u8>,
    calendar: Option<String>,
    blackletter: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarData {
    year: u16,
    month: u8,
    days: Vec<CalendarDayEntry>,
    using_lff: bool,
    show_black_letter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarDayEntry {
    month: u8,
    day: u8,
    black_letter_days: Vec<(Feast, String)>,
    listing: Option<(String, LiturgicalDay)>,
    alternatives: Vec<(String, Feast)>,
    other_notes: Vec<(Feast, String)>,
}

// TODO move to server
pub fn calendar_data(cx: Scope, params: Memo<ParamsMap>, location: Location) -> Memo<CalendarData> {
    use calendar::{BCP1979_CALENDAR, LFF2018_CALENDAR};

    create_memo(cx, move |_| {
        log::debug!("(calendar_data) running loader");

        let locale = use_locale(cx).get();
        let query = use_query::<CalendarDayQuery>(cx).get().unwrap_or_default();
        let (t, _) = use_i18n(cx);

        let tz = get_timezone_offset(cx);
        let today = today(&tz);
        let year = query.year.unwrap_or_else(|| today.year());
        let month = query.month.unwrap_or_else(|| today.month());
        let day_1 = Date::from_ymd(year, month, 1);
        let using_lff = query
            .calendar
            .map(|calendar| calendar != "bcp")
            .unwrap_or(true);
        let show_black_letter = query.blackletter.map(|v| v != "false").unwrap_or(true);

        let calendar = if using_lff {
            LFF2018_CALENDAR
        } else {
            BCP1979_CALENDAR
        };

        let language = Language::from_locale(&locale);

        let days = (0..=31)
            .filter_map(|offset| {
                let current_date = day_1.add_days(offset);
                if current_date.year() == year && current_date.month() == month {
                    let liturgical_day = calendar.liturgical_day(current_date, false);
                    let rank = calendar.rank(&liturgical_day);

                    let other_notes = liturgical_day
                        .holy_days
                        .iter()
                        .filter(|feast| calendar.feast_day_rank(feast) == Rank::EmberDay)
                        .map(|feast| {
                            (
                                *feast,
                                summary::localize_day_name(
                                    &liturgical_day,
                                    &LiturgicalDayId::Feast(*feast),
                                    &calendar,
                                    language,
                                ),
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

                    let black_letter_days = liturgical_day
                        .holy_days
                        .iter()
                        .filter(|feast| {
                            show_black_letter
                                && calendar.feast_day_rank(feast) == Rank::OptionalObservance
                        })
                        .map(|feast| {
                            (
                                *feast,
                                calendar
                                    .feast_name(*feast, language)
                                    .unwrap_or_else(|| feast.to_string()),
                            )
                        })
                        .collect();

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

        CalendarData {
            year,
            month,
            days,
            using_lff,
            show_black_letter,
        }
    })
}

#[component]
pub fn Calendar(cx: Scope) -> Element {
    let (t, _) = use_i18n(cx);
    let data = use_loader::<Memo<CalendarData>>(cx);
    let (settings_open, set_settings_open) = create_signal(cx, false);

    view! {
        <div>
            <header>
                <span></span>
                <h1>{t("menu.calendar")}</h1>
                <button on:click=move |_| set_settings_open(|n| *n = !*n)>
                    <img src={Icon::Settings.to_string()} alt={t("settings.title")}/>
                </button>
                <Modal
                    open=settings_open
                    on_close=move || set_settings_open(|n| *n = false)
                >
                    <Form>
                        <fieldset class="horizontal">
                            <label class="stacked">
                                {t("calendar.year")}
                                <input type="number" name="year" value={data.with(|d| d.year)}/>
                            </label>
                            <label class="stacked">
                                {t("calendar.month")}
                                <select name="month">
                                    <option value="1" selected={move || data.with(|d| d.month == 1)}>{t("lectionary.month_1")}</option>
                                    <option value="2" selected={move || data.with(|d| d.month == 2)}>{t("lectionary.month_2")}</option>
                                    <option value="3" selected={move || data.with(|d| d.month == 3)}>{t("lectionary.month_3")}</option>
                                    <option value="4" selected={move || data.with(|d| d.month == 4)}>{t("lectionary.month_4")}</option>
                                    <option value="5" selected={move || data.with(|d| d.month == 5)}>{t("lectionary.month_5")}</option>
                                    <option value="6" selected={move || data.with(|d| d.month == 6)}>{t("lectionary.month_6")}</option>
                                    <option value="7" selected={move || data.with(|d| d.month == 7)}>{t("lectionary.month_7")}</option>
                                    <option value="8" selected={move || data.with(|d| d.month == 8)}>{t("lectionary.month_8")}</option>
                                    <option value="9" selected={move || data.with(|d| d.month == 9)}>{t("lectionary.month_9")}</option>
                                    <option value="10" selected={move || data.with(|d| d.month == 10)}>{t("lectionary.month_10")}</option>
                                    <option value="11" selected={move || data.with(|d| d.month == 11)}>{t("lectionary.month_11")}</option>
                                    <option value="12" selected={move || data.with(|d| d.month == 12)}>{t("lectionary.month_12")}</option>
                                </select>
                            </label>
                        </fieldset>

                        // Black Letter Days
                        <fieldset class="horizontal">
                            <legend>{t("menu.calendar")}</legend>
                            <label class="horizontal">
                                {t("bcp_1979")}
                                <input type="radio" name="calendar" value="bcp" checked={move || !data.with(|d| d.using_lff)} />
                            </label>
                            <label class="horizontal">
                                {t("lff_2018")}
                                <input type="radio" name="calendar" value="lff" checked={move || data.with(|d| d.using_lff)} />
                            </label>
                        </fieldset>

                        // Black Letter Days
                        <label class="horizontal">
                            {t("calendar.omit_black_letter")}
                            <input type="checkbox" name="blackletter" value="false" checked={!data.with(|d| d.show_black_letter)} />
                        </label>

                        <input type="submit" slot="close-button" data-modal-close="settings" value={t("settings.submit")}/>
                    </Form>
                </Modal>
            </header>
            <main>
                <div class="controls">
                    <AdjacentMonth increase={false}/>
                    <h2>{t(&format!("lectionary.month_{}", data.with(|d| d.month)))}</h2>
                    <AdjacentMonth increase={true}/>
                </div>
                <time class="month" datetime={format!("{}-{:02}", data.with(|d| d.year), data.with(|d| d.month))}>
                    <div class="weekday-labels">
                        <div class="weekday-label">{t("canticle_table.sunday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.monday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.tuesday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.wednesday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.thursday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.friday_abbrev")}</div>
                        <div class="weekday-label">{t("canticle_table.saturday_abbrev")}</div>
                    </div>
                    <Weeks/>
                </time>
            </main>
        </div>
    }
}

#[component]
fn AdjacentMonth(cx: Scope, increase: bool) -> Element {
    let (t, _) = use_i18n(cx);
    let curr = use_loader::<Memo<CalendarData>>(cx);

    let year = move || {
        if curr.with(|c| c.month) == 1 && !increase {
            curr.with(|c| c.year) - 1
        } else if curr.with(|c| c.month) == 12 && increase {
            curr.with(|c| c.year) + 1
        } else {
            curr.with(|c| c.year)
        }
    };

    let month = move || {
        if curr.with(|c| c.month) == 1 && !increase {
            12
        } else if curr.with(|c| c.month) == 12 && increase {
            1
        } else if !increase {
            curr.with(|c| c.month) - 1
        } else {
            curr.with(|c| c.month) + 1
        }
    };

    let calendar = move || {
        if curr.with(|c| c.using_lff) {
            Some("lff2018".to_string())
        } else {
            None
        }
    };

    let blackletter = move || {
        if curr.with(|c| c.show_black_letter) {
            None
        } else {
            Some("false".to_string())
        }
    };

    let label = move || {
        if increase && curr.with(|c| c.month) == 12 {
            format!("{} {}", t("lectionary.month_1"), curr.with(|c| c.year) + 1)
        } else if increase {
            t(&format!("lectionary.month_{}", curr.with(|c| c.month) + 1))
        } else if curr.with(|c| c.month) == 1 {
            format!("{} {}", t("lectionary.month_12"), curr.with(|c| c.year) - 1)
        } else {
            t(&format!("lectionary.month_{}", curr.with(|c| c.month) - 1))
        }
    };

    view! {
        <Form method="GET".into()>
            <input type="hidden" name="year" value={year}/>
            <input type="hidden" name="month" value={month}/>
            <input type="hidden" name="calendar" value={calendar}/>
            <input type="hidden" name="blackletter" value={blackletter}/>
            <input type="submit" value={label}/>
        </Form>
    }
}

#[component]
fn Weeks(cx: Scope) -> impl IntoChild {
    let data = use_loader::<Memo<CalendarData>>(cx);
    move || {
        data.with(|data| {
            let padding_days = Date::from_ymd(data.year, data.month, 1)
                .weekday()
                .num_days_from_sunday();

            let padding = (1..=padding_days).map(|_| None);
            let days = data.days.iter().cloned().map(Some);
            padding
                .chain(days)
                .chunks(7)
                .into_iter()
                .map(|chunk| {
                    let week = chunk.collect::<Vec<_>>();
                    view! {
                        <Week year=data.year week />
                    }
                })
                .collect::<Vec<_>>()
        })
    }
}

#[component]
fn Week(cx: Scope, year: u16, week: Vec<Option<CalendarDayEntry>>) -> Element {
    let days = week
        .into_iter()
        .map(|day| match day {
            None => view! { <div class="padding"></div> },
            Some(day) => view! { <Day year day /> },
        })
        .collect::<Vec<_>>();

    view! { <div class="week">{days}</div>}
}

#[component]
fn Day(cx: Scope, year: u16, day: CalendarDayEntry) -> Element {
    let CalendarDayEntry {
        day,
        listing,
        alternatives,
        other_notes,
        black_letter_days,
        month,
        ..
    } = day;

    let date = Date::from_ymd(year, month, day);
    let class = if date.weekday() == Weekday::Sun {
        "day sunday"
    } else {
        "day"
    };

    view! {
        <time
            class={class}
            datetime=format!("{}-{:02}-{:02}", year, month, day)
        >
            <a id=format!("{}/{}", month, day)></a>
            <div class="month-number">{day}</div>
            <Listing
                year
                month
                day
                listing
                alternatives
            />
            <BlackLetterDays
                year
                month
                day
                black_letter_days
            />
            <OtherNotes
                year
                month
                day
                other_notes
            />
        </time>
    }
}

#[component]
fn Listing(
    cx: Scope,
    year: u16,
    month: u8,
    day: u8,
    listing: Option<(String, LiturgicalDay)>,
    alternatives: Vec<(String, Feast)>,
) -> Option<Element> {
    let locale = use_locale(cx);
    let (t, _) = use_i18n(cx);

    if let Some((day_name, liturgical_day)) = listing {
        let transferred = if matches!(
            liturgical_day.observed,
            LiturgicalDayId::TransferredFeast(_)
        ) {
            Some(
                view! {<span class="transferred">{format!(" {}", t("daily_readings.transferred"))}</span>},
            )
        } else {
            None
        };

        let alternatives = if alternatives.is_empty() {
            vec![]
        } else {
            alternatives
                .into_iter()
                .map(|(name, feast)| view! {
                    <a
                        class="alternative"
                        href={move || format!("/{}/readings/eucharist/?date={}-{}-{}&alternate={}", locale.get(), year, month, day, feast)}
                    >
                        {name}
                    </a>
                })
                .collect()
        };

        Some(view! {
            <div class="main-listing">
                <a class="day-name" href={move || format!("/{}/readings/eucharist/?date={}-{}-{}", locale.get(), year, month, day)}>{day_name}</a>
                {transferred}
                {alternatives}
            </div>
        })
    } else {
        None
    }
}

#[component]
fn BlackLetterDays(
    cx: Scope,
    year: u16,
    month: u8,
    day: u8,
    black_letter_days: Vec<(Feast, String)>,
) -> Option<Element> {
    let locale = use_locale(cx);
    if black_letter_days.is_empty() {
        None
    } else {
        let days = black_letter_days
            .into_iter()
            .map(|(feast, name)| {
                let href = move || {
                    format!(
                        "/{}/readings/holy-day/?date={}-{}-{}&id={}",
                        locale.get(),
                        year,
                        month,
                        day,
                        feast
                    )
                };
                view! {
                    <li><a href={href}>{name}</a></li>
                }
            })
            .collect::<Vec<_>>();
        Some(view! {
            <ul class="black-letter-days">{days}</ul>
        })
    }
}

#[component]
fn OtherNotes(
    cx: Scope,
    year: u16,
    month: u8,
    day: u8,
    other_notes: Vec<(Feast, String)>,
) -> Option<Element> {
    let locale = use_locale(cx);
    if other_notes.is_empty() {
        None
    } else {
        let others = other_notes
            .into_iter()
            .map(|(id, name)| {
                let href = move || {
                    format!(
                        "/{}/readings/holy-day/?date={}-{}-{}&id={}",
                        locale.get(),
                        year,
                        month,
                        day,
                        id
                    )
                };
                view! { <li><a href={href}>{name}</a></li> }
            })
            .collect::<Vec<_>>();
        Some(view! {
            <ul class="other-notes">{others}</ul>
        })
    }
}
