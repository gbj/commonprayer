use std::collections::{HashMap, HashSet};

use crate::{
    components::*,
    preferences,
    table_of_contents::PageType,
    utils::{preferences::*, time::today},
    TOCLiturgy, TABLE_OF_CONTENTS,
};
use calendar::{Calendar, Date};
use futures::StreamExt;
use itertools::Itertools;
use language::Language;
use leptos::*;
use library::{CommonPrayer, Library};
use liturgy::{
    parallel_table::ParallelDocument, Content, Document, PreferenceKey, PreferenceValue, Reference,
    Source, Version,
};
use rust_i18n::t;
use serde::Serialize;
use serde_derive::Deserialize;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    Document(DocumentPageParams, Box<Document>),
    Category(String, Version, Vec<Document>),
    CategorySummary(
        String,
        String,
        Vec<(Option<Reference>, Version, Option<String>, String)>,
    ),
    Parallels(String, String, Vec<Vec<(ParallelDocument, usize)>>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentPageProps {
    pub page_type: DocumentPageType,
    pub base_path: String,
    pub path: String,
    pub slug: Option<String>,
    pub date: String,
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
        DocumentPageType::Document(_, doc) => doc.label.as_ref(),
        DocumentPageType::Category(label, _, _) => Some(label),
        DocumentPageType::CategorySummary(label, _, _) => Some(label),
        DocumentPageType::Parallels(label, ..) => Some(label),
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
            <link rel="stylesheet" href="/static/settings.css"/>
            <link rel="stylesheet" href="/static/display-settings.css"/>
        </>
    }
}

pub fn body(locale: &str, props: &DocumentPageProps, _render_state: &()) -> View {
    let title = match &props.page_type {
        DocumentPageType::Document(_, doc) => match &doc.label {
            Some(label) => label.clone(),
            None => t!("common_prayer"),
        },
        DocumentPageType::Category(label, _, _) => label.clone(),
        DocumentPageType::CategorySummary(label, _, _) => label.clone(),
        DocumentPageType::Parallels(label, ..) => label.clone(),
    };

    match &props.page_type {
        DocumentPageType::Document(params, doc) => document_body(locale, props, title, doc, params),
        DocumentPageType::Category(_, _, docs) => category_body(locale, props, title, docs),
        DocumentPageType::CategorySummary(label, slug, pages) => {
            category_summary_body(locale, label, slug, pages)
        }
        DocumentPageType::Parallels(label, initial_text, parallels) => {
            parallels_body(locale, label, initial_text, parallels)
        }
    }
}

