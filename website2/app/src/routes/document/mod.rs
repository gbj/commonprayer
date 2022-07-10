use std::collections::HashMap;

use calendar::{Calendar, Date};
use docx::DocxDocument;
use itertools::Itertools;
use leptos2::{view::View, *};
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
pub mod views;

pub use export_links::*;

use super::{
    readings::export_docx::docx_response,
    settings::{DisplaySettings, Settings},
};

#[derive(Debug)]
pub enum DocumentPageType {
    Category {
        label: String,
        contents: Vec<(NavType, String)>,
    },
    Sections {
        label: String,
        contents: Vec<(String, Vec<(NavType, String)>)>,
    },
    Document(DocumentPageQuery, Box<Document>),
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum NavType {
    Slug(Slug),
    Url(String),
}

#[derive(Debug)]
pub struct DocumentPage {
    pub locale: String,
    pub page_type: DocumentPageType,
    pub path: String,
    pub slug: SlugPath,
    pub date: String,
    pub display_settings: DisplaySettings,
}

#[derive(Params)]
pub struct DocumentPageParams {
    pub remainder: String,
}

#[derive(Params, Clone, Debug)]
pub struct DocumentPageQuery {
    pub search: Option<String>,
    pub date: Option<Date>,
    pub calendar: Option<String>,
    pub prefs: Option<String>, // a JSON-serialized HashMap<PreferenceKey, PreferenceValue>
    pub alternate: Option<String>,
}

#[async_trait(?Send)]
impl Loader for DocumentPage {
    type Params = DocumentPageParams;
    type Query = DocumentPageQuery;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let mut slug_parts = Vec::new();
        for part in params.remainder.split('/') {
            if let Some(slug) = Slug::unslugify(part) {
                slug_parts.push(slug);
            }
        }

        let display_settings = Settings::display(&req).await;

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
                        if let Some(date) = query.date {
                            let calendar = query
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

                            let observed = query
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

                            let prefs: HashMap<PreferenceKey, PreferenceValue> = query
                                .prefs
                                .as_ref()
                                // this strange indirection is necessary because serde_json can't use structs/enums as map keys
                                // (due to JSON format limitations)
                                .and_then(|json| {
                                    serde_json::from_str::<Vec<(PreferenceKey, PreferenceValue)>>(
                                        &urlencoding::decode(json).unwrap(),
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

                            doc.map(|doc| DocumentPageType::Document(query.clone(), Box::new(doc)))
                        } else {
                            let doc = doc.clone().into_template();
                            doc.map(|doc| DocumentPageType::Document(query.clone(), Box::new(doc)))
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
                    locale: locale.to_string(),
                    page_type,
                    path: req.path().to_string(),
                    slug: slug.clone(),
                    date: query.date.map(|date| date.to_string()).unwrap_or_default(),
                    display_settings,
                })
            })
    }

    // POST to get a DOCX of any `Document` that has been POSTed
    async fn action(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> ActionResponse {
        let form_data = req
            .body()
            .and_then(|body| body.as_form_data::<DocxPostForm>().ok())
            .unwrap_or_default();
        match serde_json::from_str::<Document>(&form_data.doc) {
            Ok(doc) => {
                let docx = DocxDocument::new().add_content(&doc);
                match docx_response(
                    format!("{}-{}", form_data.liturgy.replace('/', "-"), form_data.date),
                    docx,
                ) {
                    Ok(path) => ActionResponse::from_path(path),
                    Err(e) => ActionResponse::from_error(e),
                }
            }
            Err(e) => ActionResponse::from_error(e),
        }
    }
}

#[derive(Params, Default)]
pub struct DocxPostForm {
    liturgy: String,
    date: String,
    doc: String,
}

impl View for DocumentPage {
    fn title(&self) -> String {
        let label = match &self.page_type {
            DocumentPageType::Document(_, doc) => doc.label.as_ref(),
            DocumentPageType::Category { label, .. } => Some(label),
            DocumentPageType::Sections { label, .. } => Some(label),
            DocumentPageType::ByVersion { label, .. } => Some(label),
            DocumentPageType::MultiDocument { label, .. } => Some(label),
            DocumentPageType::Parallels { label, .. } => Some(label),
        };
        match label {
            Some(label) => format!("{} – {}", label, t!("common_prayer")),
            None => t!("common_prayer"),
        }
    }

    fn styles(&self) -> view::Styles {
        vec![
            include_str!("../../styles/document.css").into(),
            include_str!("../../styles/settings.css").into(),
            include_str!("../../styles/display-settings.css").into(),
        ]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> view::Body {
        let children = match &self.page_type {
            DocumentPageType::Category { label, contents } => {
                category_body(&self.locale, &self.slug, label, contents)
            }
            DocumentPageType::Sections { label, contents } => {
                section_body(&self.locale, &self.slug, label, contents)
            }
            DocumentPageType::Document(params, document) => {
                document_body(&self.locale, &self.slug, document, &self, params)
            }
            DocumentPageType::ByVersion { label, documents } => {
                by_version_body(&self.locale, &self.slug, label, documents)
            }
            DocumentPageType::MultiDocument {
                label,
                documents,
                search,
                ..
            } => multidocument_body(&self.locale, &self.slug, label, documents, search),
            DocumentPageType::Parallels {
                label,
                intro,
                parallels,
            } => parallels_body(&self.locale, &self.slug, label, intro, parallels),
        };
        view! {
            <div class={self.display_settings.to_class()}>{children}</div>
        }
    }
}
