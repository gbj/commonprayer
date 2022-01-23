use crate::{components::*, utils::language::locale_to_language};
use episcopal_api::{
    calendar::{lff2018::LFF_BIOS, HolyDayId, LiturgicalDayId, Time, LFF2018_CALENDAR},
    lectionary::{ReadingType, LFF2018_LECTIONARY},
    library::{lff2018::collects::*, CollectId},
    liturgy::{BiblicalCitation, Choice, Content, Document},
    psalter::bcp1979::BCP1979_PSALTER,
};
use leptos::*;
use rust_i18n::t;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HolyDayProps {
    date: String,
    locale: String,
    name: String,
    bio: String,
    collect_traditional: Document,
    collect_contemporary: Document,
    first_lesson: Document,
    psalm: Document,
    gospel: Document,
}

pub fn holy_day() -> Page<HolyDayProps, ()> {
    Page::new("holy-day")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(static_props)
        .build_paths_fn(build_paths_fn)
}

fn head(_locale: &str, props: &HolyDayProps) -> View {
    let title = format!("{} – {}", props.name, t!("common_prayer"));
    view! {
        <>
            <title>{title}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
            <link rel="stylesheet" href="/static/holy-day.css"/>
        </>
    }
}

fn body(locale: &str, props: &HolyDayProps) -> View {
    let bio = View::Fragment(
        props
            .bio
            .split("\n\n")
            .map(|para| {
                view! {
                  <p>{para}</p>
                }
            })
            .collect(),
    );

    let first_lesson_citation = match &props.first_lesson.content {
        Content::BiblicalCitation(citation) => citation.to_string(),
        Content::BiblicalReading(reading) => reading.citation.clone(),
        _ => String::default(),
    };

    let psalm_citation = match &props.psalm.content {
        Content::PsalmCitation(citation) => citation.to_string(),
        Content::Psalm(psalm) => psalm.citation.clone().unwrap_or_default(),
        _ => String::default(),
    };

    let gospel_citation = match &props.gospel.content {
        Content::BiblicalCitation(citation) => citation.to_string(),
        Content::BiblicalReading(reading) => reading.citation.clone(),
        _ => String::default(),
    };

    let collect_view = DocumentController::new(Document::from(Choice::from([
        props.collect_contemporary.clone(),
        props.collect_traditional.clone(),
    ])))
    .view(locale);

    let header_title = format!(
        "{}: {}",
        props.date,
        props.name.split(',').next().unwrap_or(props.name.as_str())
    );

    view! {
        <>
            {header(locale, &header_title)}
            <main>
                <h1>{format!("{}: {}", props.date, props.name)}</h1>

                // Collects
                <dyn:view view={collect_view}/>

                // Reading Links
                <ul class="reading-links">
                    <li><a href="#first-lesson">{first_lesson_citation}</a></li>
                    <li><a href="#psalm">{psalm_citation}</a></li>
                    <li><a href="#gospel">{gospel_citation}</a></li>
                </ul>
                <hr/>

                // Bio
                {bio}
                <hr/>

                // Actual readings
                <h2>{t!("holy_day.lessons_and_psalm")}</h2>
                <a id="first-lesson"></a>
                <h3>{t!("holy_day.first_lesson")}</h3>
                <dyn:view view={DocumentController::new(props.first_lesson.clone()).view(locale)}/>
                <a id="psalm"></a>
                <h3>{t!("holy_day.psalm")}</h3>
                <dyn:view view={DocumentController::new(props.psalm.clone()).view(locale)}/>
                <a id="gospel"></a>
                <h3>{t!("holy_day.gospel")}</h3>
                <dyn:view view={DocumentController::new(props.gospel.clone()).view(locale)}/>

            </main>
        </>
    }
}

fn static_props(locale: &str, path: &str, _params: ()) -> HolyDayProps {
    let language = locale_to_language(locale);
    let mut parts = path.split('/');
    parts.next(); // /
    parts.next(); // /{locale}
    parts.next(); // /holy-day
    let feast = parts
        .next()
        .map(|s| serde_json::from_str(&format!("\"{}\"", s)).unwrap())
        .unwrap();
    let date = LFF2018_CALENDAR
        .holy_days
        .iter()
        .find(|(_, s_feast, _, _)| *s_feast == feast)
        .map(|(id, _, _, _)| match id {
            HolyDayId::Date(month, day) => format!("{} {}", language.month_name(*month), day),
            _ => panic!("expected a month/date pair for feast"),
        })
        .expect("could not find info for feast");
    let name = LFF2018_CALENDAR
        .feast_name(feast, language)
        .unwrap()
        .to_string();
    let bio = LFF_BIOS
        .iter()
        .find(|(s_feast, _)| *s_feast == feast)
        .map(|(_, bio)| bio.to_string())
        .unwrap();
    let readings = LFF2018_LECTIONARY
        .readings
        .iter()
        .filter(|(id, _, _, _)| id == &LiturgicalDayId::Feast(feast))
        .map(|(_, _, reading_type, citation)| (*reading_type, citation.to_string()))
        .collect::<Vec<_>>();

    let first_lesson = filter_readings(&readings, ReadingType::FirstReading);
    let psalm = filter_readings(&readings, ReadingType::Psalm);
    let gospel = filter_readings(&readings, ReadingType::Gospel);

    let collect_traditional = LFF_COLLECTS_TRADITIONAL
        .iter()
        .find(|(id, _)| *id == CollectId::Feast(feast))
        .map(|(_, doc)| doc.clone())
        .unwrap_or_else(|| Document::from(Content::Empty));

    let collect_contemporary = LFF_COLLECTS_CONTEMPORARY
        .iter()
        .find(|(id, _)| *id == CollectId::Feast(feast))
        .map(|(_, doc)| doc.clone())
        .unwrap_or_else(|| Document::from(Content::Empty));

    HolyDayProps {
        date,
        locale: locale.to_string(),
        name,
        bio,
        first_lesson,
        psalm,
        gospel,
        collect_traditional,
        collect_contemporary,
    }
}

fn build_paths_fn() -> Vec<String> {
    LFF2018_CALENDAR
        .holy_days
        .iter()
        .filter(|(_, _, time, _)| *time != Time::EveningOnly)
        .map(|(_, feast, _, _)| serde_json::to_string(feast).unwrap().replace('"', ""))
        .collect()
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
