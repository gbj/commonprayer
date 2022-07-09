use crate::WebView;
use crate::{
    components::Tabs,
    routes::document::views::{psalm, DocumentView},
};
use calendar::Date;
use calendar::{
    lff2018::LFF_BIOS, Feast, HolyDayId, LiturgicalDayId, Time, BCP1979_CALENDAR, LFF2018_CALENDAR,
};
use itertools::Itertools;
use language::Language;
use lectionary::{ReadingType, LFF2018_LECTIONARY, RCL};
use leptos2::*;
use library::{
    lff2018::collects::{LFF_COLLECTS_CONTEMPORARY, LFF_COLLECTS_TRADITIONAL},
    CollectId,
};
use liturgy::{Choice, Content, Document, Psalm, Version};
use psalter::bcp1979::BCP1979_PSALTER;

use super::eucharist::EucharistView;
use super::reading_loader::ReadingLoader;
use super::views::async_readings_view;

pub struct HolyDayView {
    date: Option<String>,
    locale: String,
    name: String,
    bio: Option<String>,
    collect_traditional: Document,
    collect_contemporary: Document,
    first_lesson: Vec<ReadingLoader>,
    psalm: Vec<Psalm>,
    epistle: Vec<ReadingLoader>,
    gospel: Vec<ReadingLoader>,
}

#[derive(Params)]
pub struct HolyDayQuery {
    version: Option<Version>, // any Bible translation Version
    id: Option<Feast>,
    date: Option<Date>,
}

#[async_trait(?Send)]
impl Loader for HolyDayView {
    type Params = ();
    type Query = HolyDayQuery;

    async fn loader(
        locale: &str,
        _req: Arc<dyn Request>,
        _params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let version = query
            .version
            .filter(Version::is_bible_translation)
            .unwrap_or(Version::NRSV);

        // if ?id is specified, take that
        // otherwise, try to look up feast by date
        let feast = match query.id {
            Some(feast) => Some(feast),
            None => query.date.and_then(|date| {
                let day = LFF2018_CALENDAR.liturgical_day(date, false);
                if let LiturgicalDayId::Feast(feast) = day.observed {
                    Some(feast)
                } else if let LiturgicalDayId::TransferredFeast(feast) = day.observed {
                    Some(feast)
                } else {
                    day.holy_days.get(0).copied()
                }
            }),
        }?;

        let language = Language::from_locale(locale);
        let eve_of = BCP1979_CALENDAR
            .holy_days
            .iter()
            .find(|(_, s_feast, _, _)| *s_feast == feast)
            .and_then(|(_, _, time, _)| {
                if let Time::EveningOnly(eve_of) = time {
                    eve_of.as_ref().copied()
                } else {
                    None
                }
            });

        let date = LFF2018_CALENDAR
            .holy_days
            .iter()
            .find(|(_, s_feast, _, _)| *s_feast == feast || Some(*s_feast) == eve_of)
            .and_then(|(id, _, _, _)| match id {
                HolyDayId::Date(month, day) => {
                    Some(format!("{} {}", language.month_name(*month), day))
                }
                _ => None,
            });

        let name = LFF2018_CALENDAR
            .feast_name(feast, language)
            // or, search in BCP calendar if can't find in LFF (i.e., for Eve of ___)
            .or_else(|| BCP1979_CALENDAR.feast_name(feast, language))?;

        let bio = LFF_BIOS
            .iter()
            .find(|(s_feast, _)| *s_feast == feast || Some(*s_feast) == eve_of)
            .map(|(_, bio)| bio.to_string());

        // search both RCL and LFF 2018 lectionary for holy day readings
        let lectionary = RCL
            .readings
            .iter()
            .chain(LFF2018_LECTIONARY.readings.iter());
        let readings = lectionary
            .filter(|(id, _, _, _)| id == &LiturgicalDayId::Feast(feast))
            .map(|(_, _, reading_type, citation)| (*reading_type, citation.to_string()))
            .unique()
            .collect::<Vec<_>>();

        let first_lesson = filter_readings(&readings, ReadingType::FirstReading, version);
        let psalm = filter_psalms(&readings);
        let epistle = filter_readings(&readings, ReadingType::SecondReading, version);
        let gospel = filter_readings(&readings, ReadingType::Gospel, version);

        let collect_traditional = LFF_COLLECTS_TRADITIONAL
            .iter()
            .find(|(id, _)| {
                *id == CollectId::Feast(feast) || Some(id) == eve_of.map(CollectId::Feast).as_ref()
            })
            .map(|(_, data)| data.document.clone())
            .unwrap_or_else(|| Document::from(Content::Empty));

        let collect_contemporary = LFF_COLLECTS_CONTEMPORARY
            .iter()
            .find(|(id, _)| {
                *id == CollectId::Feast(feast) || Some(id) == eve_of.map(CollectId::Feast).as_ref()
            })
            .map(|(_, data)| data.document.clone())
            .unwrap_or_else(|| Document::from(Content::Empty));

        Some(HolyDayView {
            date,
            locale: locale.to_string(),
            name,
            bio,
            first_lesson,
            psalm,
            epistle,
            gospel,
            collect_traditional,
            collect_contemporary,
        })
    }
}

