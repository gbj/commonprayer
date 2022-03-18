use std::rc::Rc;

use crate::utils::preferences::*;
use api::summary::{
    DailySummary, EucharisticLectionarySummary, EucharisticObservanceSummary, FirstLessonAndPsalm,
    ObservanceSummary, TrackedReadings,
};
use calendar::{Date, LiturgicalDayId};
use futures::{Stream, StreamExt};
use lectionary::Reading;
use leptos::*;
use library::CommonPrayer;
use liturgy::{
    BiblicalCitation, Content, Document, GlobalPref, Lectionaries, PreferenceKey, PreferenceValue,
    Psalm, Version,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::UnwrapThrowExt;

use crate::{
    components::*,
    preferences,
    utils::{language::locale_to_language, time::current_hour},
};

pub fn readings() -> Page<DailyReadingsPageProps, DailyReadingsUrlParams, ()> {
    Page::new("readings")
        .head_fn(head)
        .body_fn(body)
        .hydration_state(static_props)
        .build_paths_fn(build_paths_fn)
        .incremental_generation()
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum DailyReadingsPageProps {
    None,
    DailyOffice(Box<DailySummary>),
    Lectionary(Box<EucharisticLectionarySummary>),
}

#[derive(Deserialize, Clone)]
pub struct DailyReadingsUrlParams {
    date: Option<String>,
    alternate_readings: Option<String>,
}

fn head(locale: &str, props: &DailyReadingsPageProps, _render_state: &()) -> View {
    let title = format!(
        "{} – {}",
        if matches!(props, DailyReadingsPageProps::Lectionary(_)) {
            t!("menu.lectionary")
        } else {
            t!("toc.daily_readings")
        },
        t!("common_prayer")
    );

    // no summary means no date param is present in URL => redirect to client's current day
    let redirect = redirect_script(locale, "readings", props == &DailyReadingsPageProps::None);

    view! {
        <>
            <title>{title}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
            <link rel="stylesheet" href="/static/daily-readings.css"/>
            <link rel="stylesheet" href="/static/display-settings.css"/>
            <link rel="stylesheet" href="/static/settings.css"/>
            {redirect}
        </>
    }
}

fn static_props(
    locale: &str,
    path: &str,
    params: &DailyReadingsUrlParams,
) -> Option<DailyReadingsPageProps> {
    let language = locale_to_language(locale);
    let date = &params.date;
    if date.is_none() && !path.contains(&format!("{}/", RedirectMode::Eucharist)) {
        Some(DailyReadingsPageProps::None)
    } else if date.is_some() && path.contains(&format!("{}/", RedirectMode::Eucharist)) {
        date.as_ref().and_then(|date| {
            Date::parse_from_str(date, "%Y-%m-%d").ok().map(|date| {
                let alternate = params
                    .alternate_readings
                    .as_ref()
                    .and_then(|feast| serde_json::from_str(&format!(r#""{feast}""#)).ok());
                let summary =
                    CommonPrayer::eucharistic_lectionary_summary(&date, language, alternate);

                DailyReadingsPageProps::Lectionary(Box::new(summary))
            })
        })
    } else if let Some(date) = date {
        Date::parse_from_str(date, "%Y-%m-%d").ok().map(|date| {
            let summary = CommonPrayer::daily_office_summary(&date, language);

            DailyReadingsPageProps::DailyOffice(Box::new(summary))
        })
    } else {
        None
    }
}

fn build_paths_fn() -> Vec<String> {
    vec![
        format!("{}/{{date}}", RedirectMode::DailyOffice),
        format!("{}/{{date}}", RedirectMode::Eucharist),
        format!(
            "{}/{{date}}/{{alternate_readings}}",
            RedirectMode::Eucharist
        ),
        "{date}".into(),
        "".into(),
    ]
}

fn body(locale: &str, props: &DailyReadingsPageProps, _render_state: &()) -> View {
    match props {
        // JavaScript in header will redirect to today's Daily Office page
        DailyReadingsPageProps::None => view! {
            <>
                {header(locale, &t!("toc.daily_readings"))}
                <main>
                </main>
            </>
        },
        DailyReadingsPageProps::DailyOffice(summary) => office_body(locale, summary),
        DailyReadingsPageProps::Lectionary(summary) => eucharist_body(locale, summary),
    }
}

fn office_body(locale: &str, summary: &DailySummary) -> View {
    let controls = Controls::new(summary.clone());

    let primary_morning = observance_view(
        locale,
        current_hour() < 13,
        (
            controls.use_evening.toggled.stream(),
            controls.use_alternate_observance.toggled.stream(),
        )
            .lift()
            .map(|(evening, alternate)| {
                evening.map(|evening| !evening && !alternate.unwrap_or(false))
            }),
        controls.use_30_day_psalter.toggled.clone(),
        controls.use_lff_2018.toggled.clone(),
        &summary.morning.observed,
        &summary.morning.thirty_day_psalms,
    );

    let primary_evening = observance_view(
        locale,
        current_hour() >= 13,
        (
            controls.use_evening.toggled.stream(),
            controls.use_alternate_observance.toggled.stream(),
        )
            .lift()
            .map(|(evening, alternate)| {
                evening.map(|evening| evening && !alternate.unwrap_or(false))
            }),
        controls.use_30_day_psalter.toggled.clone(),
        controls.use_lff_2018.toggled.clone(),
        &summary.evening.observed,
        &summary.evening.thirty_day_psalms,
    );

    let alt_morning = observance_view(
        locale,
        false,
        (
            controls.use_evening.toggled.stream(),
            controls.use_alternate_observance.toggled.stream(),
        )
            .lift()
            .map(|(evening, alternate)| match (evening, alternate) {
                (Some(true), Some(true)) => Some(true),
                (None, Some(true)) => Some(true),
                _ => Some(false),
            }),
        controls.use_30_day_psalter.toggled.clone(),
        controls.use_lff_2018.toggled.clone(),
        summary
            .morning
            .alternate
            .as_ref()
            .unwrap_or(&summary.evening.observed),
        &summary.morning.thirty_day_psalms,
    );

    let alt_evening = observance_view(
        locale,
        false,
        (
            controls.use_evening.toggled.stream(),
            controls.use_alternate_observance.toggled.stream(),
        )
            .lift()
            .map(|(evening, alternate)| match (evening, alternate) {
                (Some(true), Some(true)) => Some(true),
                _ => Some(false),
            }),
        controls.use_30_day_psalter.toggled.clone(),
        controls.use_lff_2018.toggled.clone(),
        summary
            .evening
            .alternate
            .as_ref()
            .unwrap_or(&summary.evening.observed),
        &summary.evening.thirty_day_psalms,
    );

    let locale = locale.to_string();

    let primary_reading_links = reading_links(
        &summary.morning.observed,
        &summary.evening.observed,
        &summary.morning.thirty_day_psalms,
        &summary.evening.thirty_day_psalms,
    );

    let alternate_reading_links = reading_links(
        summary
            .morning
            .alternate
            .as_ref()
            .unwrap_or(&summary.morning.observed),
        summary
            .evening
            .alternate
            .as_ref()
            .unwrap_or(&summary.evening.observed),
        &summary.morning.thirty_day_psalms,
        &summary.evening.thirty_day_psalms,
    );

    let display_settings_menu = DisplaySettingsSideMenu::new();

    view! {
        <>
            {header_with_side_menu(&locale, &t!("toc.daily_readings"), display_settings_menu.view())}
            <dyn:main
                class={display_settings_menu.current_settings().stream().map(|settings| settings.to_class()).boxed_local()}
            >
                <label class="stacked">
                    {View::StaticText(t!("date"))}
                    <dyn:input type="date" class="centered"
                        value={summary.date.to_padded_string()}
                        on:change={
                            let locale = locale.clone();
                            move |ev: Event| redirect_to_date(&locale, event_target_value(ev), RedirectMode::DailyOffice)
                        }
                    />
                </label>

                // Controls to select morning/evening, psalm cycle, and alternate observance
                <dyn:view view={controls.view()}/>

                // Reading links
                <dyn:section
                    class="primary reading-link-table"
                    class:hidden={controls.use_alternate_observance.toggled.stream().boxed_local()}
                >
                    {reading_links_view(primary_reading_links, controls.use_30_day_psalter.toggled.clone())}
                </dyn:section>
                <dyn:section
                    class="alternate reading-link-table hidden "
                    class:hidden={controls.use_alternate_observance.toggled.stream().map(|using| !using).boxed_local()}
                >
                    {reading_links_view(alternate_reading_links, controls.use_30_day_psalter.toggled)}
                </dyn:section>

                // The psalms and readings themselves
                <dyn:view view={primary_morning}/>
                <dyn:view view={primary_evening}/>
                <dyn:view view={alt_morning}/>
                <dyn:view view={alt_evening}/>
            </dyn:main>
            {preference_status_footer(&display_settings_menu.status)}
        </>
    }
}

fn eucharist_body(locale: &str, summary: &EucharisticLectionarySummary) -> View {
    // TODO choose observance
    let observed = eucharistic_observance_view(locale, &summary.day.date, &summary.observed);

    let display_settings_menu = DisplaySettingsSideMenu::new();

    view! {
        <>
            {header_with_side_menu(locale, &t!("menu.lectionary"), display_settings_menu.view())}
            <dyn:main
                class={display_settings_menu.current_settings().stream().map(|settings| settings.to_class()).boxed_local()}
            >
            <label class="stacked">
                {View::StaticText(t!("date"))}
                <dyn:input type="date" class="centered"
                    value={summary.day.date.to_padded_string()}
                    on:change={
                        let locale = locale.to_string();
                        move |ev: Event| redirect_to_date(&locale, event_target_value(ev), RedirectMode::Eucharist)
                    }
                />
            </label>

            {observed}
            </dyn:main>
            {preference_status_footer(&display_settings_menu.status)}
        </>
    }
}

fn eucharistic_observance_view(
    locale: &str,
    date: &Date,
    summary: &EucharisticObservanceSummary,
) -> View {
    let bible_version = preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion))
        .and_then(|value| match value {
            PreferenceValue::Version(version) => Some(version),
            _ => None,
        })
        .unwrap_or(Version::NRSV);

    let title = title_view(locale, &summary.observance, &summary.localized_name);

    let collect_view = summary
        .collects
        .as_ref()
        .map(|collect| {
            view! {
                <h2>{t!("lookup.collect_of_the_day")}</h2>
                <dyn:view view={DocumentController::new(collect.clone()).view(locale)} />
            }
        })
        .unwrap_or(View::Empty);

    let track_choice_toggle = Toggle::new(
        false,
        "rcl-track",
        t!("lectionary.track_one"),
        t!("lectionary.track_two"),
        None,
    );

    let track_choice_toggle_view =
        if matches!(summary.tracked_readings, TrackedReadings::Tracked { .. }) {
            track_choice_toggle.view()
        } else {
            View::Empty
        };

    let tracked_readings_view = tracked_readings_view(
        locale,
        &summary.tracked_readings,
        track_choice_toggle.toggled,
        bible_version,
    );

    let vigil_readings_view = View::Fragment(
        summary
            .vigil_readings
            .iter()
            .map(|doc| {
                view! {
                    <article class="document">
                    {
                        DocumentController::new(doc.clone().version(
                            if matches!(doc.content, Content::Psalm(_)) {
                                doc.version
                            } else {
                                bible_version
                            },
                        ))
                        .view(locale)
                    }
                    </article>
                }
            })
            .collect(),
    );
    let vigil_readings_view = if summary.vigil_readings.is_empty() {
        View::Empty
    } else {
        view! {
            <>
                <a id="vigil"></a>
                <h3>{t!("lectionary.vigil")}</h3>
                {vigil_readings_view}
            </>
        }
    };

    let epistle = if let Some(epistle) = &summary.epistle {
        view! {
            <>
                <a id="epistle"></a>
                <h3>{t!("lectionary.epistle")}</h3>
                <article class="document">
                    <dyn:view view={DocumentController::new(epistle.clone().version(bible_version)).view(locale)}/>
                </article>
            </>
        }
    } else {
        View::Empty
    };

    let gospel = if let Some(gospel) = &summary.gospel {
        view! {
            <>
                <a id="gospel"></a>
                <h3>{t!("lectionary.gospel")}</h3>
                <article class="document">
                    <dyn:view view={DocumentController::new(gospel.clone().version(bible_version)).view(locale)}/>
                </article>
            </>
        }
    } else {
        View::Empty
    };

    let palms = if let Some(palms) = &summary.liturgy_of_the_palms {
        view! {
            <>
                <a id="palms"></a>
                <h3>{t!("lectionary.palms")}</h3>
                <article class="document">
                    <dyn:view view={DocumentController::new(palms.clone().version(bible_version)).view(locale)}/>
                </article>
            </>
        }
    } else {
        View::Empty
    };

    // not every day has readings assigned in The Lectionary: offer a choice to redirect
    // either to the Daily Office Lectionary or to The Lectionary
    let no_readings_link = if summary.epistle.is_none() && summary.gospel.is_none() {
        view! {
            <p class="redirect-links">
                {t!("lectionary.no_readings")}
                " "
                <a href={format!("/{}/readings/{}/{}", locale, RedirectMode::DailyOffice, date)}>
                    {t!("daily_readings.daily_office_readings")}
                </a>
                " "
                {t!("or")}
                " "
                <a href={format!("/{}/lectionary/{}#{}", locale, date.year(), date.month())}>
                    {t!("lectionary.the_lectionary")}
                </a>
                {t!("lectionary.no_readings_end")}
            </p>
        }
    } else {
        View::Empty
    };

    view! {
        <>
            {title}
            {collect_view}
            {palms}
            {vigil_readings_view}
            {track_choice_toggle_view}
            {tracked_readings_view}
            {epistle}
            {gospel}
            {no_readings_link}
        </>
    }
}

fn tracked_readings_view(
    locale: &str,
    tracked_readings: &TrackedReadings,
    use_track_2_if_available: Behavior<bool>,
    bible_version: Version,
) -> View {
    match tracked_readings {
        TrackedReadings::Any(readings) => {
            first_lesson_and_psalm_view(locale, readings, bible_version)
        }
        TrackedReadings::Tracked {
            track_one,
            track_two,
        } => {
            view! {
                <>
                    <dyn:section
                        class:hidden={use_track_2_if_available.stream().boxed_local()}
                    >
                        {first_lesson_and_psalm_view(locale, track_one, bible_version)}
                    </dyn:section>
                    <dyn:section
                        class="hidden"
                        class:hidden={use_track_2_if_available.stream().map(|use_track_2| !use_track_2).boxed_local()}
                    >
                        {first_lesson_and_psalm_view(locale, track_two, bible_version)}
                    </dyn:section>
                </>
            }
        }
    }
}

fn first_lesson_and_psalm_view(
    locale: &str,
    readings: &FirstLessonAndPsalm,
    bible_version: Version,
) -> View {
    let first_lesson = if let Some(first_lesson) = &readings.first_lesson {
        view! {
            <>
                <a id="first-lesson"></a>
                <h3>{t!("lectionary.first_lesson")}</h3>
                <article class="document">
                    <dyn:view view={DocumentController::new(first_lesson.clone().version(bible_version)).view(locale)}/>
                </article>
            </>
        }
    } else {
        View::Empty
    };

    let psalm_citation = readings
        .psalm
        .as_ref()
        .and_then(|psalm| {
            if let Content::Psalm(psalm) = &psalm.content {
                psalm.citation.clone()
            } else if let Content::Canticle(canticle) = &psalm.content {
                Some(t!(
                    "canticle_table.canticle_n",
                    n = &canticle.number.to_string()
                ))
            } else {
                None
            }
        })
        .unwrap_or_else(|| t!("lectionary.psalm"));
    let psalm = if let Some(psalm) = &readings.psalm {
        view! {
            <>
                <a id="psalm"></a>
                <h3>{psalm_citation}</h3>
                <article class="document">
                    <dyn:view view={DocumentController::new(psalm.clone()).view(locale)}/>
                </article>
            </>
        }
    } else {
        View::Empty
    };

    view! {
        <>
            {first_lesson}
            {psalm}
        </>
    }
}

enum RedirectMode {
    DailyOffice,
    Eucharist,
}

impl std::fmt::Display for RedirectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RedirectMode::DailyOffice => "office",
                RedirectMode::Eucharist => "lectionary",
            }
        )
    }
}

