use std::collections::{HashMap, HashSet};

use crate::{
    components::*,
    preferences,
    utils::{preferences::*, time::today},
};
use calendar::{Calendar, Date};
use futures::StreamExt;
use itertools::Itertools;
use language::Language;
use leptos::*;
use library::{CommonPrayer, Contents, Library};
use liturgy::{parallel_table::ParallelDocument, *};
use rust_i18n::t;
use serde::Serialize;
use serde_derive::Deserialize;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentPageParams {
    pub date: Option<Date>,
    pub calendar: Option<String>,
    pub prefs: Option<String>, // a JSON-serialized HashMap<PreferenceKey, PreferenceValue>
    pub alternate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentPageProps {
    pub page_type: DocumentPageType,
    pub path: String,
    pub slug: SlugPath,
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
    CommonPrayer::contents()
        .flatten()
        .dedup_by(|a, b| a.0 == b.0)
        .flat_map(|(slug, contents)| {
            if matches!(contents, Contents::Document(_)) {
                Box::new(
                    [
                        slug.to_string(),
                        format!("{slug}/"),
                        format!("{slug}/{{date}}"),
                        format!("{slug}/{{date}}/"),
                        format!("{slug}/{{date}}/{{calendar}}"),
                        format!("{slug}/{{date}}/{{calendar}}/"),
                        format!("{slug}/{{date}}/{{calendar}}/{{prefs}}"),
                        format!("{slug}/{{date}}/{{calendar}}/{{prefs}}/"),
                        format!("{slug}/{{date}}/{{calendar}}/{{prefs}}/{{alternate}}"),
                        format!("{slug}/{{date}}/{{calendar}}/{{prefs}}/{{alternate}}/"),
                    ]
                    .into_iter(),
                ) as Box<dyn Iterator<Item = String>>
            } else {
                Box::new([slug.to_string(), format!("{slug}/")].into_iter())
                    as Box<dyn Iterator<Item = String>>
            }
        })
        .collect()
}

pub fn hydration_state(
    locale: &str,
    path: &str,
    params: &DocumentPageParams,
) -> Option<DocumentPageProps> {
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
                                serde_json::from_str::<Vec<(PreferenceKey, PreferenceValue)>>(json)
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

            page_type.map(|page_type| DocumentPageProps {
                page_type,
                path: path.to_string(),
                slug: slug.clone(),
                date: params.date.map(|date| date.to_string()).unwrap_or_default(),
            })
        })
}

pub fn head(_locale: &str, props: &DocumentPageProps, _render_state: &()) -> View {
    let label = match &props.page_type {
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

pub fn body(locale: &str, props: &DocumentPageProps, _render_state: &()) -> View {
    match &props.page_type {
        DocumentPageType::Category { label, contents } => {
            category_body(locale, &props.slug, label, contents)
        }
        DocumentPageType::Sections { label, contents } => {
            section_body(locale, &props.slug, label, contents)
        }
        DocumentPageType::Document(params, document) => {
            document_body(locale, &props.slug, document, props, params)
        }
        DocumentPageType::ByVersion { label, documents } => {
            by_version_body(locale, &props.slug, label, documents)
        }
        DocumentPageType::MultiDocument {
            label, documents, ..
        } => multidocument_body(locale, &props.slug, label, documents),
        DocumentPageType::Parallels {
            label,
            intro,
            parallels,
        } => parallels_body(locale, &props.slug, label, intro, parallels),
    }
}

fn breadcrumbs(locale: &str, path: &SlugPath) -> View {
    let crumbs = View::Fragment(
            path.into_iter()
                .enumerate()
                .map(|(idx, slug)| {
                    let preceding = SlugPath::from(path.as_slice()[0..=idx].to_vec());
                    let label = match slug {
                        Slug::Version(version) => version.to_string(),
                        Slug::Canticle(id) => id.to_string(),
                        _ => t!(&format!("slug.{slug}"))
                    };
                    view! { <li><a href={format!("/{}/document/{}", locale, preceding)}>{label}</a></li> }
                })
                .collect(),
        );
    view! {
        <nav class="breadcrumb">
            <ol>
                <li><a href={format!("/{}", locale)}>"⌂"</a></li>
                {crumbs}
            </ol>
        </nav>
    }
}

fn category_body(
    locale: &str,
    base_slug: &SlugPath,
    title: &str,
    pages: &[(NavType, String)],
) -> View {
    let pages = View::Fragment(
        pages
            .iter()
            .map(|(nav_type, label)| {
                let href = match nav_type {
                    NavType::Slug(slug) => {
                        format!("/{}/document/{}/{}", locale, base_slug, slug.slugify())
                    }
                    NavType::Url(url) => format!("/{}/{}", locale, url),
                };
                view! {
                    <li>
                        <a href={href}>{label}</a>
                    </li>
                }
            })
            .collect(),
    );
    view! {
        <>
            {header(locale, title)}
            <main>
                {breadcrumbs(locale, base_slug)}
                <ul class="category-summary">
                    {pages}
                </ul>
            </main>
        </>
    }
}

fn section_body(
    locale: &str,
    base_slug: &SlugPath,
    title: &str,
    pages: &[(String, Vec<(NavType, String)>)],
) -> View {
    let sections = View::Fragment(
        pages
            .iter()
            .map(|(label, pages)| {
                let pages = View::Fragment(
                    pages
                        .iter()
                        .map(|(nav_type, label)| {
                            let href = match nav_type {
                                NavType::Slug(slug) => {
                                    format!("/{}/document/{}/{}", locale, base_slug, slug.slugify())
                                }
                                NavType::Url(url) => format!("/{}/{}", locale, url),
                            };
                            view! {
                                <li>
                                    <a href={href}>{label}</a>
                                </li>
                            }
                        })
                        .collect(),
                );

                let label = if !label.is_empty() {
                    view! { <h2>{label}</h2> }
                } else {
                    View::StaticText(" ".to_string())
                };

                view! {
                    <>
                        <li>
                            {label}
                            <ul>
                                {pages}
                            </ul>
                        </li>
                    </>
                }
            })
            .collect(),
    );
    view! {
        <>
            {header(locale, title)}
            <main>
                {breadcrumbs(locale, base_slug)}
                <ul class="section-summary">
                    {sections}
                </ul>
            </main>
        </>
    }
}

fn by_version_body(locale: &str, base_slug: &SlugPath, title: &str, versions: &[Version]) -> View {
    let pages = View::Fragment(
        versions
            .iter()
            .map(|version| {
                view! {
                    <li>
                        <a href={format!("/{}/document/{}/{}", locale, base_slug, Slug::Version(*version).slugify())}>{version.to_string()}</a>
                    </li>
                }
            })
            .collect(),
    );
    view! {
        <>
            {header(locale, title)}
            <main>
                {breadcrumbs(locale, base_slug)}
                <ul class="by-version-summary">
                    {pages}
                </ul>
            </main>
        </>
    }
}

fn multidocument_body(locale: &str, base_slug: &SlugPath, title: &str, docs: &[Document]) -> View {
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
            {header(locale, title)}
            <main>
                {breadcrumbs(locale, base_slug)}
                <dyn:view view={search.view()} />
                {links}
                <dyn:view view={categories} />
            </main>
        </>
    }
}

fn parallels_body(
    locale: &str,
    base_slug: &SlugPath,
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
                                    slug,
                                } => view! {
                                    <a href={format!("/{}/document/{}", locale, slug)}>{label}</a>
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
                {breadcrumbs(locale, base_slug)}
                {initial_text}
                <table>
                    {parallels}
                </table>
            </dyn:main>
        </>
    }
}