fn filter_readings(
    readings: &[(ReadingType, String)],
    reading_type: ReadingType,
    version: Version,
) -> Vec<ReadingLoader> {
    readings
        .iter()
        .filter(|(s_reading_type, _)| *s_reading_type == reading_type)
        .map(|(_, citation)| ReadingLoader::new(citation, version, None))
        .collect()
}

fn filter_psalms(readings: &[(ReadingType, String)]) -> Vec<Psalm> {
    readings
        .iter()
        .filter(|(s_reading_type, _)| *s_reading_type == ReadingType::Psalm)
        .flat_map(|(_, citation)| BCP1979_PSALTER.psalms_by_citation(citation.as_str()))
        .collect()
}

impl View for HolyDayView {
    fn title(&self) -> String {
        format!("{} – {}", self.name, t!("common_prayer"))
    }

    fn styles(&self) -> Styles {
        vec![include_str!("holy_day.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        let reading_links = self.reading_links();
        let psalm_view = self.psalm_view();

        let bio = self.bio.map(|bio| {
            view! {
                <section class="bio">{
                    bio
                    .split("\n\n")
                    .map(|para| {
                        view! {
                          <p>{para}</p>
                        }
                    })
                    .collect::<Vec<_>>()
                }</section>
            }
        });

        view! {
            <div>
                // Title
                <h1>{format!("{}{}{}", self.date.clone().unwrap_or_default(), if self.date.is_some() { ": " } else { "" }, self.name)}</h1>

                // Collect
                <h2>{t!("lookup.collect_of_the_day")}</h2>
                {DocumentView {
                    path: vec![],
                    doc: &Document::from(Choice::from(vec![
                        self.collect_contemporary,
                        self.collect_traditional,
                    ]))
                }.view(&self.locale)}

                // Reading Links
                {reading_links}

                // Bio
                {bio}

                // Actual readings
                <h2>{t!("lectionary.lessons_and_psalm")}</h2>

                // First Lesson, Psalm, Epistle
                // or Epistle, Psalm
                <h3>{t!("lectionary.first_lesson")}</h3>
                {async_readings_view(&self.locale, self.first_lesson)}

                {psalm_view}

                {if self.epistle.is_empty() {
                    None
                } else {
                    Some(view! {
                        <div>
                            <h3>{t!("lectionary.epistle")}</h3>
                            {async_readings_view(&self.locale, self.epistle)}
                        </div>
                    })
                }}

                // Gospel
                <h3>{t!("lectionary.gospel")}</h3>
                {async_readings_view(&self.locale, self.gospel)}
            </div>
        }
    }

    fn error_boundary(_error: RouterError) -> Body
    where
        Self: Sized,
    {
        view! {
            <div>
                <p class="redirect-links">
                    {t!("lectionary.no_readings_lff")}
                    " "
                    <a href="../../calendar/lff2018">
                        {t!("menu.calendar")}
                    </a>
                    {t!("lectionary.no_readings_end")}
                </p>
            </div>
        }
    }
}

impl HolyDayView {
    fn reading_links(&self) -> Node {
        if self.first_lesson.is_empty() {
            view! {
                <ul class="reading-links">
                    {EucharistView::reading_link(&self.epistle)}
                    {Self::psalm_link(&self.psalm)}
                    {EucharistView::reading_link(&self.gospel)}
                </ul>
            }
        } else {
            view! {
                <ul class="reading-links">
                    {EucharistView::reading_link(&self.first_lesson)}
                    {Self::psalm_link(&self.psalm)}
                    {EucharistView::reading_link(&self.epistle)}
                    {EucharistView::reading_link(&self.gospel)}
                </ul>
            }
        }
    }

    fn psalm_link(psalms: &[Psalm]) -> Node {
        // necessary awkwardness to avoid Itertools conflict for now
        let links = std::iter::Iterator::intersperse_with(
            psalms
                .iter()
                .map(|psalm| {
                    psalm.citation.clone().unwrap_or_else(|| {
                        t!("daily_readings.psalm", number = &psalm.number.to_string())
                    })
                })
                .map(|citation| view! { <a href={format!("#{}", citation)}>{citation}</a> }),
            || text(format!(" {} ", t!("or"))),
        )
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
                            <a id={doc.citation.clone().unwrap_or_default()}></a>
                            <h3>{t!("lectionary.psalm")}</h3>
                            <article class="document">
                                {psalm(doc).1}
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
                    doc.citation
                        .clone()
                        .unwrap_or_else(|| t!("daily_readings.option", n = &idx.to_string()))
                })
                .collect::<Vec<_>>();
            let psalms = self.psalm.iter().map(|doc| {
                view! {
                    <div>
                        <a id={doc.citation.clone().unwrap_or_default()}></a>
                        <h3>{t!("lectionary.psalm")}</h3>
                        <article class="document">
                            {psalm(doc).1}
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
