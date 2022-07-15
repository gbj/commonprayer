use crate::{components::Tabs, utils::encode_uri};
use api::summary::{DocumentOrReading, EucharisticObservanceSummary, TrackedReadings};
use calendar::{Date, Feast, LiturgicalDay, LiturgicalDayId};
use docx::DocxDocument;
use futures::{future::join_all, Future};
use language::Language;
use leptos2::*;
use library::CommonPrayer;
use liturgy::{Content, Document, DocumentError, Heading, HeadingLevel, Parallel, Version};
use std::{pin::Pin, str::FromStr};

use crate::{routes::document::views::DocumentView, utils::time::today, WebView};

use super::{
    export_docx::{add_readings, docx_response},
    reading_loader::ReadingLoader,
    views::*,
};

pub struct EucharistView {
    pub locale: String,
    pub day: LiturgicalDay,
    pub observance: LiturgicalDayId,
    pub alternates: Vec<(Option<Feast>, String)>,
    pub use_alternate: Option<Feast>,
    pub localized_name: String,
    pub version: Version,
    pub collects: Option<Document>,
    pub is_tracked: bool,
    pub using_track_two: bool,
    pub first_lesson: Vec<ReadingLoader>,
    pub psalm: Vec<Document>,
    pub epistle: Vec<ReadingLoader>,
    pub gospel: Vec<ReadingLoader>,
    pub vigil_readings: Vec<DocumentOrReadingLoader>,
    pub liturgy_of_the_palms: Vec<ReadingLoader>,
}

#[derive(Debug)]
pub enum DocumentOrReadingLoader {
    Document(Box<Document>),
    ReadingLoader(Vec<ReadingLoader>),
}

impl DocumentOrReadingLoader {
    pub fn into_future(self) -> Pin<Box<dyn Future<Output = Document>>> {
        match self {
            DocumentOrReadingLoader::Document(doc) => {
                Box::pin(async { *doc }) as Pin<Box<dyn Future<Output = Document>>>
            }
            DocumentOrReadingLoader::ReadingLoader(loaders) => Box::pin(async {
                let readings = join_all(loaders.into_iter().map(ReadingLoader::into_future)).await;
                Document::from(Parallel::from(readings.into_iter().map(|reading| {
                    reading
                        .map(|reading| Document::from(reading))
                        .unwrap_or_else(|e| {
                            Document::from(Content::Error(DocumentError::from(e.to_string())))
                        })
                })))
            }),
        }
    }
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
        _req: Arc<dyn Request>,
        _params: Self::Params,
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
        let summary = CommonPrayer::eucharistic_lectionary_summary(&date, language);

