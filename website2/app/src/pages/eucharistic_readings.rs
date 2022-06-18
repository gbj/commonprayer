use crate::{
    components::Tabs,
    utils::time::today,
    views::{readings::*, *},
    WebView,
};
use api::summary::{EucharisticObservanceSummary, FirstLessonAndPsalm, TrackedReadings};
use calendar::{Date, Feast, LiturgicalDay};
use language::Language;
use leptos2::*;
use library::CommonPrayer;
use liturgy::{Content, Version};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct EucharisticReadingsPage {
    pub day: LiturgicalDay,
    pub observed: EucharisticObservanceSummary,
    pub version: Version,
}

#[derive(Deserialize)]
pub struct EucharisticReadingsQuery {
    date: Option<String>,
    alternate: Option<String>,
    version: Option<Version>,
}

impl Page for EucharisticReadingsPage {
    type Params = ();
    type Query = EucharisticReadingsQuery;

    fn name() -> &'static str {
        "readings"
    }

    fn paths() -> Vec<String> {
        vec!["".into()]
    }

    fn build_state(
        locale: &str,
        _path: &str,
        _params: Self::Params,
        query: Self::Query,
    req: &HttpRequest
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
            day: summary.day,
            version,
            observed: if alternate.is_some() {
                summary.alternate.unwrap_or(summary.observed)
            } else {
                summary.observed
            },
        })
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        let title = format!(
            "{} – {} – {}",
            self.observed.localized_name,
            t!("lookup.lectionary_reading"),
            t!("common_prayer")
        );

        view! {
            <>
                <title>{title}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/document.css"/>
                <link rel="stylesheet" href="/static/daily-readings.css"/>
                <link rel="stylesheet" href="/static/display-settings.css"/>
                <link rel="stylesheet" href="/static/settings.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        // basically, if there's no gospel there are no readings, even
        // first lesson and psalm
        let tracked_readings = if self.observed.gospel.is_none() {
            None
        } else {
            Some(match &self.observed.tracked_readings {
                TrackedReadings::Any(readings) => {
                    view! {
                        <section>
                            {first_lesson_and_psalm_view(locale, readings, self.version)}
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
                                    {first_lesson_and_psalm_view(locale, track_one, self.version)}
                                </section>
                                <section>
                                    {first_lesson_and_psalm_view(locale, track_two, self.version)}
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
                    <a href={format!("/{}/daily-readings/{}", locale, self.day.date)}>
                        {t!("daily_readings.daily_office_readings")}
                    </a>
                    " "
                    {t!("or")}
                    " "
                    <a href={format!("/{}/lectionary/{}#{}", locale, self.day.date.year(), self.day.date.month())}>
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
                {Header::new(locale, &t!("lookup.lectionary_reading")).to_node()}
                <main>
                    // Choose Date & Bible Version
                    {readings_settings_form(locale, self.day.date, self.version, vec![])}

                    // Name of Day
                    <h1>{title_view(locale, &self.observed.observance, &self.observed.localized_name)}</h1>

                    // Collect of the Day
                    {self.observed.collects.as_ref().map(|doc| view! {
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
                    {self.observed.liturgy_of_the_palms.as_ref().map(|doc| view! {
                        <>
                            <a id="palms"></a>
                            <h3>{t!("lectionary.palms")}</h3>
                            <article class="document">
                                {DocumentView {
                                    path: vec![],
                                    doc: &doc.clone().version(self.version)
                                }
                                .view(locale)}
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
                                    .view(locale)
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
                                    .view(locale)
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
                                    .view(locale)
                                }
                            </article>
                        </>
                    }).unwrap_or_default()}
                    {no_readings_link}
                </main>
            </>
        }
    }
}

fn first_lesson_and_psalm_view(
    locale: &str,
    readings: &FirstLessonAndPsalm,
    version: Version,
) -> Vec<Node> {
    let first_lesson = if let Some(first_lesson) = &readings.first_lesson {
        view! {
            <>
                <a id="first-lesson"></a>
                <h3>{t!("lectionary.first_lesson")}</h3>
                <article class="document">
                   {DocumentView { path: vec![], doc: &first_lesson.clone().version(version) }.view(locale)}
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
