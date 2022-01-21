use std::collections::HashMap;

use crate::{components::*, utils::time::today, TABLE_OF_CONTENTS};
use episcopal_api::{
    calendar::{Calendar, Date},
    library::{CommonPrayer, Library},
    liturgy::{Content, Document, PreferenceKey, PreferenceValue, Version},
};
use futures::StreamExt;
use leptos::*;
use rust_i18n::t;
use serde::Serialize;
use serde_derive::Deserialize;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Deserialize, Clone)]
pub struct DocumentPageParams {
    pub category: String,
    pub slug: String,
    pub version: Version,
    pub date: Option<Date>,
    pub calendar: Option<String>,
    pub prefs: Option<String>, // a JSON-serialized HashMap<PreferenceKey, PreferenceValue>
    pub alternate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentPageProps {
    pub doc: Document,
    pub api_path: String,
    base_path: String,
}

pub fn document() -> Page<DocumentPageProps, DocumentPageParams> {
    Page::new("document")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(get_static_props)
        .build_paths_fn(get_static_paths)
}

pub fn get_static_paths() -> Vec<String> {
    vec![
        "{category}/{slug}/{version}".into(),
        "{category}/{slug}/{version}/{date}".into(),
        "{category}/{slug}/{version}/{date}/{calendar}".into(),
        "{category}/{slug}/{version}/{date}/{calendar}/{prefs}".into(),
        "{category}/{slug}/{version}/{date}/{calendar}/{prefs}/{alternate}".into(),
    ]
}

pub fn head(_locale: &str, props: &DocumentPageProps) -> View {
    let title = match &props.doc.label {
        Some(label) => format!("{} – {}", label, t!("common_prayer")),
        None => t!("common_prayer"),
    };

    view! {
        <>
            <title>{title}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
        </>
    }
}

pub fn body(locale: &str, props: &DocumentPageProps) -> View {
    let doc = &props.doc;

    let title = match &doc.label {
        Some(label) => label.clone(),
        None => t!("common_prayer"),
    };

    let date_picker = DatePicker::new(t!("date"), None);
    let base_path = props.base_path.clone();
    date_picker
        .date
        .stream()
        // skip the first value, because the initial value of the input will
        // always be emitted but has already been reflected in the page
        .skip(1)
        .create_effect(move |date| {
            if let Some(date) = date {
                location()
                    .set_href(&format!("{}/{}", base_path, date))
                    .unwrap_throw();
            } else {
                location().set_href(&base_path).unwrap_throw();
            }
        });

    let side_menu = if doc.has_date_condition() {
        side_menu(
            Icon::Calendar,
            view! {
                <section class="preview-menu">
                    <dyn:view view={date_picker.view()}/>
                </section>
            },
        )
    } else {
        View::Empty
    };

    view! {
        <>
            {header_with_side_menu(locale, &title, side_menu)}
            <main>
                // TODO clean up ugly export links
                <ul class="export-links">
                    <a class="link" href="#">
                        <img src="/static/icons/tabler-icon-link.svg"/>
                        {t!("export.link")}
                    </a>
                    <a class="embed" href="#">
                        <img src="/static/icons/tabler-icon-code.svg"/>
                        {t!("export.embed")}
                    </a>
                    <a class="word" download href={&format!("{}.docx", &props.api_path)}>
                        <img src="/static/icons/file-word-regular.svg"/>
                        {t!("export.word")}
                    </a>
                    <a class="venite" href="#">
                        <img src="/static/icons/venite.svg"/>
                        {t!("export.venite")}
                    </a>
                    <a class="json" download href={&format!("{}.json", &props.api_path)}>
                        <img src="/static/icons/tabler-icon-download.svg"/>
                        {t!("export.json")}
                    </a>
                </ul>
                <dyn:view view={document_view(locale, doc)}/>
            </main>
        </>
    }
}

pub fn get_static_props(
    locale: &str,
    _path: &str,
    params: DocumentPageParams,
) -> DocumentPageProps {
    let api_path = format!(
        "/api/export/{}/{:#?}/{}/{}/{}/{}/{}",
        params.category,
        params.version,
        params.date.unwrap_or_else(today),
        params
            .calendar
            .clone()
            .unwrap_or_else(|| String::from("bcp1979")),
        urlencoding::encode(&params.prefs.clone().unwrap_or_default()),
        params
            .alternate
            .clone()
            .unwrap_or_else(|| String::from("observed")),
        params.slug
    );

    let doc = TABLE_OF_CONTENTS
        .iter()
        .flat_map(|(category, docs)| {
            docs.iter()
                .map(move |(slug, _, doc)| (category.clone(), slug.clone(), doc.version, doc))
        })
        .find(|(category, slug, version, _)| {
            category == &params.category && slug == &params.slug && version == &params.version
        })
        .map(|(_, _, _, doc)| doc.clone())
        .expect("could not find document");

    let doc = if let Some(date) = params.date {
        let calendar = params
            .calendar
            .map(|slug| Calendar::from(slug.as_str()))
            .unwrap_or_default();

        let evening = if let Content::Liturgy(liturgy) = &doc.content {
            liturgy.evening
        } else {
            false
        };

        let day = calendar.liturgical_day(date, evening);

        let observed = params
            .alternate
            .map(|slug| {
                if slug == "alternate" {
                    day.alternate.unwrap_or(day.observed)
                } else {
                    day.observed
                }
            })
            .unwrap_or(day.observed);

        let prefs: HashMap<PreferenceKey, PreferenceValue> = params
            .prefs
            // this strange indirection is necessary because serde_json can't use structs/enums as map keys
            // (due to JSON format limitations)
            .and_then(|json| {
                serde_json::from_str::<Vec<(PreferenceKey, PreferenceValue)>>(&json).ok()
            })
            .unwrap_or_default()
            .into_iter()
            .collect();

        if let Content::Liturgy(liturgy) = &doc.content {
            CommonPrayer::compile(
                doc.clone(),
                &calendar,
                &day,
                &observed,
                &prefs,
                &liturgy.preferences,
            )
            .unwrap()
        } else {
            doc
        }
    } else {
        doc
    };

    DocumentPageProps {
        doc,
        base_path: format!(
            "/{}/document/{}/{}/{:#?}",
            locale, params.category, params.slug, params.version
        ),
        api_path,
    }
}