fn redirect_to_date(locale: &str, date: String, mode: RedirectMode) {
    location()
        .set_href(&format!("/{locale}/readings/{mode}/{date}"))
        .unwrap_throw();
}

struct ReadingLinks<'a> {
    morning_office_psalms: Vec<&'a str>,
    evening_office_psalms: Vec<&'a str>,
    morning_30_psalms: Vec<&'a str>,
    evening_30_psalms: Vec<&'a str>,
    morning_readings: Vec<&'a str>,
    evening_readings: Vec<&'a str>,
}

fn reading_links<'a>(
    morning: &'a ObservanceSummary,
    evening: &'a ObservanceSummary,
    morning_thirty_day_psalms: &'a [Psalm],
    evening_thiry_day_psalms: &'a [Psalm],
) -> ReadingLinks<'a> {
    ReadingLinks {
        morning_office_psalms: psalm_links(&morning.daily_office_psalms),
        evening_office_psalms: psalm_links(&evening.daily_office_psalms),
        morning_30_psalms: psalm_links(morning_thirty_day_psalms),
        evening_30_psalms: psalm_links(evening_thiry_day_psalms),
        morning_readings: lectionary_reading_links(&morning.daily_office_readings),
        evening_readings: lectionary_reading_links(&evening.daily_office_readings),
    }
}

