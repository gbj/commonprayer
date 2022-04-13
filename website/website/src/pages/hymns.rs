use std::collections::HashSet;

use crate::{
    api::bing::BingSearchResult,
    components::*,
    utils::{
        decode_uri,
        fetch::{Fetch, FetchStatus},
    },
};
use hymnal::*;
use liturgy::Text;
use futures::StreamExt;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct HymnalPageParams {
    hymnal: Option<Hymnals>,
    number: Option<HymnNumber>,
}

#[derive(Clone, Deserialize, Serialize)]
#[allow(clippy::large_enum_variant)] // only 1 instance, so memory not a huge issue
pub enum HymnalPageHydrationState {
    Hymnal(
        Option<(HashSet<(Hymnals, HymnNumber)>, String)>,
        Vec<(Hymnals, Vec<HymnNumber>)>,
    ),
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
        .incremental_generation()
}

pub fn head(
    _locale: &str,
    props: &HymnalPageHydrationState,
    render_state: &HymnalPageRenderState,
) -> View {
    let title = match props {
        HymnalPageHydrationState::Hymnal(_search, _hymnals) => {
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
            <link rel="stylesheet" href="/static/vars.css"/>
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
    path: &str,
    params: &HymnalPageParams,
) -> Option<HymnalPageHydrationState> {
    let mut search_parts = path.split("?q=");
    search_parts.next();

    Some(match (params.hymnal, params.number) {
        (None, None) => HymnalPageHydrationState::Hymnal(
            search_parts.next().map(|search| {
                let search = decode_uri(search);
                (
                    HYMNAL_1982
                        .search(&search)
                        .map(|number| (Hymnals::Hymnal1982, number))
                        .chain(LEVAS.search(&search).map(|number| (Hymnals::LEVAS, number)))
                        .chain(WLP.search(&search).map(|number| (Hymnals::WLP, number)))
                        .chain(EL_HIMNARIO.search(&search).map(|number| (Hymnals::ElHimnario, number)))
                        .collect(),
                    search,
                )
            }),
            vec![
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
        ),
        (Some(hymnal_id), None) => {
            let hymnal: Hymnal = hymnal_id.into();
            HymnalPageHydrationState::Hymnal(
                search_parts.next().map(|search| {
                    let search = decode_uri(search);
                    (
                        hymnal
                            .search(&search)
                            .map(|number| (hymnal_id, number))
                            .collect(),
                        search,
                    )
                }),
                vec![(
                    hymnal_id,
                    hymnal.hymns.iter().map(|hymn| hymn.number).collect(),
                )],
            )
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
        HymnalPageHydrationState::Hymnal(search, hymnals) => {
            hymnal_body(locale, search, hymnals, &render_state.0)
        }
        HymnalPageHydrationState::Hymn(hymnal, hymn) => hymn_body(locale, hymnal, hymn),
    }
}

pub fn hymnal_body(
    locale: &str,
    search_results: &Option<(HashSet<(Hymnals, HymnNumber)>, String)>,
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
            (Some(Hymnals::ElHimnario), t!("hymnal.el_himnario"), None),
        ],
        if hymnals.len() == 1 {
            hymnals.get(0).map(|h| h.id)
        } else {
            None
        },
    );

    // server-side hymnal API search
    let hash = location_hash().unwrap_or_default();
    let initial_search_value = if let Some((_, search)) = &search_results {
        search.clone()
    } else if hash.starts_with("q=") {
        decode_uri(&hash.replace("q=", ""))
    } else {
        String::default()
    };
    let search_bar = SearchBar::new_with_default_value(initial_search_value);

    let search_state: Behavior<Fetch<HashSet<(Hymnals, HymnNumber)>>> = Behavior::new({
        if let Some(search_results) = search_results {
            Fetch::new_with_status("", FetchStatus::Success(Box::new(search_results.0.clone())))
        } else {
            Fetch::new("")
        }
    });

    let is_searching = search_state
        .stream()
        .flat_map(|fetch| fetch.state.stream())
        .map(|status| matches!(status, FetchStatus::Loading));

    // send fetch when search changes
    search_bar
        .value
        .stream()
        .skip(if search_results.is_some() { 1 } else { 0 })
        .create_effect({
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

    // listen for window hashchange and set search, e.g., #q=... => sets searchbar to ...
    // this can be used to link to certain tags or hymns
    window_event_stream("hashchange").create_effect({
        let search_value = search_bar.value.clone();
        move |_| {
            let hash = location_hash().unwrap_or_default();
            if hash.is_empty() {
                search_value.set("".to_string());
            } else if hash.starts_with("q=") {
                search_value.set(decode_uri(&hash.replace("q=", "")))
            }
        }
    });

    // render hymnal table
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
                                            <a id={&format!("{:#?}-{}", hymn_id.0, hymn_id.1)}></a>
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
                                            <ul class="tag-list">
                                                {View::Fragment(
                                                    hymn.tags
                                                        .iter()
                                                        .map(|tag| view! {
                                                            <li>
                                                                <a href=&format!("#q=tag:{}", tag)>{tag}</a>
                                                            </li>
                                                        })
                                                        .collect()
                                                )}
                                            </ul>
                                        </>
                                    }


                                } else {
                                    View::Empty
                                };

                                let class = if let Some(search_results) = search_results {
                                    if search_results.0.contains(&hymn_id) {
                                        "hymn-listing"
                                    } else {
                                        "hymn-listing hidden"
                                    }
                                } else {
                                    "hymn-listing"
                                };

                                view! {
                                    <dyn:article
                                        class={class}
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
        Hymnals::ElHimnario => "EH1998",
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

    let video_state = Fetch::<BingSearchResult>::new(&format!(
        "/api/hymnal/videos?hymnal={:#?}&number={}",
        hymn.source, hymn.number
    ));

    if hymn.copyright_restriction && hymn.text.is_empty() {
        video_state.send()
    }

    let video_view = video_state
        .state
        .stream()
        .map(|status| match status {
            FetchStatus::Idle => View::Empty,
            FetchStatus::Loading => view! { <p class="loading">{t!("loading")}</p> },
            FetchStatus::Error(_) => view! { <p class="error">{t!("hymnal.video_error")}</p> },
            FetchStatus::Success(result) => match *result {
                BingSearchResult::ErrorResponse(_) => {
                    view! { <p class="error">{t!("hymnal.video_error")}</p> }
                }
                BingSearchResult::Videos(videos) => {
                    let embed : Behavior<Option<String>> = Behavior::new(None);
                    let player = embed.stream()
                        .map(move |embed| match embed {
                            Some(embed_code) => {
                                web_sys::DomParser::new()
                                    .and_then(|parser| parser.parse_from_string(&embed_code, web_sys::SupportedType::TextHtml))
                                    .and_then(|tree| {
                                        tree.query_selector("iframe")
                                    })
                                    .ok()
                                    .flatten()
                                    .and_then(|iframe| iframe.get_attribute("src"))
                                    .map(|src|
                                        view! {
                                            <iframe class="player" src={src} allow="fullscreen"></iframe>
                                        }
                                    )
                                    .unwrap_or_else(|| View::Empty)
                            },
                            None => View::Empty
                        })
                        .boxed_local();

                    let videos_view = View::Fragment(
                        videos
                            .value
                            .iter()
                            .map(move |video| {
                                view! {
                                    <li>
                                        <div class="thumbnail">
                                            <dyn:a on:click={
                                                let embed_html = video.embed_html.clone();
                                                let embed = embed.clone();
                                                move |ev: Event| {
                                                    if let Some(embed_html) = embed_html.as_ref() {
                                                        if embed_html.contains("iframe") && embed_html.contains("src") {
                                                            ev.prevent_default();
                                                            embed.set(Some(embed_html.clone()));
                                                            let video_view = document().get_element_by_id("video-view-label").unwrap();
                                                            video_view.scroll_into_view();
                                                        }
                                                    }
                                                }
                                            }>
                                                <img
                                                    alt=""
                                                    src={video.thumbnail_url.clone().unwrap_or_else(|| "/static/assets/icons/tabler-icon-x.svg".to_string())}
                                                />
                                            </dyn:a>
                                        </div>
                                        <div class="metadata">
                                            <h4>
                                                <a
                                                    href={video.content_url.clone().unwrap_or_else(|| String::from("#"))}
                                                    target="_blank"
                                                >
                                                    {video.name.clone().unwrap_or_default()}
                                                </a>
                                            </h4>
                                            <p class="description">{video.description.clone().unwrap_or_default()}</p>
                                            <p class="creator">
                                                {video.publisher.clone().unwrap_or_default().iter().map(|publisher| publisher.name.clone()).intersperse(" – ".to_string()).collect::<String>()}
                                                " – "
                                                {video.creator.clone().map(|creator| creator.name).unwrap_or_default()}
                                            </p>
                                        </div>
                                    </li>
                                }
                            }
                        )
                        .collect(),
                    );
                    view! {
                        <> 
                            {player}
                            <ul>
                                {videos_view}
                            </ul>
                            <a class="more" target="_blank" href={&videos.web_search_url}>{t!("hymnal.more_results")}</a>
                            <p class="description by-bing">{t!("hymnal.search_by_bing")}</p>
                        </>
                    }
                }
            },
        })
        .boxed_local();

    let rite_song_link = if let Some(link) = rite_song_link(&hymn.source, &hymn.number) {
        view! {
            <>
                <a class="hymnary-link" href={link} target="_blank">"ritesong"</a>
                " "
                {t!("or")}
                " "
            </>
        }
    } else {
        View::Empty
    };

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
                    {rite_song_link}
                    <a class="hymnary-link" href={hymnary_hymn_link} target="_blank">"Hymnary.org"</a>
                    "."
                </p>

                {if hymn.text.is_empty() {
                    View::Empty
                } else {
                    view! {
                        <>
                            <input class="toggle" type="radio" id="text-view" name="view-mode" checked/>
                            <label class="toggle" for="text-view">{t!("hymnal.text_view")}</label>
                        </>
                    }
                }}
                {if hymn.copyright_restriction {
                    View::Empty
                } else {
                    view! {
                        <>
                            <input class="toggle" type="radio" id="image-view" name="view-mode"/>
                            <label class="toggle" for="image-view">{t!("hymnal.music_view")}</label>
                        </>
                    }
                }}
                <dyn:input class="toggle" type="radio" id="video-view" name="view-mode"
                    on:change=move |_ev: Event| video_state.send()
                />
                <label class="toggle" for="video-view" id="video-view-label">{t!("hymnal.video_view")}</label>

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

                <div class={if hymn.copyright_restriction && hymn.text.is_empty() { "video-view force" } else { "video-view" }}>
                    {video_view}
                </div>

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

fn rite_song_link(hymnal: &Hymnals, number: &HymnNumber) -> Option<String> {
    let id = match (hymnal, number) {
        (Hymnals::Hymnal1982, HymnNumber::S(n)) => Some(1353 + (n - 1)),
        (Hymnals::Hymnal1982, HymnNumber::H(n)) => Some(193 + (n - 1)),
        (Hymnals::LEVAS, HymnNumber::H(n)) => Some(913 + (n - 1)),
        (Hymnals::LEVAS, HymnNumber::S(n)) => Some(913 + (n - 1)),
        (Hymnals::WLP, HymnNumber::H(n)) => Some(1968 + (n - 721)),
        (Hymnals::WLP, HymnNumber::S(n)) => Some(1968 + (n - 721)),
        (Hymnals::ElHimnario, _) => None
    };

    let base = match (hymnal, number) {
        (Hymnals::Hymnal1982, HymnNumber::S(_n)) => {
            Some("https://www.riteseries.org/song/Hymnal1982ServiceMusic/")
        }
        (Hymnals::Hymnal1982, HymnNumber::H(_n)) => Some("https://www.riteseries.org/song/Hymnal1982/"),
        (Hymnals::LEVAS, _) =>Some( "https://www.riteseries.org/song/levs/"),
        (Hymnals::WLP, _) => Some("https://www.riteseries.org/song/wlp/"),
        (Hymnals::ElHimnario, _) => None
    };

    if let (Some(base), Some(id)) = (base, id) {
        Some(format!("{}{}/", base, id))
    } else {
        None
    }

}
