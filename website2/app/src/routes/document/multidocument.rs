use super::breadcrumbs::breadcrumbs;
use super::views::DocumentView;
use crate::WebView;
use itertools::Itertools;
use leptos2::*;
use liturgy::{Document, Slug, SlugPath, Version};

type CategoryTree<'a> = Vec<(
    Option<&'a String>,
    Vec<(
        Option<&'a String>,
        Vec<(Option<&'a String>, Vec<&'a Document>)>,
    )>,
)>;

pub fn multidocument_body(
    locale: &str,
    base_slug: &SlugPath,
    title: &str,
    docs: &[Document],
    search: &str,
) -> Vec<Node> {
    let tree: CategoryTree = docs
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

    view! {
        <>
            <header><h1>{title}</h1></header>
            <main>
                {breadcrumbs(locale, base_slug)}
                <form>
                    <label class="stacked">
                        {t!("search")}
                        <input type="search" name="search" value={search}/>
                    </label>
                </form>
                {links(&tree, search)}
                {categories(&locale, &tree, search)}
            </main>
        </>
    }
}

fn links(tree: &CategoryTree, search: &str) -> Node {
    if tree.len() > 1 {
        let categories =  tree.iter()
                .map(|(category, docs_with_category)| {
                    let subcategories = if docs_with_category.len() > 1 {
                        docs_with_category
                                .iter()
                                .map(|(subcategory, docs_with_subcategory)| {
                                    let labels = if docs_with_subcategory.len() > 1 {
                                        docs_with_subcategory
                                            .iter()
                                            .map(|(label, docs)| {
                                                let hidden = !search.is_empty() && !docs.iter().any(|doc| doc.contains_case_insensitive(&search));

                                                view! {
                                                    <li class:hidden={hidden}>
                                                        <a href={format!("#{}", label.cloned().unwrap_or_default())}>{label.cloned().unwrap_or_default()}</a>
                                                    </li>
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                    } else {
                                        Vec::new()
                                    };

                                    let docs = docs_with_subcategory.iter()
                                        .flat_map(|(_, docs)| docs.iter())
                                        .map(|doc| (*doc).clone()).collect::<Vec<_>>();
                                    let hidden = !search.is_empty() && !docs.iter().any(|doc| doc.contains_case_insensitive(&search));

                                    view! {
                                        <li class:hidden={hidden}>
                                             <a href={format!("#{}", subcategory.cloned().unwrap_or_default())}>{subcategory.cloned().unwrap_or_default()}</a>
                                            <ul>{labels}</ul>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>()
                    } else {
                        Vec::new()
                    };

                    let docs = docs_with_category.iter()
                        .flat_map(|(_, docs)| docs.iter().flat_map(|(_, docs)| docs.iter()))
                        .map(|doc| (*doc).clone()).collect::<Vec<_>>();
                    let hidden = !search.is_empty() && !docs.iter().any(|doc| doc.contains_case_insensitive(&search));

                    view! {
                        <li class:hidden={hidden}>
                            <a href={format!("#{}", category.cloned().unwrap_or_default())}>{category.cloned().unwrap_or_default()}</a>
                            <ul>{subcategories}</ul>
                        </li>
                    }
                })
                .collect::<Vec<_>>();
        view! {
            <ul>{categories}</ul>
        }
    } else {
        text("")
    }
}

fn categories(locale: &str, tree: &CategoryTree, search: &str) -> Vec<Node> {
    // grouped by category (tags[0]), subcategory (tags[1]), then label
    tree.iter()
        .map(|(category, subcategories)| {
            let docs = subcategories
                .iter()
                .flat_map(|(_, docs)| docs.iter().flat_map(|(_, docs)| docs.iter()))
                .map(|doc| (*doc).clone())
                .collect::<Vec<_>>();

            let hidden = !search.is_empty()
                && !docs
                    .iter()
                    .any(|doc| doc.contains_case_insensitive(&search));

            let subcategories = subcategories
                .iter()
                .map(|(subcategory, docs_with_subcategory)| {
                    let docs = docs_with_subcategory
                        .iter()
                        .flat_map(|(_, docs)| docs.iter())
                        .map(|doc| (*doc).clone())
                        .collect::<Vec<_>>();
                    let hidden = !search.is_empty()
                        && !docs
                            .iter()
                            .any(|doc| doc.contains_case_insensitive(&search));

                    let labels = docs_with_subcategory
                        .iter()
                        .map(|(label, docs_with_label)| {
                            let docs = docs_with_label.iter().cloned().cloned().collect::<Vec<_>>();
                            let subtitle = docs.get(0).and_then(|doc| doc.subtitle.clone());

                            let docs_view = docs
                                .iter()
                                .map(|doc| {
                                    let hidden = !search.is_empty()
                                        && !doc.contains_case_insensitive(&search);

                                    let doc = DocumentView {
                                        path: vec![],
                                        doc: &Document {
                                            label: None,    // don't show the label again for each doc
                                            subtitle: None, // don't show subtitle again for every doc
                                            ..(*doc).clone()
                                        },
                                    }
                                    .view(locale);

                                    view! {
                                        <article class="document" class:hidden={hidden}>
                                            {doc}
                                        </article>
                                    }
                                })
                                .collect::<Vec<_>>();

                            let label = label.map(|label| {
                                let hidden = !label.contains(&search.to_lowercase())
                                    && !docs
                                        .iter()
                                        .any(|doc| doc.contains_case_insensitive(&search));

                                if let Some(subtitle) = subtitle {
                                    view! {
                                        <>
                                            <div class="label-and-subtitle" class:hidden={hidden}>
                                                <a id={label.to_string()}></a>
                                                <h4>{label.to_string()}</h4>
                                                <h5 class="subtitle">{subtitle}</h5>
                                            </div>
                                        </>
                                    }
                                } else {
                                    view! {
                                        <>
                                            <a id={label.to_string()}></a>
                                            <h4 class:hidden={hidden}>{label.to_string()}</h4>
                                        </>
                                    }
                                }
                            }).unwrap_or_default();

                            view! {
                                <div>
                                    {label}
                                    {docs_view}
                                </div>
                            }
                        })
                        .collect::<Vec<_>>();

                    view! {
                        <div>
                            {subcategory.map(|subcategory| view! {
                                <>
                                    <a id={subcategory.to_string()}></a>
                                    <h3 class:hidden={hidden}>{subcategory.to_string()}</h3>
                                </>
                            }).unwrap_or_default()}
                            {labels}
                        </div>
                    }
                })
                .collect::<Vec<_>>();

            view! {
                <div>
                    {category.map(|category|
                        view! {
                            <>
                                <a id={category.to_string()}></a>
                                <h2 class:hidden={hidden}>{category.to_string()}</h2>
                            </>
                        }
                    ).unwrap_or_default()}
                    {subcategories}
                </div>
            }
        })
        .collect::<Vec<_>>()
}
