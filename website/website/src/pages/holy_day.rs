use crate::{components::*, utils::language::locale_to_language};
use episcopal_api::{
    calendar::{
        lff2018::LFF_BIOS, Feast, HolyDayId, LiturgicalDayId, Time, BCP1979_CALENDAR,
        LFF2018_CALENDAR,
    },
    lectionary::{ReadingType, LFF2018_LECTIONARY},
    library::{lff2018::collects::*, CollectId},
    liturgy::{BiblicalCitation, Choice, Content, Document},
    psalter::bcp1979::BCP1979_PSALTER,
};
use leptos::*;
use rust_i18n::t;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HolyDayParams {
    feast: Feast,
}

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

pub fn holy_day() -> Page<HolyDayProps, HolyDayParams> {
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

    let first_lesson_citation = content_to_citation(&props.first_lesson.content);
    let psalm_citation = content_to_citation(&props.psalm.content);
    let gospel_citation = content_to_citation(&props.gospel.content);

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
                <article class="document">
                    <dyn:view view={DocumentController::new(props.first_lesson.clone()).view(locale)}/>
                </article>
                <a id="psalm"></a>
                <h3>{t!("holy_day.psalm")}</h3>
                <article class="document">
                    <dyn:view view={DocumentController::new(props.psalm.clone()).view(locale)}/>
                </article>
                <a id="gospel"></a>
                <h3>{t!("holy_day.gospel")}</h3>
                <article class="document">
                    <dyn:view view={DocumentController::new(props.gospel.clone()).view(locale)}/>
                </article>

            </main>
        </>
    }
}

fn static_props(locale: &str, _path: &str, params: HolyDayParams) -> HolyDayProps {
    let language = locale_to_language(locale);
    let feast = params.feast;
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
        .map(|(id, _, _, _)| match id {
            HolyDayId::Date(month, day) => format!("{} {}", language.month_name(*month), day),
            _ => panic!("expected a month/date pair for feast"),
        })
        .expect("could not find info for feast");
    let name = LFF2018_CALENDAR
        .feast_name(feast, language)
        // or, search in BCP calendar if can't find in LFF (i.e., for Eve of ___)
        .or_else(|| BCP1979_CALENDAR.feast_name(feast, language))
        .expect("could not find feast name for feast")
        .to_string();
    let bio = LFF_BIOS
        .iter()
        .find(|(s_feast, _)| *s_feast == feast || Some(*s_feast) == eve_of)
        .map(|(_, bio)| bio.to_string())
        .expect("could not find bio for feast");
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
        .find(|(id, _)| {
            *id == CollectId::Feast(feast) || Some(id) == eve_of.map(CollectId::Feast).as_ref()
        })
        .map(|(_, doc)| doc.clone())
        .unwrap_or_else(|| Document::from(Content::Empty));

    let collect_contemporary = LFF_COLLECTS_CONTEMPORARY
        .iter()
        .find(|(id, _)| {
            *id == CollectId::Feast(feast) || Some(id) == eve_of.map(CollectId::Feast).as_ref()
        })
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
    vec!["{feast}".into()]
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
