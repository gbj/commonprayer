use api::summary::{
    EucharisticObservanceSummary, FirstLessonAndPsalm, ObservanceSummary, TrackedReadings,
};
use calendar::{Date, Feast, LiturgicalDay};
use futures::Future;
use language::Language;
use lectionary::Reading;
use leptos2::*;
use library::CommonPrayer;
use liturgy::{BiblicalReading, Content, Psalm, Version};
use std::pin::Pin;
use std::str::FromStr;

use crate::{
    components::Tabs,
    utils::{fetch::FetchError, time::today},
    views::{document::DocumentView, readings::*},
    WebView,
};

use crate::utils::reading_loader::load_reading;

pub struct EucharistView {
    pub locale: String,
    pub day: LiturgicalDay,
    pub observed: EucharisticObservanceSummary,
    pub version: Version,
}

#[derive(Params)]
pub struct EucharistViewQuery {
    date: Option<String>,
    alternate: Option<String>,
    version: Option<Version>,
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
        let summary = CommonPrayer::eucharistic_lectionary_summary(&date, language, alternate);

        Some(Self {
            locale: locale.to_string(),
            day: summary.day,
            version,
            observed: if alternate.is_some() {
                summary.alternate.unwrap_or(summary.observed)
            } else {
                summary.observed
            },
        })
    }
}

impl View for EucharistView {
    fn title(&self) -> String {
        format!(
            "{} – {} – {}",
            self.observed.localized_name,
            t!("lookup.lectionary_reading"),
            t!("common_prayer")
        )
    }

    fn styles(&self) -> Styles {
        vec![]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        // basically, if there's no gospel there are no readings, even
        // first lesson and psalm
        let tracked_readings = if self.observed.gospel.is_none() {
            None
        } else {
            Some(match &self.observed.tracked_readings {
                TrackedReadings::Any(readings) => {
                    view! {
                        <section>
                            {self.first_lesson_and_psalm_view(readings)}
                        </section>
                    }
                }
                TrackedReadings::Tracked {
                    track_one,
                    track_two,
                } => view! {
                    <Tabs
                        data-id="reading-track"
                        prop:labels=vec![t!("lectionary.track_one"), t!("lectionary.track_two")]
                    >
                        {Tabs::content(view! {
                            <>
                                <section>
                                    {self.first_lesson_and_psalm_view(track_one)}
                                </section>
                                <section>
                                    {self.first_lesson_and_psalm_view(track_two)}
                                </section>
                            </>
                        })}
                    </Tabs>
                },
            })
        };

        // not every day has readings assigned in The Lectionary: offer a choice to redirect
        // either to the Daily Office Lectionary or to The Lectionary
        let no_readings_link = if self.observed.epistle.is_none() && self.observed.gospel.is_none()
        {
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
            <>
                <section>
                    // Name of Day
                    <h1>{title_view(&self.locale, &self.observed.observance, &self.observed.localized_name)}</h1>

                    // Collect of the Day
                    {self.observed.collects.as_ref().map(|doc| view! {
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
                    {self.observed.liturgy_of_the_palms.as_ref().map(|doc| view! {
                        <>
                            <a id="palms"></a>
                            <h3>{t!("lectionary.palms")}</h3>
                            <article class="document">
                                {DocumentView {
                                    path: vec![],
                                    doc: &doc.clone().version(self.version)
                                }
                                .view(&self.locale)}
                            </article>
                        </>
                    }).unwrap_or_default()}
                    {self.observed
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
                                                self.version
                                            })
                                    }
                                    .view(&self.locale)
                                }
                                </article>
                            }
                        })
                        .collect::<Vec<_>>()
                    }

                    // Readings
                    {tracked_readings}

                    {self.observed.epistle.as_ref().map(|doc| view! {
                        <>
                            <a id="epistle"></a>
                            <h3>{t!("lectionary.epistle")}</h3>
                            <article class="document">
                                {DocumentView {
                                    path: vec![],
                                    doc: &doc.clone()
                                        .version(self.version)
                                    }
                                    .view(&self.locale)
                                }
                            </article>
                        </>
                    }).unwrap_or_default()}
                    {self.observed.gospel.as_ref().map(|doc| view! {
                        <>
                            <a id="gospel"></a>
                            <h3>{t!("lectionary.gospel")}</h3>
                            <article class="document">
                                {DocumentView {
                                    path: vec![],
                                    doc: &doc.clone()
                                        .version(self.version)
                                    }
                                    .view(&self.locale)
                                }
                            </article>
                        </>
                    }).unwrap_or_default()}
                    {no_readings_link}
                </section>
            </>
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
