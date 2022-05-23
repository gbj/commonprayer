// TODO
// - search already given from server side
// - hashtags for categories

mod hymn_wrapper;
mod hymnal_metadata;
mod hymnal_search;

use std::collections::HashSet;

use crate::utils::decode_uri;
use hymnal::*;
use leptos2::*;

use crate::views::*;
use hymnal::{HymnMetadata, HymnNumber, Hymnal, Hymnals};

pub use hymn_wrapper::HymnWrapper;
pub use hymnal_metadata::HymnalMetadataWrapper;
pub use hymnal_search::HymnalSearch;

#[derive(Clone, Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
}

#[derive(Clone, Deserialize, Serialize)]
#[allow(clippy::large_enum_variant)] // only 1 instance, so memory not a huge issue
pub struct HymnalPageState {
    search_results: Option<(HashSet<(Hymnals, HymnNumber)>, String)>,
    hymnal: Vec<(Hymnals, Vec<HymnNumber>)>,
    hymnals: Vec<Hymnal>,
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
            <link rel="stylesheet" href="/static/hymnal.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
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
                HYMNAL_1982.clone(),
                LEVAS.clone(),
                WLP.clone(),
                EL_HIMNARIO.clone(),
            ]
        }
        Some(hymnal_id) => {
            let hymnal: Hymnal = hymnal_id.into();
            vec![hymnal]
        }
    };

    Some(match params.hymnal {
        None => HymnalPageState {
            search_results: search_parts.next().map(|search| {
                let search = decode_uri(search);
                (
                    HYMNAL_1982
                        .search(&search)
                        .map(|number| (Hymnals::Hymnal1982, number))
                        .chain(LEVAS.search(&search).map(|number| (Hymnals::LEVAS, number)))
                        .chain(WLP.search(&search).map(|number| (Hymnals::WLP, number)))
                        .chain(
                            EL_HIMNARIO
                                .search(&search)
                                .map(|number| (Hymnals::ElHimnario, number)),
                        )
                        .collect(),
                    search,
                )
            }),
            hymnal: vec![
                (
                    Hymnals::Hymnal1982,
                    HYMNAL_1982.hymns.iter().map(|hymn| hymn.number).collect(),
                ),
                (
                    Hymnals::LEVAS,
                    LEVAS.hymns.iter().map(|hymn| hymn.number).collect(),
                ),
                (
                    Hymnals::WLP,
                    WLP.hymns.iter().map(|hymn| hymn.number).collect(),
                ),
            ],
            hymnals,
        },
        Some(hymnal_id) => {
            let hymnal: Hymnal = hymnal_id.into();
            HymnalPageState {
                search_results: search_parts.next().map(|search| {
                    let search = decode_uri(search);
                    (
                        hymnal
                            .search(&search)
                            .map(|number| (hymnal_id, number))
                            .collect(),
                        search,
                    )
                }),
                hymnal: vec![(
                    hymnal_id,
                    hymnal.hymns.iter().map(|hymn| hymn.number).collect(),
                )],
                hymnals,
            }
        }
    })
}

pub fn body(locale: &str, props: &HymnalPageState) -> Vec<Node> {
    let search_results = &props.search_results;
    let hymnals = &props.hymnals;

    let hymns_listed = hymnals
        .iter()
        .map(|hymnal| {
            let metadata = hymnal_metadata(hymnal);
            let hymns = hymnal
                .hymns
                .iter()
                .map(|hymn| hymn_metadata(locale, hymnal.id, hymn.number, &hymn.to_metadata()))
                .collect::<Vec<_>>();
            view! {
                <section>
                    {metadata}
                    {hymns}
                </section>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <>
            {Header::new(locale, &t!("menu.hymnal")).to_node()}
            <main>
                <HymnalSearch
                    locale={locale}
                    search={search_results.as_ref().map(|(_, search)| search).cloned().unwrap_or_default()}
                >
                </HymnalSearch>
                {hymns_listed}
            </main>
        </>
    }
}

fn hymnal_metadata(hymnal: &Hymnal) -> Node {
    let title = view! { <h2>{&hymnal.title}</h2> };
    let subtitle = if !hymnal.subtitle.is_empty() {
        Some(view! { <h3>{&hymnal.subtitle}</h3> })
    } else {
        None
    };

    let copyright = view! { <p class="copyright">{&hymnal.copyright}</p> };

    view! {
        <HymnalMetadataWrapper hymnal={hymnal.id}>
            <article class="hymnal">
                {title}
                {subtitle}
                {copyright}
            </article>
        </HymnalMetadataWrapper>
    }
}

fn hymn_metadata(locale: &str, hymnal: Hymnals, number: HymnNumber, hymn: &HymnMetadata) -> Node {
    let link = format!("/{}/hymn/{:#?}/{}", locale, hymnal, number);

    let number_str = match number {
        HymnNumber::S(n) => format!("S{}", n),
        HymnNumber::H(n) => n.to_string(),
    };

    let tune_name = if hymn.tune.starts_with('[') {
        ""
    } else {
        &hymn.tune
    }
    .to_lowercase();

    view! {
        <HymnWrapper hymnal={hymnal} number={number}>
            <article class="hymn-listing"> //data-id={ser_attr!((hymnal, number))}>
                <a id={&format!("{:#?}-{}", hymnal, number_str)}></a>
                <div class="primary">
                    <span class="music-available">
                        {if hymn.copyright_restriction {
                            None
                        } else {
                            Some(view! { <img src={Icon::Music.to_string()} alt={t!("hymnal.music_available")}/> })
                        }}
                    </span>
                    <span class="text-available">
                        {if hymn.text_empty {
                            ""
                        } else {
                            "T"
                        }}
                    </span>
                    <a class="number" href={&link}>{number_str}</a>
                    <a class="title" href={&link}>{&hymn.title}</a>
                    <span class="tune">{&tune_name}</span>
                </div>
                <div class="secondary">
                    <div>
                        {if hymn.authors.is_empty() {
                            None
                        } else {
                            Some(view! {
                                <span class="list-field author">
                                    <span class="label">{t!("hymnal.text")} ": "</span>
                                    {&hymn.authors}
                                </span>
                        })}}
                        {if hymn.composers.is_empty() {
                            None
                        } else {
                            Some(view! {
                                <span class="list-field composer">
                                    <span class="label">{t!("hymnal.music")} ": "</span>
                                    {&hymn.composers}
                                </span>
                        })}}
                    </div>
                    <span class="list-field meter">{&hymn.meter}</span>
                </div>
                <ul class="tag-list">
                    {hymn.tags
                            .iter()
                            .map(|tag| view! {
                                <li>
                                    <a href=&format!("#q=tag:{}", tag)>{tag}</a>
                                </li>
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
            </article>
        </HymnWrapper>
    }
}
