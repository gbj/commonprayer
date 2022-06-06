use crate::{
    preferences,
    views::{document::DocumentView, Header},
    WebView,
};
use calendar::{
    lff2018::LFF_BIOS, Feast, HolyDayId, LiturgicalDayId, Time, BCP1979_CALENDAR, LFF2018_CALENDAR,
};
use language::Language;
use lectionary::{ReadingType, LFF2018_LECTIONARY, RCL};
use leptos2::*;
use library::{lff2018::collects::*, CollectId};
use liturgy::{
    BiblicalCitation, Choice, Content, Document, GlobalPref, PreferenceKey, PreferenceValue,
    Version,
};
use psalter::bcp1979::BCP1979_PSALTER;
use rust_i18n::t;
use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct HolyDayParams {
    feast: String,
}

#[derive(Clone)]
pub struct HolyDayPage {
    date: Option<String>,
    locale: String,
    name: String,
    bio: Option<String>,
    collect_traditional: Document,
    collect_contemporary: Document,
    first_lesson: Document,
    psalm: Document,
    epistle: Document,
    gospel: Document,
}

impl Page for HolyDayPage {
    type Params = HolyDayParams;
    type Query = ();

    fn name() -> &'static str {
        "holy-day"
    }

    fn paths() -> Vec<String> {
        vec!["{feast}".into()]
    }

    fn build_state(
        locale: &str,
        _path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        // deserializing here instead of in the params means that
        // a bad feast ID will be a 404 error (finds path, but branches here to None => 404)
        // not a server error (when it's unable to find path b/c can't deserialize to Feast)

        serde_json::from_str::<Feast>(&format!(r#""{}""#, params.feast))
            .ok()
            .and_then(|feast| {
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
                    .collect::<Vec<_>>();

                let first_lesson = filter_readings(&readings, ReadingType::FirstReading);
                let psalm = filter_readings(&readings, ReadingType::Psalm);
                let epistle = filter_readings(&readings, ReadingType::SecondReading);
                let gospel = filter_readings(&readings, ReadingType::Gospel);

                let collect_traditional = LFF_COLLECTS_TRADITIONAL
                    .iter()
                    .find(|(id, _)| {
                        *id == CollectId::Feast(feast)
                            || Some(id) == eve_of.map(CollectId::Feast).as_ref()
                    })
                    .map(|(_, data)| data.document.clone())
                    .unwrap_or_else(|| Document::from(Content::Empty));

                let collect_contemporary = LFF_COLLECTS_CONTEMPORARY
                    .iter()
                    .find(|(id, _)| {
                        *id == CollectId::Feast(feast)
                            || Some(id) == eve_of.map(CollectId::Feast).as_ref()
                    })
                    .map(|(_, data)| data.document.clone())
                    .unwrap_or_else(|| Document::from(Content::Empty));

                Some(HolyDayPage {
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
            })
    }

    fn head(&self, _locale: &str) -> Vec<Node> {
        let title = format!("{} – {}", self.name, t!("common_prayer"));
        view! {
            <>
                <title>{title}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/document.css"/>
                <link rel="stylesheet" href="/static/holy-day.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        let bio = if let Some(bio) = &self.bio {
            let mut bio = bio
                .split("\n\n")
                .map(|para| {
                    view! {
                      <p>{para}</p>
                    }
                })
                .collect::<Vec<_>>();
            bio.push(view! { <hr/> });
            bio
        } else {
            Vec::new()
        };

        let first_lesson_citation = content_to_citation(&self.first_lesson.content);
        let psalm_citation = content_to_citation(&self.psalm.content);
        let epistle_citation = content_to_citation(&self.epistle.content);
        let gospel_citation = content_to_citation(&self.gospel.content);

        let collect_view = DocumentView {
            doc: &Document::from(Choice::from([
                self.collect_contemporary.clone(),
                self.collect_traditional.clone(),
            ])),
            path: vec![],
        }
        .view(locale);

        let name_primary = self
            .name
            .split(',')
            .next()
            .unwrap_or(self.name.as_str())
            .replace('[', "");
        let header_title = if let Some(date) = &self.date {
            format!("{}: {}", date, name_primary)
        } else {
            name_primary
        };

        let bible_version = preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion))
            .and_then(|value| match value {
                PreferenceValue::Version(version) => Some(version),
                _ => None,
            })
            .unwrap_or(Version::NRSV);

        view! {
            <>
                {Header::new(locale, &header_title).to_node()}
                <main>
                    <h1>{format!("{}{}{}", self.date.clone().unwrap_or_default(), if self.date.is_some() { ": " } else { "" }, self.name)}</h1>

                    // Collects
                    {collect_view}

                    // Reading Links
                    <ul class="reading-links">
                        <li><a href="#first-lesson">{first_lesson_citation}</a></li>
                        <li><a href="#psalm">{psalm_citation}</a></li>
                        {if self.epistle.content != Content::Empty {
                            Some(view! {
                                <li><a href="#epistle">{epistle_citation}</a></li>
                            })
                        } else {
                            None
                        }}
                        <li><a href="#gospel">{gospel_citation}</a></li>
                    </ul>
                    <hr/>

                    // Bio
                    {bio}

                    // Actual readings
                    <h2>{t!("lectionary.lessons_and_psalm")}</h2>

                    // First Lesson
                    <a id="first-lesson"></a>
                    <h3>{t!("lectionary.first_lesson")}</h3>
                    { DocumentView { doc: &self.first_lesson.clone().version(bible_version), path: vec![] }.view(locale) }

                    // Psalm
                    <a id="psalm"></a>
                    <h3>{t!("lectionary.psalm")}</h3>
                    { DocumentView { doc: &self.psalm.clone(), path: vec![] }.view(locale) }

                    // Epistle
                    {if self.epistle.content != Content::Empty {
                        Some(view! {
                            <a id="epistle"></a>
                            <h3>{t!("lectionary.epistle")}</h3>
                            { DocumentView { doc: &self.epistle.clone().version(bible_version), path: vec![] }.view(locale) }
                        })
                    } else {
                        None
                    }}

                    // Gospel
                    <a id="gospel"></a>
                    <h3>{t!("lectionary.gospel")}</h3>
                    { DocumentView { doc: &self.gospel.clone().version(bible_version), path: vec![] }.view(locale) }
                </main>
            </>
        }
    }
}

fn filter_readings(readings: &[(ReadingType, String)], reading_type: ReadingType) -> Document {
    let filtered = readings
        .iter()
        .filter(|(s_reading_type, _)| *s_reading_type == reading_type)
        .collect::<Vec<_>>();

    let document = if reading_type == ReadingType::Psalm {
        if is_server!() {
            Document::choice_or_document(&mut filtered.into_iter().flat_map(|(_, citation)| {
                BCP1979_PSALTER
                    .psalms_by_citation(citation.as_str())
                    .iter()
                    .map(|psalm| Document::from(psalm.clone()))
                    .collect::<Vec<_>>()
            }))
        } else {
            None
        }
    } else {
        Document::choice_or_document(
            &mut filtered
                .into_iter()
                .map(|(_, citation)| Document::from(BiblicalCitation::from(citation.clone()))),
        )
    };

    document.unwrap_or_else(|| Document::from(Content::Empty))
}

fn content_to_citation(content: &Content) -> String {
    match content {
        Content::BiblicalCitation(citation) => citation.to_string(),
        Content::BiblicalReading(reading) => reading.citation.clone(),
        Content::PsalmCitation(citation) => citation.to_string(),
        Content::Psalm(psalm) => psalm.citation.clone().unwrap_or_default(),
        Content::Choice(choice) => choice
            .options
            .iter()
            .map(|doc| content_to_citation(&doc.content))
            .intersperse(format!(" {} ", t!("daily_readings.or")))
            .collect(),
        _ => String::default(),
    }
}
