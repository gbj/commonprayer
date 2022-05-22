use super::breadcrumbs::breadcrumbs;
use crate::views::Header;
use itertools::Itertools;
use leptos2::*;
use liturgy::{Document, Slug, SlugPath, Version};

pub fn multidocument_body(
    locale: &str,
    base_slug: &SlugPath,
    title: &str,
    docs: &[Document],
) -> Vec<Node> {
    todo!() /*
            // TODO let search = SearchBar::new();

            // grouped by category (tags[0]), subcategory (tags[1]), then label
            let tree = docs
                .iter()
                .group_by(|doc| doc.tags.get(0))
                .into_iter()
                .map(move |(category, docs_with_category)| {
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
                                docs_with_category
                                        .iter()
                                        .map(|(subcategory, docs_with_subcategory)| {
                                            let labels = if docs_with_subcategory.len() > 1 {
                                                docs_with_subcategory
                                                    .iter()
                                                    .map(|(label, docs)| {
                                                        let hidden = search
                                                                    .value
                                                                    .stream()
                                                                    .map({
                                                                        let docs = docs.iter().map(|doc| (*doc).clone()).collect::<Vec<_>>();
                                                                        move |search| {
                                                                            !search.is_empty()
                                                                                && !docs.iter().any(|doc| doc.contains_case_insensitive(&search))
                                                                        }
                                                                    })
                                                                    .boxed_local();

                                                        view! {
                                                            <dyn:li class:hidden={hidden}>
                                                                <a href={format!("#{}", label.cloned().unwrap_or_default())}>{label.cloned().unwrap_or_default()}</a>
                                                            </dyn:li>
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            } else {
                                                Vec::new()
                                            };

                                            let docs = docs_with_subcategory.iter()
                                                .flat_map(|(_, docs)| docs.iter())
                                                .map(|doc| (*doc).clone()).collect::<Vec<_>>();
                                            let hidden = search.value
                                                .stream()
                                                .map(move |search| {
                                                    !search.is_empty()
                                                        && !docs.iter().any(|doc| doc.contains_case_insensitive(&search))
                                                })
                                                .boxed_local();

                                            view! {
                                                <dyn:li class:hidden={hidden}>
                                                     <a href={format!("#{}", subcategory.cloned().unwrap_or_default())}>{subcategory.cloned().unwrap_or_default()}</a>
                                                    <ul>{labels}</ul>
                                                </dyn:li>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            };

                            let docs = docs_with_category.iter()
                                .flat_map(|(_, docs)| docs.iter().flat_map(|(_, docs)| docs.iter()))
                                .map(|doc| (*doc).clone()).collect::<Vec<_>>();
                            let hidden = search.value
                                .stream()
                                .map(move |search| {
                                    !search.is_empty()
                                        && !docs.iter().any(|doc| doc.contains_case_insensitive(&search))
                                })
                                .boxed_local();

                            view! {
                                <dyn:li class:hidden={hidden}>
                                    <a href={format!("#{}", category.cloned().unwrap_or_default())}>{category.cloned().unwrap_or_default()}</a>
                                    <ul>{subcategories}</ul>
                                </dyn:li>
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
                        let docs = subcategories.iter()
                            .flat_map(|(_, docs)| docs.iter().flat_map(|(_, docs)| docs.iter()))
                            .map(|doc| (*doc).clone()).collect::<Vec<_>>();
                        let hidden = search.value
                            .stream()
                            .map(move |search| {
                                !search.is_empty()
                                    && !docs.iter().any(|doc| doc.contains_case_insensitive(&search))
                            })
                            .boxed_local();

                        let subcategories = View::Fragment(
                            subcategories
                                .iter()
                                .map(|(subcategory, docs_with_subcategory)| {
                                    let docs = docs_with_subcategory.iter()
                                        .flat_map(|(_, docs)| docs.iter())
                                        .map(|doc| (*doc).clone())
                                        .collect::<Vec<_>>();
                                    let hidden = search.value
                                        .stream()
                                        .map(move |search| {
                                            !search.is_empty()
                                                && !docs.iter().any(|doc| doc.contains_case_insensitive(&search))
                                        })
                                        .boxed_local();

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
                                                        <dyn:h3 class:hidden={hidden}>{subcategory.to_string()}</dyn:h3>
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
                                            <dyn:h2 class:hidden={hidden}>{category.to_string()}</dyn:h2>
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
            } */
}
