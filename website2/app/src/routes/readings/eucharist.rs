use api::summary::{EucharisticObservanceSummary, FirstLessonAndPsalm, TrackedReadings};
use calendar::{Date, Feast, LiturgicalDay, LiturgicalDayId};
use futures::future::join_all;
use language::Language;
use leptos2::*;
use library::CommonPrayer;
use liturgy::{BiblicalReading, Choice, Content, Document, DocumentError, Psalm, Version};
use std::pin::Pin;
use std::str::FromStr;

use crate::{
    components::Tabs,
    utils::{fetch::FetchError, time::today},
    views::{document::DocumentView, readings::*},
    WebView,
};

use super::reading_loader::{load_reading, ReadingFuture, ReadingLoader};

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
    pub first_lesson: Vec<ReadingLoader>,
    pub psalm: Vec<Document>,
    pub epistle: Vec<ReadingLoader>,
    pub gospel: Vec<ReadingLoader>,
    //pub vigil_readings: Vec<ReadingLoader>,
    pub liturgy_of_the_palms: Vec<ReadingLoader>,
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
                readings
                    .first_lesson
                    .iter()
                    .map(|citation| load_reading(citation, version, None))
                    .collect(),
                readings.psalm,
            ),
            TrackedReadings::Tracked {
                track_one,
                track_two,
            } => {
                if using_track_two {
                    (
                        track_two
                            .first_lesson
                            .iter()
                            .map(|citation| load_reading(citation, version, None))
                            .collect(),
                        track_two.psalm,
                    )
                } else {
                    (
                        track_one
                            .first_lesson
                            .iter()
                            .map(|citation| load_reading(citation, version, None))
                            .collect(),
                        track_one.psalm,
                    )
                }
            }
        };
        let epistle = epistle
            .iter()
            .map(|citation| load_reading(citation, version, None))
            .collect();
        let gospel = gospel
            .iter()
            .map(|citation| load_reading(citation, version, None))
            .collect();
        let liturgy_of_the_palms = liturgy_of_the_palms
            .iter()
            .map(|citation| load_reading(citation, version, None))
            .collect();
        // TODO some of these may Psalms
        /*let vigil_readings = vec![];  observed
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
            //vigil_readings,
        })
    }
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
        let no_readings_link = if self.epistle.is_empty() && self.gospel.is_empty() {
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

                    {self.reading_links()}

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
                    {async_readings_view(&self.locale, self.liturgy_of_the_palms)}
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

                    {async_readings_view(&self.locale, self.first_lesson)}

                    {self.psalm
                        .iter()
                        .flat_map(|doc| view! {
                            <>
                                <a id={doc.as_citation().unwrap_or_default()}></a>
                                <h3>{t!("lectionary.psalm")}</h3>
                                <article class="document">
                                    {DocumentView {
                                        path: vec![],
                                        doc: &doc
                                    }.view(&self.locale)}
                                </article>
                            </>
                        })
                        .collect::<Vec<_>>()
                    }

                    {async_readings_view(&self.locale, self.epistle)}
                    {async_readings_view(&self.locale, self.gospel)}

                    {no_readings_link}
                </section>
            </div>
        }
    }
}

impl EucharistView {
    fn reading_links(&self) -> Node {
        view! {
            <ul class="reading-links">
                {self.reading_link(&self.first_lesson)}
                {self.psalm_link(&self.psalm)}
                {self.reading_link(&self.epistle)}
                {self.reading_link(&self.gospel)}
            </ul>
        }
    }

    fn reading_link(&self, readings: &[ReadingLoader]) -> Node {
        let links = readings
            .iter()
            .map(ReadingLoader::as_citation)
            .map(|citation| view! { <a href={format!("#{}", citation)}>{citation}</a> })
            .intersperse_with(|| text(format!(" {} ", t!("or"))))
            .collect::<Vec<_>>();
        view! {
            <li>{links}</li>
        }
    }

    fn psalm_link(&self, readings: &[Document]) -> Node {
        let links = readings
            .iter()
            .filter_map(|doc| {
                let label = match &doc.content {
                    Content::Psalm(psalm) => Some(psalm.citation.clone().unwrap_or(
                        doc.label.clone().unwrap_or_else(|| {
                            t!("daily_readings.psalm", number = &psalm.number.to_string())
                        }),
                    )),
                    Content::PsalmCitation(citation) => Some(citation.0.clone()),
                    Content::Canticle(canticle) => Some(t!(
                        "daily_readings.canticle",
                        number = &canticle.number.to_string()
                    )),
                    _ => None,
                };
                let citation = doc.as_citation();
                match (citation, label) {
                    (Some(citation), Some(label)) => Some((citation, label)),
                    _ => None,
                }
            })
            .map(|(citation, label)| view! { <a href={format!("#{}", citation)}>{label}</a> })
            .intersperse_with(|| text(t!("or")))
            .collect::<Vec<_>>();
        view! {
            <li>{links}</li>
        }
    }
}