fn psalm_links(psalms: &[Psalm]) -> Vec<&str> {
    psalms
        .iter()
        .filter_map(|psalm| psalm.citation.as_deref())
        .collect()
}

fn lectionary_reading_links(readings: &[Reading]) -> Vec<&str> {
    readings
        .iter()
        .map(|reading| reading.citation.as_str())
        .collect()
}

fn reading_links_view(links: ReadingLinks, thirty_day: Behavior<bool>) -> View {
    let readings_different = links.morning_readings != links.evening_readings;

    view! {
        <table>
            <tr>
                <th>{t!("daily_readings.morning")}</th>
                <th>{t!("daily_readings.evening")}</th>
            </tr>
            <tr>
                <td>
                    <dyn:view view={
                        psalm_links_view(
                            links.morning_office_psalms,
                            links.morning_30_psalms,
                            thirty_day.clone()
                        )
                    }/>
                </td>
                <td>
                    <dyn:view view={
                        psalm_links_view(
                            links.evening_office_psalms,
                            links.evening_30_psalms,
                            thirty_day
                        )
                    }/>
                </td>
            </tr>
            <tr>
                <td colspan={if readings_different { "1" } else { "2" } }>
                    {reading_links_reading_view(links.morning_readings)}
                </td>
                <td>
                    { if readings_different {
                        reading_links_reading_view(links.evening_readings)
                    } else {
                        View::Empty
                    }}
                </td>
            </tr>
        </table>
    }
}

