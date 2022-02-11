use std::collections::HashMap;

use crate::{components::*, table_of_contents::PageType, TABLE_OF_CONTENTS};
use episcopal_api::{
    calendar::{Calendar, Date},
    library::{CommonPrayer, Library},
    liturgy::{Content, Document, PreferenceKey, PreferenceValue, Version},
};
use futures::StreamExt;
use itertools::Itertools;
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
    pub page_type: PageType,
    base_path: String,
    slug: String,
    date: String,
}

pub fn document() -> Page<DocumentPageProps, DocumentPageParams, ()> {
    Page::new("document")
        .head_fn(head)
        .body_fn(body)
        .hydration_state(hydration_state)
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

pub fn head(_locale: &str, props: &DocumentPageProps, _render_state: &()) -> View {
    let label = match &props.page_type {
        PageType::Document(doc) => doc.label.as_ref(),
        PageType::Category(label, _, _) => Some(label),
    };
    let title = match label {
        Some(label) => format!("{} – {}", label, t!("common_prayer")),
        None => t!("common_prayer"),
    };

    view! {
        <>
            <title>{title}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
            {analytics()}
        </>
    }
}

pub fn body(locale: &str, props: &DocumentPageProps, _render_state: &()) -> View {
    let title = match &props.page_type {
        PageType::Document(doc) => match &doc.label {
            Some(label) => label.clone(),
            None => t!("common_prayer"),
        },
        PageType::Category(label, _, _) => label.clone(),
    };

    match &props.page_type {
        PageType::Document(doc) => document_body(locale, props, title, doc),
        PageType::Category(_, _, docs) => category_body(locale, props, title, docs),
    }
}

fn document_body(locale: &str, props: &DocumentPageProps, title: String, doc: &Document) -> View {
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

    let document_controller = DocumentController::new(doc.clone());

    view! {
        <>
            {header_with_side_menu(locale, &title, side_menu)}
            <main>
                //<dyn:view view={export_links(&props.slug, &props.date, &document_controller)} />
                <dyn:view view={document_controller.view(locale)}/>
            </main>
        </>
    }
}

fn category_body(
    locale: &str,
    _props: &DocumentPageProps,
    title: String,
    docs: &[Document],
) -> View {
    let search = SearchBar::new();

    let categories = View::Fragment(
        docs.iter()
            .group_by(|doc| doc.label.clone())
            .into_iter()
            .map(|(label, group)| {
                let docs = group.cloned().collect::<Vec<_>>();

                let docs_view = View::Fragment(
                    docs.iter()
                        .map(|doc| {
                            let hidden = search
                                .value
                                .stream()
                                .map({
                                    let doc = doc.clone();
                                    move |search| {
                                        !search.is_empty()
                                            && !doc.contains_case_insensitive(&search)
                                    }
                                })
                                .boxed_local();

                            let doc = DocumentController::new(Document {
                                label: None, // don't show the label again for each doc
                                ..doc.clone()
                            })
                            .view(locale);

                            view! {
                                <dyn:article class="document" class:hidden={hidden}>
                                    {doc}
                                </dyn:article>
                            }
                        })
                        .collect(),
                );

                let label = if let Some(label) = label {
                    let hidden = search
                        .value
                        .stream()
                        .map({
                            let label = label.to_lowercase();
                            move |search| {
                                !label.contains(&search.to_lowercase())
                                    && !docs
                                        .iter()
                                        .any(|doc| doc.contains_case_insensitive(&search))
                            }
                        })
                        .boxed_local();
                    view! {
                        <dyn:h3 class:hidden={hidden}>{&label}</dyn:h3>
                    }
                } else {
                    View::Empty
                };

                view! {
                    <>
                        {label}
                        {docs_view}
                    </>
                }
            })
            .collect::<Vec<_>>(),
    );

    view! {
        <>
            {header(locale, &title)}
            <main>
                <dyn:view view={search.view()} />
                <dyn:view view={categories} />
            </main>
        </>
    }
}

fn find_page(category: &str, slug: &str, version: Option<Version>) -> Option<PageType> {
    TABLE_OF_CONTENTS
        .iter()
        .flat_map(|(category, docs)| {
            docs.iter().map(move |(slug, page_type)| {
                let version = match &page_type {
                    PageType::Document(doc) => doc.version,
                    PageType::Category(_, version, _) => *version,
                };
                (category.clone(), slug.clone(), version, page_type)
            })
        })
        .find(|(s_category, s_slug, s_version, _)| {
            s_category == category
                && s_slug == slug
                && (version.is_none() || version == Some(*s_version))
        })
        .map(|(_, _, _, page_type)| page_type.clone())
}

pub fn hydration_state(
    locale: &str,
    _path: &str,
    params: &DocumentPageParams,
) -> Option<DocumentPageProps> {
    // find page in TOC, either with the given version or in any version
    find_page(&params.category, &params.slug, Some(params.version))
        .or_else(|| find_page(&params.category, &params.slug, None))
        // if document is not found, with return None => 404
        .map(|page_type| {
            // if it's a PageType::Document and we're given a date, compile it
            let page_type = match (&page_type, &params.date) {
                (PageType::Category(_, _, _), _) => page_type,
                (PageType::Document(_), None) => page_type,
                (PageType::Document(doc), Some(date)) => {
                    let calendar = params
                        .calendar
                        .as_ref()
                        .map(|slug| Calendar::from(slug.as_str()))
                        .unwrap_or_default();

                    let evening = if let Content::Liturgy(liturgy) = &doc.content {
                        liturgy.evening
                    } else {
                        false
                    };

                    let day = calendar.liturgical_day(*date, evening);

                    let observed = params
                        .alternate
                        .as_ref()
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
                        .as_ref()
                        // this strange indirection is necessary because serde_json can't use structs/enums as map keys
                        // (due to JSON format limitations)
                        .and_then(|json| {
                            serde_json::from_str::<Vec<(PreferenceKey, PreferenceValue)>>(json).ok()
                        })
                        .unwrap_or_default()
                        .into_iter()
                        .collect();

                    let doc = if let Content::Liturgy(liturgy) = &doc.content {
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
                        doc.clone()
                    };

                    PageType::Document(doc)
                }
            };

            DocumentPageProps {
                page_type,
                base_path: format!(
                    "/{}/document/{}/{}/{:#?}",
                    locale, params.category, params.slug, params.version
                ),
                slug: params.slug.clone(),
                date: params.date.map(|date| date.to_string()).unwrap_or_default(),
            }
        })
}
