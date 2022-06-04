use std::rc::Rc;

use api::summary::{
    DailySummary, EucharisticLectionarySummary, EucharisticObservanceSummary, FirstLessonAndPsalm,
    ObservanceSummary, TrackedReadings,
};
use calendar::{Date, LiturgicalDayId};
use futures::{Stream, StreamExt};
use lectionary::Reading;
use leptos2::*;
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
    views::{document::DocumentView, Header},
    WebView,
};

mod daily_office_view;
mod eucharistic_reading_view;
pub use daily_office_view::*;
pub use eucharistic_reading_view::*;
mod reading_links;
use reading_links::*;
mod redirect;
use redirect::*;

#[derive(PartialEq)]
pub enum ReadingsPage {
    None,
    DailyOffice(Box<DailySummary>),
    Lectionary(Box<EucharisticLectionarySummary>),
}

#[derive(Deserialize)]
pub struct ReadingsPageParams {
    date: Option<String>,
    alternate_readings: Option<String>,
}

impl Page for ReadingsPage {
    type Params = ReadingsPageParams;

    fn name() -> &'static str {
        "readings"
    }

    fn paths() -> Vec<String> {
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

    fn build_state(locale: &str, path: &str, params: Self::Params) -> Option<Self> {
        let language = locale_to_language(locale);
        let date = &params.date;
        if date.is_none() && !path.contains(&format!("{}/", RedirectMode::Eucharist)) {
            Some(ReadingsPage::None)
        } else if date.is_some() && path.contains(&format!("{}/", RedirectMode::Eucharist)) {
            date.as_ref().and_then(|date| {
                Date::parse_from_str(date, "%Y-%m-%d").ok().map(|date| {
                    let alternate = params
                        .alternate_readings
                        .as_ref()
                        .and_then(|feast| serde_json::from_str(&format!(r#""{feast}""#)).ok());
                    let summary =
                        CommonPrayer::eucharistic_lectionary_summary(&date, language, alternate);

                    ReadingsPage::Lectionary(Box::new(summary))
                })
            })
        } else if let Some(date) = date {
            Date::parse_from_str(date, "%Y-%m-%d").ok().map(|date| {
                let summary = CommonPrayer::daily_office_summary(&date, language);

                ReadingsPage::DailyOffice(Box::new(summary))
            })
        } else {
            None
        }
    }

    fn head(&self, locale: &str) -> Vec<Node> {
        let title = format!(
            "{} – {}",
            if matches!(self, ReadingsPage::Lectionary(_)) {
                t!("menu.lectionary")
            } else {
                t!("toc.daily_readings")
            },
            t!("common_prayer")
        );

        // no summary means no date param is present in URL => redirect to client's current day
        let redirect = redirect_script(locale, "readings", self == &ReadingsPage::None);

        view! {
            <>
                <title>{title}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/document.css"/>
                <link rel="stylesheet" href="/static/daily-readings.css"/>
                <link rel="stylesheet" href="/static/display-settings.css"/>
                <link rel="stylesheet" href="/static/settings.css"/>
                {redirect}
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        match self {
            // JavaScript in header will redirect to today's Daily Office page
            ReadingsPage::None => view! {
                <>
                    {Header::new(locale, &t!("toc.daily_readings")).to_node()}
                    <main>
                    </main>
                </>
            },
            ReadingsPage::DailyOffice(summary) => office_body(locale, summary),
            ReadingsPage::Lectionary(summary) => eucharist_body(locale, summary),
        }
    }
}

fn office_body(locale: &str, summary: &DailySummary) -> Vec<Node> {
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

    view! {
        <>
            {Header::new(locale, &t!("toc.daily_readings")).to_node()}
            <main>
                <DailyOfficeView
                    locale={locale}
                    prop:date={Some(summary.date)}
                    morning-observance={&summary.morning.observed.localized_name}
                    morning-alternate={summary.morning.alternate.as_ref().map(|observance| observance.localized_name.clone()).unwrap_or_default()}
                    evening-observance={&summary.evening.observed.localized_name}
                    evening-alternate={summary.evening.alternate.as_ref().map(|observance| observance.localized_name.clone()).unwrap_or_default()}
                >
                    // Reading Links
                    <section class="reading-link-table" slot="primary_links_daily_psalms">
                        {reading_links_view(&primary_reading_links, false)}
                    </section>
                    <section class="reading-link-table" slot="primary_links_30day_psalms">
                        {reading_links_view(&primary_reading_links, true)}
                    </section>
                    <section class="reading-link-table" slot="alternate_links_daily_psalms">
                        {reading_links_view(&alternate_reading_links, false)}
                    </section>
                    <section class="reading-link-table" slot="alternate_links_30day_psalms">
                        {reading_links_view(&alternate_reading_links, true)}
                    </section>

                    // Name/Collect
                    <section slot="primary_morning_header">
                        {observance_header_view(locale, Some(&summary.morning.observed), false)}
                    </section>
                    <section slot="primary_evening_header">
                        {observance_header_view(locale, Some(&summary.evening.observed), false)}
                    </section>
                     <section slot="alternate_morning_header">
                        {observance_header_view(locale, summary.morning.alternate.as_ref(), false)}
                    </section>
                     <section slot="alternate_evening_header">
                        {observance_header_view(locale, summary.evening.alternate.as_ref(), false)}
                    </section>
                    <section slot="primary_morning_header_lff">
                        {observance_header_view(locale, Some(&summary.morning.observed), true)}
                    </section>
                    <section slot="primary_evening_header_lff">
                        {observance_header_view(locale, Some(&summary.evening.observed), true)}
                    </section>
                     <section slot="alternate_morning_header_lff">
                        {observance_header_view(locale, summary.morning.alternate.as_ref(), true)}
                    </section>
                     <section slot="alternate_evening_header_lff">
                        {observance_header_view(locale, summary.evening.alternate.as_ref(), true)}
                    </section>

                    // Reading Views
                    <section slot="primary_morning_daily">{psalms_view(locale, &summary.morning.observed.daily_office_psalms)}</section>
                    <section slot="morning_30">{psalms_view(locale, &summary.morning.thirty_day_psalms)}</section>
                    <section slot="primary_morning_readings">
                        {readings_view(locale, &summary.morning.observed)}
                    </section>
                    <section slot="primary_evening_daily">{psalms_view(locale, &summary.evening.observed.daily_office_psalms)}</section>
                    <section slot="evening_30">{psalms_view(locale, &summary.evening.thirty_day_psalms)}</section>
                    <section slot="primary_evening_readings">
                        {readings_view(locale, &summary.evening.observed)}
                    </section>

                    <section slot="alternate_morning_daily">
                        {summary.morning.alternate.as_ref().map(|observance| psalms_view(locale, &observance.daily_office_psalms)).unwrap_or_default()}
                    </section>
                    <section slot="alternate_morning_readings">
                        {summary.morning.alternate.as_ref().map(|observance| readings_view(locale, observance)).unwrap_or_default()}
                    </section>
                    <section slot="alternate_evening_daily">
                        {summary.evening.alternate.as_ref().map(|observance| psalms_view(locale, &observance.daily_office_psalms)).unwrap_or_default()}
                    </section>
                    <section slot="alternate_evening_readings">
                        {summary.evening.alternate.as_ref().map(|observance| readings_view(locale, observance)).unwrap_or_default()}
                    </section>
                </DailyOfficeView>
            </main>
        </>
    }
}

fn eucharist_body(locale: &str, summary: &EucharisticLectionarySummary) -> Vec<Node> {
    let bible_version = preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion))
        .and_then(|value| match value {
            PreferenceValue::Version(version) => Some(version),
            _ => None,
        })
        .unwrap_or(Version::NRSV);

    let tracked_readings = match &summary.observed.tracked_readings {
        TrackedReadings::Any(readings) => {
            view! {
                <>
                    <section slot="untracked">
                        {first_lesson_and_psalm_view(locale, &readings, bible_version)}
                    </section>
                </>
            }
        }
        TrackedReadings::Tracked {
            track_one,
            track_two,
        } => view! {
            <>
                <section slot="track_one">
                    {first_lesson_and_psalm_view(locale, track_one, bible_version)}
                </section>
                <section slot="track_two">
                    {first_lesson_and_psalm_view(locale, track_two, bible_version)}
                </section>
            </>
        },
    };

    // not every day has readings assigned in The Lectionary: offer a choice to redirect
    // either to the Daily Office Lectionary or to The Lectionary
    let no_readings_link = if summary.observed.epistle.is_none()
        && summary.observed.gospel.is_none()
    {
        Some(view! {
            <p class="redirect-links">
                {t!("lectionary.no_readings")}
                " "
                <a href={format!("/{}/readings/{}/{}", locale, RedirectMode::DailyOffice, summary.day.date)}>
                    {t!("daily_readings.daily_office_readings")}
                </a>
                " "
                {t!("or")}
                " "
                <a href={format!("/{}/lectionary/{}#{}", locale, summary.day.date.year(), summary.day.date.month())}>
                    {t!("lectionary.the_lectionary")}
                </a>
                {t!("lectionary.no_readings_end")}
            </p>
        })
    } else {
        None
    };

    view! {
        <>
            {Header::new(locale, &t!("toc.daily_readings")).to_node()}
            <main>
                <EucharisticReadingView
                    locale={locale}
                    prop:date={Some(summary.day.date)}
                    prop:tracked={matches!(summary.observed.tracked_readings, TrackedReadings::Tracked { .. })}
                >
                    <section slot="header">
                        // Name of Day
                        {title_view(locale, &summary.observed.observance, &summary.observed.localized_name)}

                        // Collect of the Day
                        {summary.observed.collects.as_ref().map(|doc| view! {
                            <div class="collect">
                                <h3>{t!("lookup.collect_of_the_day")}</h3>
                                {DocumentView {
                                    path: vec![],
                                    doc
                                }
                                .view(locale)}
                            </div>
                        })}

                        // Palms and Vigil Readings preceded other Eucharistic readings
                        {summary.observed.liturgy_of_the_palms.as_ref().map(|doc| view! {
                            <>
                                <a id="palms"></a>
                                <h3>{t!("lectionary.palms")}</h3>
                                <article class="document">
                                   {DocumentView {
                                        path: vec![],
                                        doc: &doc.clone().version(bible_version)
                                    }
                                    .view(locale)}
                                </article>
                            </>
                        }).unwrap_or_default()}
                        {summary.observed
                            .vigil_readings
                            .iter()
                            .map(|doc| {
                                view! {
                                    <article class="document">
                                    {
                                        DocumentView {
                                            path: vec![],
                                            doc: &doc.clone()
                                                .version(if matches!(doc.content, Content::Psalm(_)) {
                                                    doc.version
                                                } else {
                                                    bible_version
                                                })
                                        }
                                        .view(locale)
                                    }
                                    </article>
                                }
                            })
                            .collect::<Vec<_>>()
                        }
                    </section>

                    // Readings
                    {tracked_readings}

                    <section slot="readings">
                        {summary.observed.epistle.as_ref().map(|doc| view! {
                            <>
                                <a id="epistle"></a>
                                <h3>{t!("lectionary.epistle")}</h3>
                                <article class="document">
                                    {DocumentView {
                                        path: vec![],
                                        doc: &doc.clone()
                                            .version(bible_version)
                                        }
                                        .view(locale)
                                    }
                                </article>
                            </>
                        }).unwrap_or_default()}
                        {summary.observed.gospel.as_ref().map(|doc| view! {
                            <>
                                <a id="gospel"></a>
                                <h3>{t!("lectionary.gospel")}</h3>
                                <article class="document">
                                    {DocumentView {
                                        path: vec![],
                                        doc: &doc.clone()
                                            .version(bible_version)
                                        }
                                        .view(locale)
                                    }
                                </article>
                            </>
                        }).unwrap_or_default()}
                        {no_readings_link}
                    </section>
                </EucharisticReadingView>
            </main>
        </>
    }
}

fn observance_header_view(
    locale: &str,
    summary: Option<&ObservanceSummary>,
    lff: bool,
) -> Vec<Node> {
    summary
        .map(|summary| {
            let title = title_view(locale, &summary.observance, &summary.localized_name);

            let black_letter_days = if lff {
                &summary.lff_black_letter_days
            } else {
                &summary.bcp_black_letter_days
            }
            .iter()
            .map(|(feast, name)| {
                let url = format!("/{}/holy-day/{:#?}", locale, feast);
                view! {
                    <li>
                        <a href={url}>{name}</a>
                    </li>
                }
            })
            .collect::<Vec<_>>();
            let black_letter_days = view! {
                <ul class="black-letter-days">{black_letter_days}</ul>
            };

            let collects = summary
                .collects
                .as_ref()
                .map(|collects| {
                    [
                        view! {
                            <h2>{t!("lookup.collect_of_the_day")}</h2>
                        },
                        DocumentView {
                            path: vec![],
                            doc: collects,
                        }
                        .view(locale),
                    ]
                })
                .into_iter()
                .flatten();

            [title, black_letter_days]
                .into_iter()
                .chain(collects)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn title_view(locale: &str, observance: &LiturgicalDayId, localized_name: &str) -> Node {
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

fn psalms_view(locale: &str, psalms: &[Psalm]) -> Vec<Node> {
    psalms
        .iter()
        .map(|psalm| {
            let id = psalm.citation.clone().unwrap_or_default();
            let doc_view = DocumentView {
                doc: &Document::from(psalm.clone()),
                path: vec![],
            };
            view! {
                <article class="document" id={id}>
                    {doc_view.view(locale)}
                </article>
            }
        })
        .collect()
}

fn readings_view(locale: &str, summary: &ObservanceSummary) -> Vec<Node> {
    let readings = &summary.daily_office_readings;
    let version = preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion))
        .and_then(|value| match value {
            PreferenceValue::Version(version) => Some(version),
            _ => None,
        })
        .unwrap_or(Version::NRSV);

    if readings.is_empty() {
        vec![]
    } else {
        readings
            .iter()
            .flat_map(|reading| {
                let doc_view = DocumentView {
                    doc: &Document::from(BiblicalCitation::from(reading.citation.clone()))
                        .version(version),
                    path: vec![],
                };

                view! {
                    <>
                        <a id={&reading.citation}></a>
                        {doc_view.view(locale)}
                    </>
                }
            })
            .collect()
    }
}

// TODO Bible Version
fn first_lesson_and_psalm_view(
    locale: &str,
    readings: &FirstLessonAndPsalm,
    bible_version: Version,
) -> Vec<Node> {
    let first_lesson = if let Some(first_lesson) = &readings.first_lesson {
        view! {
            <>
                <a id="first-lesson"></a>
                <h3>{t!("lectionary.first_lesson")}</h3>
                <article class="document">
                   {DocumentView { path: vec![], doc: &first_lesson.clone().version(bible_version) }.view(locale)}
                </article>
            </>
        }
    } else {
        vec![]
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
                   {DocumentView { path: vec![], doc: psalm }.view(locale)}
                </article>
            </>
        }
    } else {
        vec![]
    };

    let mut lessons = first_lesson;
    lessons.extend(psalm);
    lessons
}
/*
fn office_body(locale: &str, summary: &DailySummary) -> Vec<Node> {
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

fn observance_view(
    locale: &str,
    use_default: bool,
    use_this_observance: impl Stream<Item = Option<bool>> + 'static,
    use_thirty_day_psalms: Behavior<bool>,
    use_lff_2018: Behavior<bool>,
    observance: &ObservanceSummary,
    thirty_day_psalms: &[Psalm],
) -> Vec<Node> {
    todo!()
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
                &PreferenceValue::Local("lff2018".into()),
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
 */