fn psalm_links_view(
    daily_psalms: Vec<&str>,
    thirty_day_psalms: Vec<&str>,
    use_thirty_day_psalms: Behavior<bool>,
) -> View {
    let daily_psalm_links = View::Fragment(
        daily_psalms
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                    </li>
                }
            })
            .collect(),
    );
    let thirty_psalm_links = View::Fragment(
        thirty_day_psalms
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                    </li>
                }
            })
            .collect(),
    );
    view! {
        <>
            <dyn:ul class:hidden={use_thirty_day_psalms.stream().map(|use_30| use_30).boxed_local()}>
                {daily_psalm_links}
            </dyn:ul>
            <dyn:ul
                class="hidden"
                class:hidden={use_thirty_day_psalms.stream().map(|use_30| !use_30).boxed_local()}
            >
                {thirty_psalm_links}
            </dyn:ul>
        </>
    }
}

fn reading_links_reading_view(readings: Vec<&str>) -> View {
    let reading_links = View::Fragment(
        readings
            .iter()
            .map(|citation| {
                view! {
                    <li>
                        <a href={&format!("#{}", citation)}>{citation.to_string()}</a>
                    </li>
                }
            })
            .collect(),
    );

    view! {
        <ul>{reading_links}</ul>
    }
}

fn title_view(locale: &str, observance: &LiturgicalDayId, localized_name: &str) -> View {
    match observance {
        LiturgicalDayId::Feast(feast) => view! {
            <h1>
                <a href={&format!("/{}/holy-day/{:#?}", locale, feast)}>{localized_name}</a>
            </h1>
        },
        LiturgicalDayId::TransferredFeast(feast) => view! {
            <h1>
                <a href={&format!("/{}/holy-day/{:#?}", locale, feast)}>{localized_name}</a>
                <br/>
                {t!("daily_readings.transferred")}
            </h1>
        },
        _ => view! {
            <h1>{localized_name}</h1>
        },
    }
}