fn category_summary_body(
    locale: &str,
    label: &str,
    category: &str,
    pages: &[(Option<Reference>, Version, Option<String>, String)],
) -> View {
    let title = label;

    let versions_per_source: HashMap<Option<Source>, Vec<Version>> = pages
        .iter()
        .group_by(|(reference, ..)| reference.map(|reference| reference.source))
        .into_iter()
        .map(|(source, docs)| {
            (
                source,
                docs.into_iter()
                    .unique_by(|(_, version, ..)| version)
                    .map(|(_, version, ..)| *version)
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    let pages = View::Fragment(
        pages
            .iter()
            .group_by(|(reference, version, ..)| {
                (version, reference.map(|reference| reference.source))
            })
            .into_iter()
            .map(|((version, source), pages)| {
                let pages = View::Fragment(
                    pages
                        .into_iter()
                        .map(|(_, version, slug, label)| {
                            let link = if let Some(slug) = slug {
                                format!("/{}/document/{}/{}/{:?}", locale, category, slug, version)
                            } else {
                                format!("/{}/document/{}/{:?}", locale, category, version)
                            };

                            view! {
                                <li><a href={link}>{label}</a></li>
                            }
                        })
                        .collect(),
                );

                let source_has_docs_with_multiple_versions = versions_per_source
                    .get(&source)
                    .map(|docs| docs.len() > 1)
                    .unwrap_or(false);

                let label = if let Some(source) = source {
                    let label = if version == &Version::Parallel {
                        t!("version.Parallel")
                    } else if source_has_docs_with_multiple_versions
                        && version != &Version::default()
                    {
                        format!("{}: {}", source, version)
                    } else {
                        source.long_name().to_string()
                    };
                    view! {
                        <h2>{label}</h2>
                    }
                } else {
                    View::Empty
                };

                view! {
                    <>
                        {label}
                        {pages}
                    </>
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

fn document_body(
    locale: &str,
    props: &DocumentPageProps,
    title: String,
    doc: &Document,
    params: &DocumentPageParams,
) -> View {
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

    let date_menu = if doc.has_date_condition() {
        view! {
            <section class="preview-menu">
                <dyn:view view={date_picker.view()}/>
            </section>
        }
    } else {
        View::Empty
    };
    let display_settings_menu = DisplaySettingsSideMenu::new();
    let current_liturgy = props.slug.as_ref().and_then(|slug| {
        let id = TOCLiturgy::from(slug.as_str());
        if id == TOCLiturgy::NotFound {
            None
        } else {
            Some(id)
        }
    });
    let liturgy_preference_menu = if let Some(current_liturgy) = current_liturgy {
        let liturgy_preference_menu = liturgy_preferences_view(
            &display_settings_menu.status,
            current_liturgy,
            doc.language,
            doc.version,
            &liturgy_to_preferences(doc),
        );
        view! {
            <section class="liturgy-preferences">
                <h2>{t!("settings.liturgy")}</h2>
                {liturgy_preference_menu}
                <dyn:button
                    on:click={
                        let params = params.clone();
                        let props = props.clone();
                        let language = doc.language;
                        let version = doc.version;
                        let status = display_settings_menu.status.clone();
                        move |_ev: Event| redirect_with_new_preferences(&status, current_liturgy, language, version, &params, &props)
                    }
                >
                    {t!("settings.use_preferences")}
                </dyn:button>
            </section>
        }
    } else {
        View::Empty
    };

    let selections = Behavior::new(HashSet::new());
    let document_controller = DocumentController::new_with_selections(doc.clone(), selections);
    document_controller.selection_mode.set(SelectionMode::Auto);

    let side_menu = side_menu(
        Icon::Settings,
        view! {
            <>
                {date_menu}

                <h2>{t!("settings.display_settings.title")}</h2>
                {display_settings_menu.component.view()}

                {liturgy_preference_menu}
            </>
        },
    );

    // export button and overlay
    let export_alert = Alert::new(export_links(props, &document_controller));
    let export_button = {
        let is_open = export_alert.is_open.clone();
        view! {
            <dyn:button on:click=move |_ev: Event| is_open.set(!is_open.get())>
                <img src={Icon::Link.to_string()}/>
                <span class="screen-reader-only">{t!("export.export")}</span>
            </dyn:button>
        }
    };

    view! {
        <>
            {header_with_side_menu_and_buttons(locale, &title, side_menu, [export_button])}
            <dyn:main
                class={display_settings_menu.current_settings().stream().map(|settings| settings.to_class()).boxed_local()}
            >
                <dyn:view view={document_controller.view(locale)}/>
                {export_alert.view()}
            </dyn:main>
        </>
    }
}

fn redirect_with_new_preferences(
    status: &Behavior<Status>,
    liturgy: TOCLiturgy,
    language: Language,
    version: Version,
    params: &DocumentPageParams,
    props: &DocumentPageProps,
) {
    let prefs = preferences::get_prefs_for_liturgy(liturgy, language, version);
    // convert HashMap<K, V> to Vec<(K, V)> because serde_json can't serialize a HashMap with enum keys to a JSON map
    let serialized_prefs =
        serde_json::to_string(&prefs.iter().collect::<Vec<_>>()).unwrap_or_default();
    let new_url = format!(
        "{}/{}/{}/{}{}",
        props.base_path,
        params.date.unwrap_or_else(today),
        params
            .calendar
            .clone()
            .unwrap_or_else(|| "bcp1979".to_string()),
        serialized_prefs,
        if let Some(alternate) = &params.alternate {
            format!("/{alternate}")
        } else {
            "".to_string()
        }
    );
    match location().set_href(&new_url) {
        Ok(_) => {}
        Err(_) => status.set(Status::Error),
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

fn parallels_body(
    locale: &str,
    label: &str,
    initial_text: &str,
    parallels: &[Vec<(ParallelDocument, usize)>],
) -> View {
    let selecting = Behavior::new(false);
    let parallel_selections: Behavior<HashMap<(usize, usize), Document>> =
        Behavior::new(HashMap::new());

    // convert table into view
    let parallels = View::Fragment(
        parallels
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                let cols = View::Fragment(
                    row.iter()
                        .enumerate()
                        .map(|(col_idx, (child, width))| {
                            let this_doc_selected = Behavior::new(false);

                            let view = match child {
                                ParallelDocument::Source(reference) => source_link(reference),
                                ParallelDocument::Link {
                                    label,
                                    version,
                                    category,
                                    slug,
                                } => view! {
                                    <a href={format!("/{}/document/{}/{}/{:?}", locale, category, slug, version)}>{label}</a>
                                },
                                ParallelDocument::Explainer(Some(explainer)) => view! {
                                    <p class="explainer">{explainer}</p>
                                },
                                ParallelDocument::Explainer(None) => View::Empty,
                                ParallelDocument::Document(doc) => {
                                    let doc = *(*doc).clone();

                                    let checkbox = view! {
                                        <dyn:input
                                            type="checkbox"
                                            class="manual-select hidden"
                                            class:hidden={selecting.stream().map(|selecting| !selecting).boxed_local()}
                                            on:click={
                                                let doc = doc.clone();
                                                let selections = parallel_selections.clone();
                                                let this_doc_selected = this_doc_selected.clone();
                                                move |ev: Event| {
                                                    let checked = event_target_checked(ev);
                                                    let key = (row_idx, col_idx);
                                                    if checked {
                                                        this_doc_selected.set(true);
                                                        selections.update(|selections| { selections.insert(key, doc.clone()); });
                                                    } else {
                                                        this_doc_selected.set(false);
                                                        selections.update(|selections| { selections.remove(&key); });
                                                    }
                                                }
                                            }
                                        />
                                    };

                                    view! {
                                        <>
                                            {checkbox}
                                            {DocumentController::new(doc).view(locale)}
                                        </>
                                    }
                                }
                            };

                            view! {
                                <dyn:td colspan={width.to_string()} class:selected={this_doc_selected.stream().boxed_local()}>{view}</dyn:td>
                            }
                        })
                        .collect(),
                );
                view! {
                    <tr>{cols}</tr>
                }
            })
            .collect(),
    );

    let display_settings_menu = DisplaySettingsSideMenu::new();
    let side_menu = side_menu(
        Icon::Settings,
        view! {
            <>
                <h2>{t!("settings.display_settings.title")}</h2>
                {display_settings_menu.component.view()}
            </>
        },
    );

    let alert = Alert::new(parallel_exports(&parallel_selections));

    let select_button = {
        view! {
            <>
                <dyn:button
                    on:click={
                        let selecting = selecting.clone();
                        let alert_is_open = alert.is_open.clone();
                        move |_ev: Event| {
                            // if currently selecting, show export alert
                            if selecting.get() {
                                alert_is_open.set(true);
                            }
                            // otherwise, show checkboxes
                            else {
                                selecting.set(true);
                            }
                        }
                    }
                >
                    <dyn:img src={Icon::Checkbox.to_string()} class:hidden={selecting.stream().boxed_local()} />
                    <dyn:span class="screen-reader-only" class:hidden={selecting.stream().boxed_local()}>{t!("export.select")}</dyn:span>
                    <dyn:img src={Icon::Download.to_string()} class="hidden" class:hidden={selecting.stream().map(|n| !n).boxed_local()} />
                    <dyn:span class="hidden screen-reader-only" class:hidden={selecting.stream().map(|n| !n).boxed_local()}>{t!("export.export")}</dyn:span>
                </dyn:button>
                {alert.view()}
            </>
        }
    };

    let initial_text = View::Fragment(
        initial_text
            .split("\n\n")
            .map(|para| view! { <p class="initial-text">{para}</p> })
            .collect(),
    );

    view! {
        <>
            {header_with_side_menu_and_buttons(locale, label, side_menu, [select_button])}
            <dyn:main
                class="parallels"
                class={display_settings_menu.current_settings().stream().map(|settings| format!("parallels {}", settings.to_class())).boxed_local()}
            >
                {initial_text}
                <table>
                    {parallels}
                </table>
            </dyn:main>
        </>
    }
}

fn find_page(
    params: &DocumentPageParams,
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
                    PageType::Parallel(s_slug, ..) => s_slug == slug,
                }
            } else if let Some(version) = version {
                match page {
                    PageType::Document(_, doc) => version == doc.version,
                    PageType::Category(_, s_version, _) => {
                        &version == s_version || version.is_subset_of(s_version)
                    }
                    PageType::Parallel(s_slug, _, _, _) => &Some(s_slug.to_string()) == slug,
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
                        doc.source,
                        doc.version,
                        Some(slug.to_string()),
                        doc.label.clone().unwrap_or_else(|| slug.to_string()),
                    ),
                    PageType::Category(label, version, _) => {
                        (None, *version, None, label.to_string())
                    }
                    PageType::Parallel(slug, label, _, _) => (
                        None,
                        Version::Parallel,
                        Some(slug.to_string()),
                        label.to_string(),
                    ),
                })
                .collect(),
        ))
    } else {
        filtered_pages.get(0).map(|page| match page {
            PageType::Document(_, doc) => {
                DocumentPageType::Document(params.clone(), Box::new((*doc).clone()))
            }
            PageType::Category(label, version, docs) => {
                DocumentPageType::Category(label.to_string(), *version, docs.clone())
            }
            PageType::Parallel(_, label, initial_text, table) => DocumentPageType::Parallels(
                label.to_string(),
                initial_text.to_string(),
                table.to_vec(),
            ),
        })
    }
}

pub fn hydration_state(
    locale: &str,
    path: &str,
    params: &DocumentPageParams,
) -> Option<DocumentPageProps> {
    // find page in TOC, either with the given version or in any version
    find_page(params, &params.category, &params.slug, params.version)
        .or_else(|| find_page(params, &params.category, &params.slug, params.version))
        // if document is not found, with return None => 404
        .and_then(|page_type| {
            // if it's a PageType::Document and we're given a date, compile it
            let page_type = match (&page_type, &params.date) {
                (DocumentPageType::CategorySummary(_, _, _), _) => Some(page_type),
                (DocumentPageType::Category(_, _, _), _) => Some(page_type),
                (DocumentPageType::Document(params, doc), None) => {
                    let doc = *doc.clone();
                    let doc = doc.into_template();
                    doc.map(|doc| DocumentPageType::Document(params.clone(), Box::new(doc)))
                }
                (DocumentPageType::Document(params, doc), Some(date)) => {
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

                    doc.map(|doc| DocumentPageType::Document(params.clone(), Box::new(doc)))
                }
                (DocumentPageType::Parallels(_, _, _), _) => Some(page_type),
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
                path: path.to_string(),
                slug: params.slug.clone(),
                date: params.date.map(|date| date.to_string()).unwrap_or_default(),
            })
        })
}
