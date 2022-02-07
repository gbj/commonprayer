use std::collections::{HashMap, HashSet};

use crate::{
    components::*,
    utils::fetch::{Fetch, FetchStatus},
};
use episcopal_api::hymnal::*;
use episcopal_api::liturgy::Text;
use futures::StreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
    number: Option<HymnNumber>,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum HymnalPageHydrationState {
    Hymnal(Vec<(Hymnals, Vec<HymnNumber>)>),
    Hymn(HymnalMetadata, Hymn),
}

#[derive(Default, Clone)]
pub struct HymnalPageRenderState(Vec<Hymnal>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HymnalMetadata {
    pub id: Hymnals,
    pub title: String,
    pub subtitle: String,
    pub copyright: String,
}

pub fn hymnal() -> Page<HymnalPageHydrationState, HymnalPageParams, HymnalPageRenderState> {
    Page::new("hymnal")
        .head_fn(head)
        .body_fn(body)
        .hydration_state(hydration_state)
        .render_state(render_state)
        .build_paths_fn(get_static_paths)
}

pub fn head(
    _locale: &str,
    props: &HymnalPageHydrationState,
    render_state: &HymnalPageRenderState,
) -> View {
    let title = match props {
        HymnalPageHydrationState::Hymnal(_hymnals) => {
            if render_state.0.len() == 1 {
                render_state.0[0].title.clone()
            } else {
                t!("menu.hymnal")
            }
        }
        HymnalPageHydrationState::Hymn(_, hymn) => format!("{} {}", hymn.number, hymn.title),
    };

    view! {
        <>
            <title>{title} " – " {t!("common_prayer")}</title>
            <link rel="stylesheet" href="/static/general.css"/>
            <link rel="stylesheet" href="/static/hymnal.css"/>
            <link rel="stylesheet" href="/static/document.css"/>
        </>
    }
}

pub fn get_static_paths() -> Vec<String> {
    vec!["".into(), "{hymnal}".into(), "{hymnal}/{number}".into()]
}

pub fn hydration_state(
    _locale: &str,
    _path: &str,
    params: &HymnalPageParams,
) -> Option<HymnalPageHydrationState> {
    Some(match (params.hymnal, params.number) {
        (None, None) => HymnalPageHydrationState::Hymnal(vec![
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
        ]),
        (Some(hymnal_id), None) => {
            let hymnal: Hymnal = hymnal_id.into();
            HymnalPageHydrationState::Hymnal(vec![(
                hymnal_id,
                hymnal.hymns.iter().map(|hymn| hymn.number).collect(),
            )])
        }
        (hymnal, Some(number)) => {
            let hymnal: Hymnal = hymnal.unwrap_or_default().into();
            let metadata = HymnalMetadata {
                id: hymnal.id,
                title: hymnal.title,
                subtitle: hymnal.subtitle,
                copyright: hymnal.copyright,
            };
            let hymn = hymnal.hymns.iter().find(|s_hymn| s_hymn.number == number)?;
            HymnalPageHydrationState::Hymn(metadata, hymn.clone())
        }
    })
}

pub fn render_state(
    _locale: &str,
    _path: &str,
    params: &HymnalPageParams,
) -> Option<HymnalPageRenderState> {
    Some(match (params.hymnal, params.number) {
        (None, None) => {
            HymnalPageRenderState(vec![HYMNAL_1982.clone(), LEVAS.clone(), WLP.clone()])
        }
        (Some(hymnal_id), None) => {
            let hymnal: Hymnal = hymnal_id.into();
            HymnalPageRenderState(vec![hymnal])
        }
        (_hymnal, Some(_)) => HymnalPageRenderState(Vec::new()),
    })
}

pub fn body(
    locale: &str,
    props: &HymnalPageHydrationState,
    render_state: &HymnalPageRenderState,
) -> View {
    match props {
        HymnalPageHydrationState::Hymnal(hymnals) => hymnal_body(locale, hymnals, &render_state.0),
        HymnalPageHydrationState::Hymn(hymnal, hymn) => hymn_body(locale, hymnal, hymn),
    }
}

pub fn hymnal_body(
    locale: &str,
    hymnal_tocs: &[(Hymnals, Vec<HymnNumber>)],
    hymnals: &[Hymnal],
) -> View {
    let hymnal_choice = SegmentButton::new_with_default_value(
        "search-hymnal",
        Some(t!("menu.hymnal")),
        [
            (None, t!("hymnal.any"), None),
            (Some(Hymnals::Hymnal1982), t!("hymnal.h82_abbrev"), None),
            (Some(Hymnals::LEVAS), t!("hymnal.levas_abbrev"), None),
            (Some(Hymnals::WLP), t!("hymnal.wlp_abbrev"), None),
        ],
        if hymnals.len() == 1 {
            hymnals.get(0).map(|h| h.id)
        } else {
            None
        },
    );

    // server-side hymnal API search
    let search_bar = SearchBar::new();

    let search_state: Behavior<Fetch<HashSet<(Hymnals, HymnNumber)>>> =
        Behavior::new(Fetch::new(""));

    let is_searching = search_state
        .stream()
        .flat_map(|fetch| fetch.state.stream())
        .map(|status| matches!(status, FetchStatus::Loading));

    // send fetch when search changes
    search_bar.value.stream().create_effect({
        let search_state = search_state.clone();
        move |search| {
            // abort in-flight requests
            let current_request = search_state.get();
            if current_request.state.get() == FetchStatus::Loading {
                current_request.abort();
            }

            // send request for new search
            if !search.is_empty() {
                let new_request = Fetch::new(&format!("/api/hymnal/search?q={}", search));
                new_request.send();
                search_state.set(new_request);
            }
            // if you've cleared the search, reset
            else {
                search_state.set(Fetch::new(""));
            }
        }
    });

    // TODO
    // 1) iterate over HYDRATE state, instead of render state
    // 2) for each item, look up the hymn data in the render state — all these static elements will only be created then anyway
    // 3) use the hydrate state to manage the search interface

    let hymnal_tables = View::Fragment(
        hymnal_tocs
            .iter()
            .enumerate()
            .map({
                let hymnal_choice = hymnal_choice.value.clone();
                move |(hymnal_idx, (id, hymn_numbers))| {
                    let hymnal = hymnals.get(hymnal_idx);

                    let hymnal_metadata = if let Some(hymnal) = hymnal {
                        let title = view! { <h2>{&hymnal.title}</h2> };
                        let subtitle = if !hymnal.subtitle.is_empty() {
                            view! { <h3>{&hymnal.subtitle}</h3> }
                        } else {
                            View::Empty
                        };
                        let copyright = view! { <p class="copyright">{&hymnal.copyright}</p> };

                        view! {
                            {title}
                            {subtitle}
                            {copyright}
                        }
                    } else {
                        View::Empty
                    };

                    let hidden = hymnal_choice
                        .stream()
                        .map({
                            let id = *id;
                            move |choice| choice.is_some() && choice != Some(id)
                        })
                        .boxed_local();

                    let hymns = View::Fragment(
                        hymn_numbers
                            .iter()
                            .map(|number| {
                                let hymn = hymnal.and_then(|hymnal| {
                                    hymnal.hymns.iter().find(|s_hymn| s_hymn.number == *number)
                                });

                                let hymn_id = (*id, *number);

                                let hidden = search_state
                                    .stream()
                                    .flat_map(|fetch| fetch.state.stream())
                                    .filter_map({
                                        let source = hymn_id.0;
                                        let number = hymn_id.1;
                                        move |status| async move {
                                            match status {
                                                // if no search has been sent, show everything
                                                FetchStatus::Idle => Some(false),
                                                // if still loading, or if there's an error, make no changes
                                                FetchStatus::Loading => None,
                                                FetchStatus::Error(_) => None,
                                                // if we have results, check if this hymn is in them
                                                FetchStatus::Success(matches) => {
                                                    Some(!matches.contains(&(source, number)))
                                                },
                                            }
                                        }
                                    })
                                    .boxed_local();

                                let link =
                                    format!("/{}/hymnal/{:#?}/{}", locale, hymn_id.0, hymn_id.1);

                                let hymn_metadata = if let Some(hymn) = hymn {
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

                                    view! {
                                        <>
                                            <div class="primary">
                                                <span class="music-available">
                                                    {if hymn.copyright_restriction {
                                                        View::Empty
                                                    } else {
                                                        view! { <img src={Icon::Music.to_string()} alt={t!("hymnal.music_available")}/> }
                                                    }}
                                                </span>
                                                <span class="text-available">
                                                    {if hymn.text.is_empty() {
                                                        ""
                                                    } else {
                                                        "T"
                                                    }}
                                                </span>
                                                <a class="number" href={&link}>{number}</a>
                                                <a class="title" href={&link}>{&hymn.title}</a>
                                                <span class="tune">{&tune_name}</span>
                                            </div>
                                            <div class="secondary">
                                                <div>
                                                    {if hymn.authors.is_empty() {
                                                        View::Empty
                                                    } else {
                                                        view! {
                                                            <span class="list-field author">
                                                                <span class="label">{t!("hymnal.text")} ": "</span>
                                                                {&hymn.authors}
                                                            </span>
                                                    }}}
                                                    {if hymn.composers.is_empty() {
                                                        View::Empty
                                                    } else {
                                                        view! {
                                                            <span class="list-field composer">
                                                                <span class="label">{t!("hymnal.music")} ": "</span>
                                                                {&hymn.composers}
                                                            </span>
                                                    }}}
                                                </div>
                                                <span class="list-field meter">{&hymn.meter}</span>
                                            </div>
                                        </>
                                    }


                                } else {
                                    View::Empty
                                };

                                view! {
                                    <dyn:article
                                        class="hymn-listing"
                                        class:hidden={hidden}
                                    >
                                        {hymn_metadata}
                                    </dyn:article>
                                }
                            })
                            .collect(),
                    );

                    view! {
                        <dyn:section class:hidden={hidden}>
                            {hymnal_metadata}
                            <table>
                                {hymns}
                            </table>
                        </dyn:section>
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
                <dyn:p class="search-state hidden"
                    class:hidden={is_searching.map(|n| !n).boxed_local()}
                >
                    {t!("loading")}
                </dyn:p>
                {if hymnals.len() == 1 {
                    View::Empty
                } else {
                    hymnal_choice.view()
                }}
                {hymnal_tables}
            </main>
        </>
    }
}

fn hymn_body(locale: &str, hymnal: &HymnalMetadata, hymn: &Hymn) -> View {
    let hymnary_hymnal_id = match hymnal.id {
        Hymnals::Hymnal1982 => "EH1982",
        Hymnals::LEVAS => "LEVS1993",
        Hymnals::WLP => "WLP1997",
    };
    let hymnary_hymn_link = format!(
        "https://hymnary.org/hymn/{}/{}",
        &hymnary_hymnal_id, hymn.number
    );

    let image_expanded = Behavior::new(false);

    // page scrolling through hymnal
    let initial_page: i32 = hymn.page_number.into();
    let page_scan_offset = Behavior::new(0);
    let page_scan_url = page_scan_offset
        .stream()
        .map(move |offset| {
            let current_page = initial_page + offset;
            format!(
                "https://hymnary.org/page/fetch/{}/{}/high",
                &hymnary_hymnal_id, current_page
            )
        })
        .boxed_local();

    view! {
        <>
            {header(locale, &format!("{} {}", hymn.number, hymn.title))}
            <main>
                <h2>
                    <a href={&format!("/{}/hymnal/{:#?}", locale, hymnal.id)}>
                        {&hymnal.title}
                    </a>
                    " "
                    {hymn.number.to_string()}
                </h2>

                // Hymn metadata
                <h3>{&hymn.title}</h3>

                <dl>
                    <dt>{t!("hymnal.tune")}</dt>
                    <dd class="tune">{hymn.tune.to_lowercase()}</dd>
                    {possible_field(&t!("hymnal.authors"), &hymn.authors)}
                    {possible_field(&t!("hymnal.composers"), &hymn.composers)}
                    {possible_field(&t!("hymnal.meter"), &hymn.meter)}
                    {possible_field(&t!("hymnal.text_sources"), &hymn.text_sources)}
                    {possible_field(&t!("hymnal.tune_sources"), &hymn.tune_sources)}
                </dl>
                // Links to RiteSong and Hymnary
                <p class="hymnary-link">
                    {t!("hymnal.more_info")}
                    " "
                    <a class="hymnary-link" href={rite_song_link(&hymn.source, &hymn.number)} target="_blank">"ritesong"</a>
                    " "
                    {t!("or")}
                    " "
                    <a class="hymnary-link" href={hymnary_hymn_link} target="_blank">"Hymnary.org"</a>
                    "."
                </p>

                {if hymn.text.is_empty() || hymn.copyright_restriction {
                    View::Empty
                } else {
                    view! {
                        <>
                            <input class="toggle" type="radio" id="text-view" name="view-mode" checked/>
                            <label class="toggle" for="text-view">{t!("hymnal.text_view")}</label>
                            <input class="toggle" type="radio" id="image-view" name="view-mode"/>
                            <label class="toggle" for="image-view">{t!("hymnal.music_view")}</label>
                        </>
                    }
                }}

                // Hymn text
                {if hymn.text.is_empty() {
                    View::Empty
                } else {
                    view! {
                        <div class="text-view">{text(&Text::from(hymn.text.clone())).1}</div>
                    }
                }}

                // Hymn image
                {if hymn.copyright_restriction {
                    View::Empty
                } else {
                    view! {
                        <div class="image-view">
                            <dyn:div class="overlay"
                                class:expanded={image_expanded.stream().boxed_local()}
                                on:click={
                                    let image_expanded = image_expanded.clone();
                                    move |_ev: Event| image_expanded.set(!image_expanded.get())
                                }
                            ></dyn:div>

                            <dyn:div class="page-scan-controls"
                                class:expanded={image_expanded.stream().boxed_local()}
                            >
                                <dyn:button
                                    class="page-left"
                                    on:click={
                                        let page_scan_offset = page_scan_offset.clone();
                                        move |_ev: Event| {
                                            let current_offset = page_scan_offset.get();
                                            let current_page = initial_page + current_offset;
                                            if current_page > 1 {
                                                page_scan_offset.set(current_offset - 1);
                                            }
                                        }
                                    }
                                >
                                    <img src={Icon::Left.to_string()} alt={t!("hymnal.page_back")}/>
                                </dyn:button>
                                <dyn:p class="page-scan-number">
                                    {page_scan_offset.stream().map(move |offset| t!("hymnal.page_n", number = &(initial_page + offset).to_string() )).boxed_local()}
                                </dyn:p>
                                <dyn:button
                                    class="page-left"
                                    on:click=move |_ev: Event| page_scan_offset.set(page_scan_offset.get() + 1)
                                >
                                    <img src={Icon::Right.to_string()} alt={t!("hymnal.page_forward")}/>
                                </dyn:button>
                            </dyn:div>

                            {if hymn.copyright_restriction {
                                view! {
                                    <p class="page-scan">{t!("hymnal.copyright_restriction")}</p>
                                }
                            } else {
                                view! {
                                    <dyn:img
                                        src={page_scan_url}
                                        alt={t!("hymnal.alt_text")}
                                        class="page-scan"
                                        class:expanded={image_expanded.stream().boxed_local()}
                                        on:click=move |_ev: Event| image_expanded.set(!image_expanded.get())
                                    />
                                }
                            }}
                        </div>
                    }
                }}

                // Copyright notice in footer
                <footer>
                    {t!("hymnal.copyright_footer")}
                </footer>
            </main>
        </>
    }
}

fn possible_field(label: &str, value: &str) -> View {
    if value.is_empty() {
        View::Empty
    } else {
        view! {
            <>
                <dt>{label}</dt>
                <dd>{escape_italics(value)}</dd>
            </>
        }
    }
}

fn escape_italics(original: &str) -> View {
    View::Fragment(
        original
            .split("<i>")
            .flat_map(|piece| piece.split("</i>"))
            .enumerate()
            .map(|(idx, piece)| {
                if idx % 2 == 0 {
                    View::StaticText(piece.to_string())
                } else {
                    // every odd character piece will be *after* a <i> but before a </i>
                    view! { <i>{piece}</i> }
                }
            })
            .collect(),
    )
}

fn rite_song_link(hymnal: &Hymnals, number: &HymnNumber) -> String {
    let id = match (hymnal, number) {
        (Hymnals::Hymnal1982, HymnNumber::S(n)) => 1353 + (n - 1),
        (Hymnals::Hymnal1982, HymnNumber::H(n)) => 193 + (n - 1),
        (Hymnals::LEVAS, HymnNumber::H(n)) => 913 + (n - 1),
        (Hymnals::LEVAS, HymnNumber::S(n)) => 913 + (n - 1),
        (Hymnals::WLP, HymnNumber::H(n)) => 1968 + (n - 721),
        (Hymnals::WLP, HymnNumber::S(n)) => 1968 + (n - 721),
    };

    let base = match (hymnal, number) {
        (Hymnals::Hymnal1982, HymnNumber::S(_n)) => {
            "https://www.riteseries.org/song/Hymnal1982ServiceMusic/"
        }
        (Hymnals::Hymnal1982, HymnNumber::H(_n)) => "https://www.riteseries.org/song/Hymnal1982/",
        (Hymnals::LEVAS, _) => "https://www.riteseries.org/song/levs/",
        (Hymnals::WLP, _) => "https://www.riteseries.org/song/wlp/",
    };

    format!("{}{}/", base, id)
}
