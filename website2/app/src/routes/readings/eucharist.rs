use api::summary::{EucharisticObservanceSummary, FirstLessonAndPsalm, TrackedReadings};
use calendar::{Date, Feast, LiturgicalDay, LiturgicalDayId};
use language::Language;
use leptos2::*;
use library::CommonPrayer;
use liturgy::{BiblicalReading, Content, Document, Psalm, Version};
use std::pin::Pin;
use std::str::FromStr;

use crate::{
    components::Tabs,
    utils::{fetch::FetchError, time::today},
    views::{document::DocumentView, readings::*},
    WebView,
};

use super::reading_loader::{load_reading, ReadingFuture};

pub struct EucharistView {
    pub locale: String,
    pub day: LiturgicalDay,
    pub observance: LiturgicalDayId,
    pub use_alternate: bool,
    pub localized_name: String,
    pub version: Version,
    pub collects: Option<Document>,
    pub is_tracked: bool,
    pub using_track_two: bool,
    pub first_lesson: Option<(String, ReadingFuture)>,
    pub psalm: Option<Document>,
    pub epistle: Option<(String, ReadingFuture)>,
    pub gospel: Option<(String, ReadingFuture)>,
    pub vigil_readings: Vec<(String, ReadingFuture)>,
    pub liturgy_of_the_palms: Option<(String, ReadingFuture)>,
}

#[derive(Params)]
pub struct EucharistViewQuery {
    date: Option<String>,      // "YYYY-MM-DD"
    alternate: Option<String>, // can be any string value
    version: Option<Version>,  // any Bible translation Version
    track: Option<String>,     // empty, "one", or "two"
}

#[async_trait(?Send)]
impl Loader for EucharistView {
    type Params = ();
    type Query = EucharistViewQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        // Extract basic settings from URL query
        let language = Language::from_locale(locale);
        let date = query
            .date
            .as_ref()
            .and_then(|date| Date::parse_from_str(date, "%Y-%m-%d").ok())
            .unwrap_or_else(today);
        let alternate = query
            .alternate
            .as_ref()
            .and_then(|feast| Feast::from_str(feast).ok());
        let version = query
            .version
            .filter(Version::is_bible_translation)
            .unwrap_or(Version::NRSV);

        // Build lectionary summary
        let summary = CommonPrayer::eucharistic_lectionary_summary(&date, language, alternate);
        let observed = if alternate.is_some() {
            summary.alternate.unwrap_or(summary.observed)
        } else {
            summary.observed
        };

        let EucharisticObservanceSummary {
            localized_name,
            tracked_readings,
            epistle,
            gospel,
            liturgy_of_the_palms,
            vigil_readings,
            observance,
            collects,
        } = observed;

        // Select the particular readings we need, and set up async loading
        let is_tracked = matches!(&tracked_readings, TrackedReadings::Tracked { .. });
        let using_track_two = query.track.map(|track| track == "two").unwrap_or(false);
        let (first_lesson, psalm) = match tracked_readings {
            TrackedReadings::Any(readings) => (
                (build_reading_with_citation_from_doc(readings.first_lesson, version)),
                readings.psalm,
            ),
            TrackedReadings::Tracked {
                track_one,
                track_two,
            } => {
                if using_track_two {
                    (
                        build_reading_with_citation_from_doc(track_two.first_lesson, version),
                        track_two.psalm,
                    )
                } else {
                    (
                        build_reading_with_citation_from_doc(track_one.first_lesson, version),
                        track_one.psalm,
                    )
                }
            }
        };
        let epistle = build_reading_with_citation_from_doc(epistle, version);
        let gospel = build_reading_with_citation_from_doc(gospel, version);
        let liturgy_of_the_palms =
            build_reading_with_citation_from_doc(liturgy_of_the_palms, version);
        // TODO some of these may Psalms
        let vigil_readings = vec![]; /* observed
                                     .vigil_readings
                                     .into_iter()
                                     .map(|reading| build_reading_from_doc(Some(reading), version))
                                     .collect(); */

        Some(Self {
            locale: locale.to_string(),
            localized_name,
            day: summary.day,
            observance,
            use_alternate: alternate.is_some(),
            version,
            collects,
            is_tracked,
            using_track_two,
            first_lesson,
            psalm,
            epistle,
            gospel,
            liturgy_of_the_palms,
            vigil_readings,
        })
    }
}

fn build_reading_with_citation_from_doc(
    doc: Option<Document>,
    version: Version,
) -> Option<(String, ReadingFuture)> {
    doc.and_then(|doc| match doc.content {
        Content::BiblicalReading(reading) => Some((
            reading.citation.clone(),
            Box::pin(async move { Ok(reading) }) as ReadingFuture,
        )),
        Content::BiblicalCitation(citation) => Some((
            citation.citation.clone(),
            Box::pin(load_reading(citation.citation, version, citation.intro)),
        )),
        _ => None,
    })
}

