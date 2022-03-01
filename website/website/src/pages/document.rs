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

#[derive(Debug, Deserialize, Clone)]
pub struct DocumentPageParams {
    pub category: String,
    pub slug: Option<String>,
    pub version: Option<Version>,
    pub date: Option<Date>,
    pub calendar: Option<String>,
    pub prefs: Option<String>, // a JSON-serialized HashMap<PreferenceKey, PreferenceValue>
    pub alternate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DocumentPageType {
    Document(Box<Document>),
    Category(String, Version, Vec<Document>),
    CategorySummary(String, String, Vec<(Version, String)>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentPageProps {
    pub page_type: DocumentPageType,
    base_path: String,
    slug: Option<String>,
    date: String,
}

pub fn document() -> Page<DocumentPageProps, DocumentPageParams, ()> {
    Page::new("document")
        .head_fn(head)
        .body_fn(body)
        .hydration_state(hydration_state)
        .build_paths_fn(get_static_paths)
        .incremental_generation()
}

pub fn get_static_paths() -> Vec<String> {
    vec![
        "{category}".into(),
        "{category}/{version}".into(),
        "{category}/{slug}/{version}".into(),
        "{category}/{slug}/{version}/{date}".into(),
        "{category}/{slug}/{version}/{date}/{calendar}".into(),
        "{category}/{slug}/{version}/{date}/{calendar}/{prefs}".into(),
        "{category}/{slug}/{version}/{date}/{calendar}/{prefs}/{alternate}".into(),
    ]
}

pub fn head(_locale: &str, props: &DocumentPageProps, _render_state: &()) -> View {
    let label = match &props.page_type {
        DocumentPageType::Document(doc) => doc.label.as_ref(),
        DocumentPageType::Category(label, _, _) => Some(label),
        DocumentPageType::CategorySummary(label, _, _) => Some(label),
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
        </>
    }
}

pub fn body(locale: &str, props: &DocumentPageProps, _render_state: &()) -> View {
    let title = match &props.page_type {
        DocumentPageType::Document(doc) => match &doc.label {
            Some(label) => label.clone(),
            None => t!("common_prayer"),
        },
        DocumentPageType::Category(label, _, _) => label.clone(),
        DocumentPageType::CategorySummary(label, _, _) => label.clone(),
    };

    match &props.page_type {
        DocumentPageType::Document(doc) => document_body(locale, props, title, doc),
        DocumentPageType::Category(_, _, docs) => category_body(locale, props, title, docs),
        DocumentPageType::CategorySummary(label, slug, pages) => {
            category_summary_body(locale, label, slug, pages)
        }
    }
}

fn category_summary_body(
    locale: &str,
    label: &str,
    category: &str,
    pages: &[(Version, String)],
) -> View {
    let title = label;
    let pages = View::Fragment(
        pages
            .iter()
            .map(|(version, label)| {
                view! {
                    <li><a href={format!("/{}/document/{}/{:?}", locale, category, version)}>{label}</a></li>
                }
            })
            .collect(),
    );
    view! {
        <>
            {header(locale, title)}
            <main>
                <ul class="category-summary">
                    {pages}
                </ul>
            </main>
        </>
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

    // grouped by category (tags[0]), subcategory (tags[1]), then label
    let tree = docs
        .iter()
        .group_by(|doc| doc.tags.get(0))
        .into_iter()
        .map(|(category, docs_with_category)| {
            (
                category,
                docs_with_category
                    .into_iter()
                    .group_by(|doc| doc.tags.get(1))
                    .into_iter()
                    .map(|(subcategory, docs_with_subcategory)| {
                        (
                            subcategory,
                            docs_with_subcategory
                                .into_iter()
                                .group_by(|doc| doc.label.as_ref())
                                .into_iter()
                                .map(|(label, docs_with_label)| {
                                    (label, docs_with_label.into_iter().collect::<Vec<_>>())
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let links = if tree.len() > 1 {
        let categories = View::Fragment(
            tree.iter()
                .map(|(category, docs_with_category)| {
                    let subcategories = if docs_with_category.len() > 1 {
                        View::Fragment(
                            docs_with_category
                                .iter()
                                .map(|(subcategory, docs_with_subcategory)| {
                                    let labels = if docs_with_subcategory.len() > 1 {
                                        View::Fragment(docs_with_subcategory
                                            .iter()
                                            .map(|(label, _)| view! {
                                                <li> <a href={format!("#{}", label.cloned().unwrap_or_default())}>{label.cloned().unwrap_or_default()}</a></li>
                                            })
                                            .collect()
                                        )
                                    } else {
                                        View::Empty
                                    };

                                    view! {
                                        <li>
                                             <a href={format!("#{}", subcategory.cloned().unwrap_or_default())}>{subcategory.cloned().unwrap_or_default()}</a>
                                            <ul>{labels}</ul>
                                        </li>
                                    }
                                })
                                .collect(),
                            )
                    } else {
                        View::Empty
                    };

                    view! {
                        <li>
                            <a href={format!("#{}", category.cloned().unwrap_or_default())}>{category.cloned().unwrap_or_default()}</a>
                            <ul>{subcategories}</ul>
                        </li>
                    }
                })
                .collect(),
        );

        view! {
            <ul>
                {categories}
            </ul>
        }
    } else {
        View::Empty
    };

    // grouped by category (tags[0]), subcategory (tags[1]), then label
    let categories = View::Fragment(
        tree.iter()
            .map(|(category, subcategories)| {
                let subcategories = View::Fragment(
                    subcategories
                        .iter()
                        .map(|(subcategory, docs_with_subcategory)| {
                            let labels = View::Fragment(docs_with_subcategory.iter().map(|(label, docs_with_label)| {
                                        let docs = docs_with_label.iter().cloned().cloned().collect::<Vec<_>>();
                                        let subtitle =
                                            docs.get(0).and_then(|doc| doc.subtitle.clone());

                                            let docs_view = View::Fragment(
                                                docs.iter()
                                                    .map(|doc| {
                                                        let hidden = search
                                                            .value
                                                            .stream()
                                                            .map({
                                                                let doc = (*doc).clone();
                                                                move |search| {
                                                                    !search.is_empty()
                                                                        && !doc.contains_case_insensitive(&search)
                                                                }
                                                            })
                                                            .boxed_local();

                                                        let doc = DocumentController::new(Document {
                                                            label: None,    // don't show the label again for each doc
                                                            subtitle: None, // don't show subtitle again for every doc
                                                            ..(*doc).clone()
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
                                            if let Some(subtitle) = subtitle {
                                                view! {
                                                    <dyn:div class="label-and-subtitle" class:hidden={hidden}>
                                                        <a id={label.to_string()}></a>
                                                        <h4>{label.to_string()}</h4>
                                                        <h5 class="subtitle">{subtitle}</h5>
                                                    </dyn:div>
                                                }
                                            } else {
                                                view! {
                                                    <>
                                                        <a id={label.to_string()}></a>
                                                        <dyn:h4 class:hidden={hidden}>{label.to_string()}</dyn:h4>
                                                    </>
                                                }
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
                                    .collect());

                            view! {
                                <>
                                    {if let Some(subcategory) = subcategory {
                                        view! {
                                            <>
                                                <a id={subcategory.to_string()}></a>
                                                <h3>{subcategory.to_string()}</h3>
                                            </>
                                        }
                                    } else {
                                        View::Empty
                                    }}
                                    {labels}
                                </>
                            }
                        })
                        .collect(),
                );

                view! {
                    <>
                        {if let Some(category) = category {
                            view! {
                                <>
                                    <a id={category.to_string()}></a>
                                    <h2>{category.to_string()}</h2>
                                </>
                            }
                        } else {
                            View::Empty
                        }}
                        {subcategories}
                    </>
                }
            })
            .collect(),
    );

    view! {
        <>
            {header(locale, &title)}
            <main>
                <dyn:view view={search.view()} />
                {links}
                <dyn:view view={categories} />
            </main>
        </>
    }
}

fn find_page(
    category: &str,
    slug: &Option<String>,
    version: Option<Version>,
) -> Option<DocumentPageType> {
    let pages = TABLE_OF_CONTENTS.get(&(category, version));
    let mut pages = if let Some(pages) = pages {
        pages.clone()
    } else {
        Vec::new()
    };

    if version.is_some() {
        if let Some(pages_without_version) = TABLE_OF_CONTENTS.get(&(category, None)) {
            let mut pages_without_version = pages_without_version.clone();
            pages.append(&mut pages_without_version);
        }
    }

    let filtered_pages = pages
        .iter()
        .filter(|page| {
            if let Some(slug) = slug {
                match page {
                    PageType::Document(s_slug, doc) => {
                        s_slug == slug
                            && (version.is_none()
                                || version == Some(doc.version)
                                || version.unwrap().is_subset_of(&doc.version))
                    }
                    PageType::Category(_, s_version, _) => {
                        version.is_none() || version.as_ref() == Some(s_version)
                    }
                }
            } else if let Some(version) = version {
                match page {
                    PageType::Document(_, doc) => version == doc.version,
                    PageType::Category(_, s_version, _) => {
                        &version == s_version || version.is_subset_of(s_version)
                    }
                }
            } else {
                true
            }
        })
        .collect::<Vec<_>>();

    if filtered_pages.len() > 1 {
        Some(DocumentPageType::CategorySummary(
            t!(&format!("category.{}", category)),
            category.to_string(),
            filtered_pages
                .iter()
                .map(|page| match page {
                    PageType::Document(slug, doc) => (
                        doc.version,
                        doc.label.clone().unwrap_or_else(|| slug.to_string()),
                    ),
                    PageType::Category(label, version, _) => (*version, label.to_string()),
                })
                .collect(),
        ))
    } else {
        filtered_pages.get(0).map(|page| match page {
            PageType::Document(_, doc) => DocumentPageType::Document(Box::new((*doc).clone())),
            PageType::Category(label, version, docs) => {
                DocumentPageType::Category(label.to_string(), *version, docs.clone())
            }
        })
    }
}

pub fn hydration_state(
    locale: &str,
    _path: &str,
    params: &DocumentPageParams,
) -> Option<DocumentPageProps> {
    // find page in TOC, either with the given version or in any version
    find_page(&params.category, &params.slug, params.version)
        .or_else(|| find_page(&params.category, &params.slug, params.version))
        // if document is not found, with return None => 404
        .and_then(|page_type| {
            // if it's a PageType::Document and we're given a date, compile it
            let page_type = match (&page_type, &params.date) {
                (DocumentPageType::CategorySummary(_, _, _), _) => Some(page_type),
                (DocumentPageType::Category(_, _, _), _) => Some(page_type),
                (DocumentPageType::Document(doc), None) => {
                    let doc = *doc.clone();
                    let doc = doc.into_template();
                    doc.map(|doc| DocumentPageType::Document(Box::new(doc)))
                }
                (DocumentPageType::Document(doc), Some(date)) => {
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
                            *doc.clone(),
                            &calendar,
                            &day,
                            &observed,
                            &prefs,
                            &liturgy.preferences,
                        )
                    } else {
                        doc.clone().into_template()
                    };

                    doc.map(|doc| DocumentPageType::Document(Box::new(doc)))
                }
            };

            let base_path = match (&params.slug, &params.version) {
                (None, None) => format!("/{}/document/{}", locale, params.category),
                (None, Some(version)) => {
                    format!("/{}/document/{}/{:?}", locale, params.category, version)
                }
                (Some(slug), None) => format!("/{}/document/{}/{}", locale, params.category, slug),
                (Some(slug), Some(version)) => format!(
                    "/{}/document/{}/{}/{:?}",
                    locale, params.category, slug, version
                ),
            };

            page_type.map(|page_type| DocumentPageProps {
                page_type,
                base_path,
                slug: params.slug.clone(),
                date: params.date.map(|date| date.to_string()).unwrap_or_default(),
            })
        })
}
