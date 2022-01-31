use crate::components::{header, SearchBar};
use episcopal_api::hymnal::*;
use futures::StreamExt;
use itertools::Itertools;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
    number: Option<HymnNumber>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct HymnalPageProps {
    hymnals: Vec<Hymnal>,
}

pub fn hymnal() -> Page<HymnalPageProps, HymnalPageParams> {
    Page::new("hymnal")
        .head_fn(head)
        .body_fn(body)
        .static_props_fn(get_static_props)
        .build_paths_fn(get_static_paths)
}

pub fn head(_locale: &str, props: &HymnalPageProps) -> View {
    let title = if props.hymnals.len() == 1 {
        props.hymnals[0].title.clone()
    } else {
        t!("menu.hymnal")
    };

    view! {
        <>
            <title>{title} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/hymnal.css"/>
        </>
    }
}

pub fn get_static_paths() -> Vec<String> {
    vec!["".into(), "{hymnal}".into(), "{hymnal}/{number}".into()]
}

pub fn get_static_props(_locale: &str, _path: &str, params: HymnalPageParams) -> HymnalPageProps {
    HymnalPageProps {
        hymnals: match params.hymnal {
            None => vec![HYMNAL_1982.clone(), LEVAS.clone(), WLP.clone()],
            Some(Hymnals::Hymnal1982) => vec![HYMNAL_1982.clone()],
            Some(Hymnals::LEVAS) => vec![LEVAS.clone()],
            Some(Hymnals::WLP) => vec![WLP.clone()],
        },
    }
}

pub fn body(locale: &str, props: &HymnalPageProps) -> View {
    let search_bar = SearchBar::new();

    let hymnals = View::Fragment(
        props
            .hymnals
            .iter()
            .map({
                let search = search_bar.value.clone();
                move |hymnal| {
                    let title = view! { <h2>{&hymnal.title}</h2> };
                    let subtitle = if !hymnal.subtitle.is_empty() {
                        view! { <h3>{&hymnal.subtitle}</h3> }
                    } else {
                        View::Empty
                    };
                    let copyright = view! { <p class="copyright">{&hymnal.copyright}</p> };

                    let hymns = View::Fragment(
                        hymnal
                            .hymns
                            .iter()
                            .map(|hymn| {
                                let number = match hymn.number {
                                    HymnNumber::S(n) => format!("S{}", n),
                                    HymnNumber::H(n) => n.to_string(),
                                };

                                let tune_name = if hymn.tune.starts_with('[') {
                                    ""
                                } else {
                                    &hymn.tune
                                }
                                .to_lowercase();

                                let hidden = search
                                    .stream()
                                    .map({
                                        let number = number.clone();
                                        let title = hymn.title.clone();
                                        let tune = hymn.tune.clone();
                                        move |search| {
                                            let search = search.to_lowercase();
                                            !search.is_empty()
                                                && !number.to_lowercase().contains(&search)
                                                && !title.to_lowercase().contains(&search)
                                                && !tune.to_lowercase().contains(&search)
                                        }
                                    })
                                    .boxed_local();

                                view! {
                                    <dyn:tr class:hidden={hidden}>
                                        <td>{number}</td>
                                        <td>{&hymn.title}</td>
                                        <td class="tune">{&tune_name}</td>
                                    </dyn:tr>
                                }
                            })
                            .collect(),
                    );

                    view! {
                        {title}
                        {subtitle}
                        {copyright}
                        <table>
                            {hymns}
                        </table>
                    }
                }
            })
            .collect(),
    );

    view! {
        <>
            {header(locale, &t!("menu.hymnal"))}
            <main>
                {search_bar.view()}
                {hymnals}
            </main>
        </>
    }
}
