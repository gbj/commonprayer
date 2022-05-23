// TODO
// - search already given from server side
// - hashtags for categories

mod hymnal_search;

use crate::utils::decode_uri;
use hymnal::*;
use leptos2::*;

use crate::views::*;
use hymnal::{HymnMetadata, Hymnal, Hymnals};

pub use hymnal_search::HymnalSearch;

#[derive(Clone, Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct HymnalPageState {
    search: String,
    search_results: Vec<HymnMetadata>,
    hymnals: Vec<(Hymnal)>,
}

#[derive(Default, Clone)]
pub struct HymnalPageRenderState(Vec<Hymnal>);

pub fn hymnal() -> Page<HymnalPageState, HymnalPageParams> {
    Page::new("hymnal")
        .head_fn(head)
        .body_fn(body)
        .state(state)
        .build_paths_fn(get_static_paths)
        .incremental_generation()
}

pub fn head(_locale: &str, props: &HymnalPageState) -> Vec<Node> {
    let title = if props.hymnals.len() == 1 {
        props.hymnals[0].title.clone()
    } else {
        t!("menu.hymnal")
    };

    view! {
        <>
            <title>{title} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/vars.css"/>
            <link rel="stylesheet" href="/static/general.css"/>
        </>
    }
}

pub fn get_static_paths() -> Vec<String> {
    vec!["".into(), "{hymnal}".into()]
}

pub fn state(_locale: &str, path: &str, params: &HymnalPageParams) -> Option<HymnalPageState> {
    let mut search_parts = path.split("?q=");
    search_parts.next();

    let hymnals = match params.hymnal {
        None => {
            vec![
                HYMNAL_1982.to_metadata(),
                LEVAS.to_metadata(),
                WLP.to_metadata(),
                EL_HIMNARIO.to_metadata(),
            ]
        }
        Some(hymnal_id) => {
            let hymnal: Hymnal = hymnal_id.into();
            vec![hymnal.to_metadata()]
        }
    };

    let search = decode_uri(search_parts.next().unwrap_or_default());

    Some(match params.hymnal {
        None => HymnalPageState {
            search_results: HYMNAL_1982
                .search(&search)
                .chain(LEVAS.search(&search))
                .chain(WLP.search(&search))
                .chain(EL_HIMNARIO.search(&search))
                .collect(),
            hymnals,
            search,
        },
        Some(hymnal_id) => {
            let hymnal: Hymnal = hymnal_id.into();
            HymnalPageState {
                search_results: hymnal.search(&search).collect(),
                hymnals,
                search,
            }
        }
    })
}

pub fn body(locale: &str, props: &HymnalPageState) -> Vec<Node> {
    view! {
        <>
            {Header::new(locale, &t!("menu.hymnal")).to_node()}
            <main>
                <HymnalSearch
                    locale={locale}
                    search={props.search.clone()}
                    prop:results={props.search_results.clone()}
                    prop:hymnals={props.hymnals.clone()}
                >
                </HymnalSearch>
            </main>
        </>
    }
}