fn document_body(
    locale: &str,
    base_slug: &SlugPath,
    document: &Document,
    props: &DocumentPageProps,
    params: &DocumentPageParams,
) -> View {
    let date_picker = DatePicker::new(t!("date"), None);
    date_picker
        .date
        .stream()
        // skip the first value, because the initial value of the input will
        // always be emitted but has already been reflected in the page
        .skip(1)
        .create_effect({
            let base_slug = base_slug.clone();
            let params = params.clone();
            let language = document.language;
            let version = document.version;
            let locale = locale.to_string();
            let base_slug = base_slug.clone();
            move |date| {
                if let Some(date) = date {
                    let mut params = params.clone();
                    params.date = Some(date);
                    redirect_with_new_preferences(
                        &locale,
                        &Behavior::new(Status::Idle),
                        &base_slug,
                        language,
                        version,
                        &params,
                    )
                } else {
                    location().set_href(&base_slug.to_string()).unwrap_throw();
                }
            }
        });

    let date_menu = if document.has_date_condition() {
        view! {
            <section class="preview-menu">
                <dyn:view view={date_picker.view()}/>
            </section>
        }
    } else {
        View::Empty
    };
    let display_settings_menu = DisplaySettingsSideMenu::new();

    let liturgy_preference_menu = liturgy_preferences_view(
        &display_settings_menu.status,
        base_slug,
        document.language,
        document.version,
        &liturgy_to_preferences(document),
    );
    let liturgy_preference_menu = view! {
        <section class="liturgy-preferences">
            <h2>{t!("settings.liturgy")}</h2>
            {liturgy_preference_menu}
            <dyn:button
                on:click={
                    let params = params.clone();
                    let props = props.clone();
                    let language = document.language;
                    let version = document.version;
                    let status = display_settings_menu.status.clone();
                    let locale = locale.to_string();
                    let base_slug = base_slug.clone();
                    move |_ev: Event| redirect_with_new_preferences(&locale, &status, &base_slug, language, version, &params)
                }
            >
                {t!("settings.use_preferences")}
            </dyn:button>
        </section>
    };

    let selections = Behavior::new(HashSet::new());
    let document_controller = DocumentController::new_with_selections(document.clone(), selections);
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
            {header_with_side_menu_and_buttons(locale, &document.label.clone().unwrap_or_default(), side_menu, [export_button])}
            <dyn:main
                class={display_settings_menu.current_settings().stream().map(|settings| settings.to_class()).boxed_local()}
            >
                {breadcrumbs(locale, base_slug)}
                <dyn:view view={document_controller.view(locale)}/>
                {export_alert.view()}
            </dyn:main>
        </>
    }
}

fn redirect_with_new_preferences(
    locale: &str,
    status: &Behavior<Status>,
    slug: &SlugPath,
    language: Language,
    version: Version,
    params: &DocumentPageParams,
) {
    let prefs = preferences::get_prefs_for_liturgy(slug, language, version);
    // convert HashMap<K, V> to Vec<(K, V)> because serde_json can't serialize a HashMap with enum keys to a JSON map
    let serialized_prefs =
        serde_json::to_string(&prefs.iter().collect::<Vec<_>>()).unwrap_or_default();
    let new_url = format!(
        "/{}/document/{}/{}/{}/{}{}",
        locale,
        slug,
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