impl View for EucharistView {
    fn title(&self) -> String {
        format!(
            "{} – {} – {}",
            self.localized_name,
            t!("lookup.lectionary_reading"),
            t!("common_prayer")
        )
    }

    fn styles(&self) -> Styles {
        vec![]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        // not every day has readings assigned in The Lectionary: offer a choice to redirect
        // either to the Daily Office Lectionary or to The Lectionary
        let no_readings_link = if self.epistle.is_none() && self.gospel.is_none() {
            Some(view! {
                <p class="redirect-links">
                    {t!("lectionary.no_readings")}
                    " "
                    <a href={format!("/{}/daily-readings/{}", self.locale, self.day.date)}>
                        {t!("daily_readings.daily_office_readings")}
                    </a>
                    " "
                    {t!("or")}
                    " "
                    <a href={format!("/{}/lectionary/{}#{}", self.locale, self.day.date.year(), self.day.date.month())}>
                        {t!("lectionary.the_lectionary")}
                    </a>
                    {t!("lectionary.no_readings_end")}
                </p>
            })
        } else {
            None
        };

        view! {
            <div>
                <section>
                    // Name of Day
                    <h1>{title_view(&self.locale, &self.observance, &self.localized_name)}</h1>

                    // Collect of the Day
                    {self.collects.as_ref().map(|doc| view! {
                        <div class="collect">
                            <h3>{t!("lookup.collect_of_the_day")}</h3>
                            {DocumentView {
                                path: vec![],
                                doc
                            }
                            .view(&self.locale)}
                        </div>
                    })}

                    // Palms and Vigil Readings preceded other Eucharistic readings
                    {self.liturgy_of_the_palms.map(|(citation, reading)| view! {
                        <>
                            <a id="palms"></a>
                            <h3>{t!("lectionary.palms")}</h3>
                            {async_reading_node(&self.locale, &citation, reading, self.version)}
                        </>
                    }).unwrap_or_default()}
                    /* {self.vigil_readings
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
                                                self.version
                                            })
                                    }
                                    .view(&self.locale)
                                }
                                </article>
                            }
                        })
                        .collect::<Vec<_>>()
                    } */

                    // Readings
                    {if self.is_tracked {
                        Some(view! {
                            <div class="toggle-links right">
                                <a href={
                                    format!(
                                        "/{}/readings/eucharist/?date={}&version={:?}{}",
                                        self.locale,
                                        self.day.date.to_padded_string(),
                                        self.version,
                                        if self.use_alternate {
                                            "&alternate"
                                        } else {
                                            ""
                                        }
                                    )}
                                    class:current={!self.using_track_two}
                                >
                                    {t!("lectionary.track_one")}
                                </a>
                                <a href={
                                    format!(
                                        "/{}/readings/eucharist/?date={}&version={:?}&track=two{}",
                                        self.locale,
                                        self.day.date.to_padded_string(),
                                        self.version,
                                        if self.use_alternate {
                                            "&alternate"
                                        } else {
                                            ""
                                        }
                                    )}
                                    class:current={self.using_track_two}
                                >
                                    {t!("lectionary.track_two")}
                                </a>
                            </div>
                        })
                    } else {
                        None
                    }}

                    {self.first_lesson.map(|(citation, reading)| view! {
                        <>
                            <a id="first-lesson"></a>
                            <h3>{t!("lectionary.first_lesson")}</h3>
                            {async_reading_node(&self.locale, &citation, reading, self.version)}
                        </>
                    }).unwrap_or_default()}

                    {self.psalm.map(|doc| view! {
                        <>
                            <a id="psalm"></a>
                            <h3>{t!("lectionary.psalm")}</h3>
                            <article class="document">
                                {DocumentView {
                                    path: vec![],
                                    doc: &doc
                                }.view(&self.locale)}
                            </article>
                        </>
                    }).unwrap_or_default()}

                    {self.epistle.map(|(citation, reading)| view! {
                        <>
                            <a id="epistle"></a>
                            <h3>{t!("lectionary.epistle")}</h3>
                            {async_reading_node(&self.locale, &citation, reading, self.version)}
                        </>
                    }).unwrap_or_default()}
                    {self.gospel.map(|(citation, reading)| view! {
                        <>
                            <a id="gospel"></a>
                            <h3>{t!("lectionary.gospel")}</h3>
                            {async_reading_node(&self.locale, &citation, reading, self.version)}
                        </>
                    }).unwrap_or_default()}

                    {no_readings_link}
                </section>
            </div>
        }
    }
}

impl EucharistView {
    fn first_lesson_and_psalm_view(&self, readings: &FirstLessonAndPsalm) -> Vec<Node> {
        let first_lesson = if let Some(first_lesson) = &readings.first_lesson {
            view! {
                <>
                    <a id="first-lesson"></a>
                    <h3>{t!("lectionary.first_lesson")}</h3>
                    <article class="document">
                    {DocumentView { path: vec![], doc: &first_lesson.clone().version(self.version) }.view(&self.locale)}
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
                    {DocumentView { path: vec![], doc: psalm }.view(&self.locale)}
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
}