fn observance_view(
    locale: &str,
    use_default: bool,
    use_this_observance: impl Stream<Item = Option<bool>> + 'static,
    use_thirty_day_psalms: Behavior<bool>,
    use_lff_2018: Behavior<bool>,
    observance: &ObservanceSummary,
    thirty_day_psalms: &[Psalm],
) -> View {
    let bcp_black_letter_days = View::Fragment(
        observance
            .bcp_black_letter_days
            .iter()
            .map(|(feast, name)| {
                let url = format!("/{}/holy-day/{:#?}", locale, feast);
                view! {
                    <li>
                        <a href={url}>{name}</a>
                    </li>
                }
            })
            .collect(),
    );

    let lff_black_letter_days = View::Fragment(
        observance
            .lff_black_letter_days
            .iter()
            .map(|(feast, name)| {
                let url = format!("/{}/holy-day/{:#?}", locale, feast);
                view! {
                    <li>
                        <a href={url}>{name}</a>
                    </li>
                }
            })
            .collect(),
    );

    let collect_view = observance
        .collects
        .as_ref()
        .map(|collects| {
            view! {
                <h2>{t!("lookup.collect_of_the_day")}</h2>
                <dyn:view view={DocumentController::new(collects.clone()).view(locale)} />
            }
        })
        .unwrap_or(View::Empty);

    let hide_bcp_black_letter_days = use_lff_2018.stream().boxed_local();
    let hide_lff_black_letter_days = use_lff_2018.stream().map(|n| !n).boxed_local();

    let title = title_view(locale, &observance.observance, &observance.localized_name);

    view! {
        <dyn:section
            class={if use_default { "" } else { "hidden" }}
            class:hidden={use_this_observance.map(move |yes| !yes.unwrap_or(use_default)).boxed_local()}
        >
            {title}
            <dyn:ul class="black-letter-days"
                class:hidden={hide_bcp_black_letter_days}
            >
                {bcp_black_letter_days}
            </dyn:ul>
            <dyn:ul class="black-letter-days hidden"
                class:hidden={hide_lff_black_letter_days}
            >
                {lff_black_letter_days}
            </dyn:ul>

            {collect_view}

            <dyn:section
                class:hidden={use_thirty_day_psalms.stream().map(|use_30| use_30).boxed_local()}
            >
                <h2>{t!("daily_readings.psalms")}</h2>
                {psalms_view(locale, &observance.daily_office_psalms)}
            </dyn:section>
            <dyn:section
                class="hidden"
                class:hidden={use_thirty_day_psalms.stream().map(|use_30| !use_30).boxed_local()}
            >
                <h2>{t!("daily_readings.psalms")}</h2>
                {psalms_view(locale, thirty_day_psalms)}
            </dyn:section>
            <section>
                <h2>{t!("daily_readings.daily_office_readings")}</h2>
                {readings_view(locale, observance)}
            </section>
        </dyn:section>
    }
}

