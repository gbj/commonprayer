use std::collections::HashMap;

use calendar::{Calendar, Date};
use itertools::Itertools;
use leptos2::*;
use library::{CommonPrayer, Contents, Library};
use liturgy::{parallel_table::ParallelDocument, *};
use rust_i18n::t;
use serde::Serialize;
use serde_derive::Deserialize;

use self::{
    by_version::by_version_body, category::category_body, document_body::document_body,
    multidocument::multidocument_body, parallels::parallels_body, section::section_body,
};

mod breadcrumbs;
mod by_version;
mod category;
mod document_body;
mod export_links;
mod multidocument;
mod parallels;
mod section;

pub use export_links::*;

#[derive(Clone)]
pub enum DocumentPageType {
    Category {
        label: String,
        contents: Vec<(NavType, String)>,
    },
    Sections {
        label: String,
        contents: Vec<(String, Vec<(NavType, String)>)>,
    },
    Document(DocumentPageParams, Box<Document>),
    ByVersion {
        label: String,
        documents: Vec<Version>,
    },
    MultiDocument {
        label: String,
        documents: Vec<Document>,
        hidden_in_toc: bool,
        search: String,
    },
    Parallels {
        label: String,
        intro: String,
        parallels: Vec<Vec<(ParallelDocument, usize)>>,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum NavType {
    Slug(Slug),
    Url(String),
}

pub struct DocumentPage {
    pub page_type: DocumentPageType,
    pub path: String,
    pub slug: SlugPath,
    pub date: String,
}

#[derive(Deserialize, Clone)]
pub struct DocumentPageParams {
    pub date: Option<Date>,
    pub calendar: Option<String>,
    pub prefs: Option<String>, // a JSON-serialized HashMap<PreferenceKey, PreferenceValue>
    pub alternate: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct DocumentPageQuery {
    pub search: Option<String>,
}

impl Page for DocumentPage {
    type Params = DocumentPageParams;
    type Query = DocumentPageQuery;

    fn name() -> &'static str {
        "document"
    }

    fn paths() -> Vec<String> {
        CommonPrayer::contents()
            .flatten()
            .dedup_by(|a, b| a.0 == b.0)
            .flat_map(|(slug, contents)| {
                if let Contents::Document(doc) = contents {
                    let has_date_condition = doc.has_date_condition();
                    let has_preferences = if let Content::Liturgy(liturgy) = &doc.content {
                        !liturgy.preferences.is_empty()
                    } else {
                        false
                    };

                    if has_date_condition || has_preferences {
                        Box::new(
                            [
                                slug.to_string(),
                                format!("{slug}"),
                                format!("{slug}/{{date}}"),
                                format!("{slug}/{{date}}/{{calendar}}"),
                                format!("{slug}/{{date}}/{{calendar}}/{{prefs}}"),
                                format!("{slug}/{{date}}/{{calendar}}/{{prefs}}/{{alternate}}"),
                            ]
                            .into_iter(),
                        ) as Box<dyn Iterator<Item = String>>
                    } else {
                        Box::new(std::iter::once(slug.to_string()))
                            as Box<dyn Iterator<Item = String>>
                    }
                } else {
                    Box::new([slug.to_string(), format!("{slug}")].into_iter())
                        as Box<dyn Iterator<Item = String>>
                }
            })
            .collect()
    }

    fn build_state(
        locale: &str,
        path: &str,
        params: Self::Params,
        query: Self::Query,
    req: &HttpRequest
    ) -> Option<Self> {
        let locale_and_base = format!("/{locale}/document/");
        let path = path.replace(&locale_and_base, "");

        let mut slug_parts = Vec::new();
        for part in path.split('/') {
            if let Some(slug) = Slug::unslugify(part) {
                slug_parts.push(slug);
            }
        }

        let slug = SlugPath::from(slug_parts);

        CommonPrayer::contents()
            .contents_at_path(&slug)
            .and_then(|contents| {
                let page_type = match contents {
                    Contents::Category { label, contents } => Some(DocumentPageType::Category {
                        label,
                        contents: contents
                            .into_iter()
                            .filter_map(|(slug, contents)| {
                                if contents.hidden_in_toc() {
                                    None
                                } else {
                                    Some((
                                        if let Contents::Page { url, .. } = &contents {
                                            NavType::Url(url.to_string())
                                        } else {
                                            NavType::Slug(slug)
                                        },
                                        contents.label().unwrap_or_default(),
                                    ))
                                }
                            })
                            .collect(),
                    }),
                    Contents::Sections { label, contents } => Some(DocumentPageType::Sections {
                        label,
                        contents: contents
                            .into_iter()
                            .map(|section| {
                                (
                                    section.label.unwrap_or_default(),
                                    section
                                        .contents
                                        .into_iter()
                                        .filter_map(|(slug, contents)| {
                                            if contents.hidden_in_toc() {
                                                None
                                            } else {
                                                Some((
                                                    if let Contents::Page { url, .. } = &contents {
                                                        NavType::Url(url.to_string())
                                                    } else {
                                                        NavType::Slug(slug)
                                                    },
                                                    contents.label().unwrap_or_default(),
                                                ))
                                            }
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                    }),
                    Contents::Document(doc) => {
                        if let Some(date) = params.date {
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

                            let day = calendar.liturgical_day(date, evening);

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
                                    serde_json::from_str::<Vec<(PreferenceKey, PreferenceValue)>>(
                                        json,
                                    )
                                    .ok()
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
                            } else {
                                doc.clone().into_template()
                            };

                            doc.map(|doc| DocumentPageType::Document(params.clone(), Box::new(doc)))
                        } else {
                            let doc = doc.clone().into_template();
                            doc.map(|doc| DocumentPageType::Document(params.clone(), Box::new(doc)))
                        }
                    }
                    Contents::ByVersion { label, documents } => Some(DocumentPageType::ByVersion {
                        label,
                        documents: documents.iter().map(|document| document.version).collect(),
                    }),
                    Contents::MultiDocument {
                        label,
                        documents,
                        hidden_in_toc,
                    } => Some(DocumentPageType::MultiDocument {
                        label,
                        documents,
                        hidden_in_toc,
                        search: query.search.unwrap_or_default(),
                    }),
                    Contents::Parallels {
                        label,
                        intro,
                        parallels,
                    } => Some(DocumentPageType::Parallels {
                        label,
                        intro,
                        parallels,
                    }),
                    Contents::Page { .. } => None,
                };

                page_type.map(|page_type| DocumentPage {
                    page_type,
                    path: path.to_string(),
                    slug: slug.clone(),
                    date: params.date.map(|date| date.to_string()).unwrap_or_default(),
                })
            })
    }

    fn head(&self, locale: &str) -> Vec<Node> {
        let label = match &self.page_type {
            DocumentPageType::Document(_, doc) => doc.label.as_ref(),
            DocumentPageType::Category { label, .. } => Some(label),
            DocumentPageType::Sections { label, .. } => Some(label),
            DocumentPageType::ByVersion { label, .. } => Some(label),
            DocumentPageType::MultiDocument { label, .. } => Some(label),
            DocumentPageType::Parallels { label, .. } => Some(label),
        };
        let title = match label {
            Some(label) => format!("{} – {}", label, t!("common_prayer")),
            None => t!("common_prayer"),
        };

        view! {
            <>
                <title>{title}</title>
                <link rel="stylesheet" href="/static/vars.css"/>
                <link rel="stylesheet" href="/static/general.css"/>
                <link rel="stylesheet" href="/static/document.css"/>
                <link rel="stylesheet" href="/static/settings.css"/>
                <link rel="stylesheet" href="/static/display-settings.css"/>
            </>
        }
    }

    fn body(&self, locale: &str) -> Vec<Node> {
        match &self.page_type {
            DocumentPageType::Category { label, contents } => {
                category_body(locale, &self.slug, label, contents)
            }
            DocumentPageType::Sections { label, contents } => {
                section_body(locale, &self.slug, label, contents)
            }
            DocumentPageType::Document(params, document) => {
                document_body(locale, &self.slug, document, self, params)
            }
            DocumentPageType::ByVersion { label, documents } => {
                by_version_body(locale, &self.slug, label, documents)
            }
            DocumentPageType::MultiDocument {
                label,
                documents,
                search,
                ..
            } => multidocument_body(locale, &self.slug, label, documents, search),
            DocumentPageType::Parallels {
                label,
                intro,
                parallels,
            } => parallels_body(locale, &self.slug, label, intro, parallels),
        }
    }
}