        let alternates = if summary.alternates.is_empty() {
            vec![]
        } else {
            std::iter::once((None, summary.observed.localized_name.clone()))
                .chain(summary.alternates.iter().filter_map(
                    |observance| match observance.observance {
                        LiturgicalDayId::Feast(feast)
                        | LiturgicalDayId::TransferredFeast(feast) => {
                            Some((Some(feast), observance.localized_name.clone()))
                        }
                        _ => None,
                    },
                ))
                .collect()
        };
        let (use_alternate, observed) = match alternate {
            Some(chosen_alternate) => {
                match summary.alternates.into_iter().find(|alternate| {
                    alternate.observance == LiturgicalDayId::Feast(chosen_alternate)
                        || alternate.observance
                            == LiturgicalDayId::TransferredFeast(chosen_alternate)
                }) {
                    Some(alternate_observance) => (Some(chosen_alternate), alternate_observance),
                    None => (None, summary.observed),
                }
            }
            _ => (None, summary.observed),
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
                    .map(|citation| ReadingLoader::new(citation, version, None))
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
                            .map(|citation| ReadingLoader::new(citation, version, None))
                            .collect(),
                        track_two.psalm,
                    )
                } else {
                    (
                        track_one
                            .first_lesson
                            .iter()
                            .map(|citation| ReadingLoader::new(citation, version, None))
                            .collect(),
                        track_one.psalm,
                    )
                }
            }
        };
        let epistle = epistle
            .iter()
            .map(|citation| ReadingLoader::new(citation, version, None))
            .collect();
        let gospel = gospel
            .iter()
            .map(|citation| ReadingLoader::new(citation, version, None))
            .collect();
        let liturgy_of_the_palms = liturgy_of_the_palms
            .iter()
            .map(|citation| ReadingLoader::new(citation, version, None))
            .collect();

        let vigil_readings = vigil_readings
            .into_iter()
            .map(|reading| match reading {
                DocumentOrReading::Document(doc) => DocumentOrReadingLoader::Document(doc),
                DocumentOrReading::Reading(readings) => DocumentOrReadingLoader::ReadingLoader(
                    readings
                        .into_iter()
                        .map(|citation| ReadingLoader::new(&citation, version, None))
                        .collect(),
                ),
            })
            .collect();

        Some(Self {
            locale: locale.to_string(),
            localized_name,
            day: summary.day,
            observance,
            alternates,
            use_alternate,
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

    // POST to download Word doc
    async fn action(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> ActionResponse {
        let data = Self::loader(locale, req, params, query).await;
        if let Some(data) = data {
            let docx = DocxDocument::new();

            // Title
            let docx = docx.add_content(&Document::from(Heading::from((
                HeadingLevel::Heading1,
                data.day
                    .date
                    .to_localized_name(Language::from_locale(locale)),
            ))));

            // Collect
            let mut docx = if let Some(collects) = data.collects {
                docx.add_content(&Document::from(Heading::from((
                    HeadingLevel::Heading2,
                    t!("lookup.collect_of_the_day"),
                ))))
                .add_content(&collects)
            } else {
                docx
            };

            // Psalms & Readings
            if !data.liturgy_of_the_palms.is_empty() {
                docx = add_readings(docx, data.liturgy_of_the_palms).await;
            }

            if !data.vigil_readings.is_empty() {
                let readings = join_all(
                    data.vigil_readings
                        .into_iter()
                        .map(DocumentOrReadingLoader::into_future),
                )
                .await;
                docx = readings
                    .into_iter()
                    .fold(docx, |docx, reading| docx.add_content(&reading));
            }

            if data.first_lesson.is_empty() {
                docx = add_readings(docx, data.epistle).await;
                docx = data
                    .psalm
                    .into_iter()
                    .fold(docx, |docx, psalm| docx.add_content(&psalm));
            } else {
                docx = add_readings(docx, data.first_lesson).await;
                docx = data
                    .psalm
                    .into_iter()
                    .fold(docx, |docx, psalm| docx.add_content(&psalm));
                docx = add_readings(docx, data.epistle).await;
            }

            docx = add_readings(docx, data.gospel).await;

            match docx_response(data.day.date, docx) {
                Ok(path) => ActionResponse::from_path(path),
                Err(e) => ActionResponse::from_error(e),
            }
        } else {
            ActionResponse::None
        }
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

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
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

        let has_vigil = !self.vigil_readings.is_empty();
        let psalm_after_epistle = self.first_lesson.is_empty();
        let psalm_view = self.psalm_view();

        let track_selection = if self.is_tracked {
            Some(view! {
                <div class="toggle-links right">
                    <a href={format!(
                            "?{}",
                            self.build_query_string(self.day.date, self.use_alternate, self.version, false)
                        )}
                        class:current={!self.using_track_two}
                    >
                        {t!("lectionary.track_one")}
                    </a>
                    <a href={format!(
                            "?{}",
                            self.build_query_string(self.day.date, self.use_alternate, self.version, true)
                        )}
                        class:current={self.using_track_two}
                    >
                        {t!("lectionary.track_two")}
                    </a>
                </div>
            })
        } else {
            None
        };

        view! {
            <div>
                <section>
                    {self.alternate_toggles()}

                    // Name of Day
                    <h1>{title_view(&self.locale, &self.observance, &self.localized_name)}</h1>

                    {self.reading_links()}

                    // Collect of the Day
                    {self.collects.as_ref().map(|doc| view! {
                        <div class="collect">
                            <h3>{t!("lookup.collect_of_the_day")}</h3>
                            {DocumentView {
                                path: vec![],
                                doc,
                                url: ""
                            }
                            .view(&self.locale)}
                        </div>
                    })}

                    {if has_vigil {
                        Some(view! { <h2>{t!("daily_readings.vigil_readings")}</h2> })
                    } else {
                        None
                    }}

                    // Palms and Vigil Readings preceded other Eucharistic readings
                    {async_readings_view(&self.locale, self.liturgy_of_the_palms)}
                    {self.vigil_readings
                        .into_iter()
                        .flat_map(|doc| {
                            match doc {
                                DocumentOrReadingLoader::Document(doc) => {
                                    view! {
                                        <>
                                            <article class="document">
                                            {
                                                DocumentView {
                                                    path: vec![],
                                                    doc: &doc.clone()
                                                        .version(if matches!(doc.content, Content::Psalm(_)) {
                                                            doc.version
                                                        } else {
                                                            self.version
                                                        }),
                                                    url: ""
                                                }
                                                .view(&self.locale)
                                            }
                                            </article>
                                        </>
                                    }
                                }
                                DocumentOrReadingLoader::ReadingLoader(loaders) => async_readings_view(&self.locale, loaders)
                            }
                        })
                        .collect::<Vec<_>>()
                    }

                    {if has_vigil {
                        Some(view! { <h2>{t!("daily_readings.eucharist_readings")}</h2> })
                    } else {
                        None
                    }}

                    // Readings
                    {track_selection}

                    {async_readings_view(&self.locale, self.first_lesson)}

                    {if !psalm_after_epistle {
                        psalm_view.clone()
                    } else {
                        vec![]
                    }}

                    {async_readings_view(&self.locale, self.epistle)}

                    {if psalm_after_epistle {
                        psalm_view
                    } else {
                        vec![]
                    }}

                    {async_readings_view(&self.locale, self.gospel)}

                    {no_readings_link}
                </section>
            </div>
        }
    }
}

impl EucharistView {
    fn build_query_string(
        &self,
        date: Date,
        alternate: Option<Feast>,
        version: Version,
        track_two: bool,
    ) -> String {
        [
            ("date", Some(date.to_padded_string())),
            ("alternate", alternate.map(|v| v.to_string())),
            ("version", Some(version.to_string())),
            ("track", if track_two { Some("two".into()) } else { None }),
        ]
        .into_iter()
        .filter_map(|(k, v)| v.map(|v| format!("{}={}", k, encode_uri(&v))))
        .intersperse_with(|| String::from("&"))
        .collect::<String>()
    }

    fn alternate_toggles(&self) -> Option<Node> {
        if self.alternates.is_empty() {
            None
        } else {
            let links = self
                .alternates
                .iter()
                .map(|(feast, name)| {
                    let href = format!(
                        "?{}",
                        self.build_query_string(
                            self.day.date,
                            *feast,
                            self.version,
                            self.is_tracked && self.using_track_two
                        )
                    );
                    let is_current = &self.use_alternate == feast;
                    view! { <a href={href} class:current={is_current}>{name}</a> }
                })
                .collect::<Vec<_>>();
            Some(view! {
                <div class="toggle-links">
                    {links}
                </div>
            })
        }
    }

    fn reading_links(&self) -> Node {
        let eucharist_links = if self.first_lesson.is_empty() {
            view! {
                <ul class="reading-links">
                    {Self::reading_link(&self.epistle)}
                    {Self::psalm_link(&self.psalm)}
                    {Self::reading_link(&self.gospel)}
                </ul>
            }
        } else {
            view! {
                <ul class="reading-links">
                    {Self::reading_link(&self.first_lesson)}
                    {Self::psalm_link(&self.psalm)}
                    {Self::reading_link(&self.epistle)}
                    {Self::reading_link(&self.gospel)}
                </ul>
            }
        };

        if self.vigil_readings.is_empty() {
            eucharist_links
        } else {
            let vigil_links = self
                .vigil_readings
                .iter()
                .map(|reading| match reading {
                    DocumentOrReadingLoader::Document(doc) => match &doc.content {
                        Content::Choice(choice) => {
                            let links = choice
                                .options
                                .iter()
                                .enumerate()
                                .map(|(idx, doc)| {
                                    let citation =
                                        doc.as_citation().unwrap_or_else(|| idx.to_string());
                                    view! { <a href={format!("#{}", citation)}>{citation}</a> }
                                })
                                .intersperse_with(|| text(format!(" {} ", t!("daily_readings.or"))))
                                .collect::<Vec<_>>();
                            view! { <li class="vigil-psalm">{links}</li> }
                        }
                        _ => {
                            let citation = doc.as_citation().unwrap_or_default();
                            view! { <li class="vigil-psalm"><a href={format!("#{}", citation)}>{citation}</a></li> }
                        }
                    },
                    DocumentOrReadingLoader::ReadingLoader(readings) => Self::reading_link(readings),
                })
                .collect::<Vec<_>>();
            view! {
                <ul class="reading-links">
                    <li>
                        <h3>{t!("daily_readings.vigil_readings")}</h3>
                        <ul class="reading-links">{vigil_links}</ul>
                    </li>
                    <li>
                        <h3>{t!("daily_readings.eucharist_readings")}</h3>
                        {eucharist_links}
                    </li>
                </ul>
            }
        }
    }

    pub(crate) fn reading_link(readings: &[ReadingLoader]) -> Node {
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

    pub(crate) fn psalm_link(readings: &[Document]) -> Node {
        let links = readings
            .iter()
            .filter_map(|doc| {
                let label = match &doc.content {
                    Content::Psalm(psalm) => Some(psalm.citation.clone().unwrap_or_else(|| {
                        doc.label.clone().unwrap_or_else(|| {
                            t!("daily_readings.psalm", number = &psalm.number.to_string())
                        })
                    })),
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
            .intersperse_with(|| text(format!(" {} ", t!("or"))))
            .collect::<Vec<_>>();
        view! {
            <li>{links}</li>
        }
    }

    fn psalm_view(&self) -> Vec<Node> {
        if self.psalm.is_empty() {
            vec![]
        } else if self.psalm.len() == 1 {
            self.psalm
                .get(0)
                .map(|doc| {
                    view! {
                        <>
                            <a id={doc.as_citation().unwrap_or_default()}></a>
                            <h3>{t!("lectionary.psalm")}</h3>
                            <article class="document">
                                {DocumentView {
                                    path: vec![],
                                    doc,
                                    url: ""
                                }.view(&self.locale)}
                            </article>
                        </>
                    }
                })
                .unwrap_or_default()
        } else {
            let labels = self
                .psalm
                .iter()
                .enumerate()
                .map(|(idx, doc)| {
                    Document::as_citation(doc)
                        .unwrap_or_else(|| t!("daily_readings.option", n = &idx.to_string()))
                })
                .collect::<Vec<_>>();
            let psalms = self.psalm.iter().map(|doc| {
                view! {
                    <div>
                        <a id={doc.as_citation().unwrap_or_default()}></a>
                        <h3>{t!("lectionary.psalm")}</h3>
                        <article class="document">
                            {DocumentView {
                                path: vec![],
                                doc,
                                url: ""
                            }.view(&self.locale)}
                        </article>
                    </div>
                }
            });
            view! {
                <>
                    <Tabs
                        prop:labels={labels.clone()}
                    >
                        {Tabs::content(psalms)}
                    </Tabs>
                </>
            }
        }
    }
}