fn psalms_view(locale: &str, psalms: &[Psalm]) -> View {
    View::Fragment(
        psalms
            .iter()
            .map(|psalm| {
                let id = psalm.citation.clone().unwrap_or_default();
                view! {
                    <article class="document" id={id}>
                        {DocumentController::new(Document::from(psalm.clone())).view(locale)}
                    </article>
                }
            })
            .collect(),
    )
}

fn readings_view(locale: &str, summary: &ObservanceSummary) -> View {
    let readings = &summary.daily_office_readings;
    let version = preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion))
        .and_then(|value| match value {
            PreferenceValue::Version(version) => Some(version),
            _ => None,
        })
        .unwrap_or(Version::NRSV);

    if readings.is_empty() {
        View::Empty
    } else {
        View::Fragment(
            readings
                .iter()
                .map(|reading| {
                    view! {
                        <>
                            <a id={&reading.citation}></a>
                            {DocumentController::new(Document::from(BiblicalCitation::from(reading.citation.clone())).version(version)).view(locale)}
                        </>
                    }
                })
                .collect(),
        )
    }
}

struct Controls {
    use_evening: Toggle,
    use_30_day_psalter: Toggle,
    use_alternate_observance: ObservancePicker,
    use_lff_2018: Toggle,
}

impl Controls {
    fn new(summary: DailySummary) -> Self {
        let use_lff_2018 = Toggle::new(
            preferences::is(
                &PreferenceKey::from(GlobalPref::Calendar),
                &PreferenceValue::from("lff2018"),
            ),
            "calendar",
            t!("bcp_1979"),
            t!("lff_2018"),
            None,
        );
        let use_evening = Toggle::new(
            current_hour() >= 13,
            "time_of_day",
            t!("daily_readings.morning"),
            t!("daily_readings.evening"),
            None,
        );
        let use_30_day_psalter = Toggle::new(
            preferences::is(
                &PreferenceKey::from(GlobalPref::PsalmCycle),
                &PreferenceValue::from(Lectionaries::BCP1979ThirtyDayPsalms),
            ),
            "psalm_cycle",
            t!("daily_readings.daily_office_psalms"),
            t!("daily_readings.thirty_day_psalms"),
            None,
        );
        let use_alternate_observance =
            ObservancePicker::new(use_evening.toggled.clone(), Rc::new(summary));
        Self {
            use_evening,
            use_30_day_psalter,
            use_alternate_observance,
            use_lff_2018,
        }
    }

    fn view(&self) -> View {
        view! {
            <>
                <dyn:view view={self.use_lff_2018.view()}/>
                <dyn:view view={self.use_evening.view()}/>
                <dyn:view view={self.use_30_day_psalter.view()}/>
                <dyn:view view={self.use_alternate_observance.view()}/>
            </>
        }
    }
}
