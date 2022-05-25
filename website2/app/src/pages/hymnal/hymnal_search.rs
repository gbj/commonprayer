use std::rc::Rc;

use crate::fragments::SegmentButton;
use crate::utils::fetch::{Fetch, FetchMsg, FetchStatus};
use crate::views::Icon;
use hymnal::{HymnMetadata, HymnNumber, Hymnal, Hymnals};
use itertools::Itertools;
use leptos2::*;
use web_sys::AbortController;

#[derive(Debug, Default, WebComponent)]
pub struct HymnalSearch {
    pub locale: String,
    pub search: String,
    #[prop]
    pub hymnals: Vec<Hymnal>,
    #[prop]
    pub hymnal: Option<Hymnals>,
    #[prop]
    pub state: NestedState<Fetch<Vec<HymnMetadata>>>,
}

impl PartialEq for HymnalSearch {
    fn eq(&self, other: &Self) -> bool {
        self.locale == other.locale && self.hymnal == other.hymnal && self.search == other.search
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HymnalSearchMsg {
    Search(String, Option<Hymnals>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HymnalSearchCmd {
    FetchSearch(String, Option<Hymnals>, Option<AbortController>),
    CancelSearch(AbortController, String, Option<Hymnals>),
}

#[async_trait(?Send)]
impl State for HymnalSearch {
    type Msg = HymnalSearchMsg;
    type Cmd = ();

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        match msg {
            HymnalSearchMsg::Search(search, hymnal) => {
                let hymnal_query = hymnal
                    .map(|hymnal| format!("&hymnal={:?}", hymnal))
                    .unwrap_or_default();

                self.state.send(FetchMsg::SetUrlAndGet(format!(
                    "/api/hymnal/search_with_metadata?q={}{}",
                    search, hymnal_query
                )));
            }
        }
        None
    }

    async fn cmd(cmd: Self::Cmd, _host: web_sys::HtmlElement) -> Option<Self::Msg> {
        None
    }

    fn nested_states(&mut self) -> Vec<&mut dyn StateMachine> {
        vec![&mut self.state]
    }
}

impl Component for HymnalSearch {
    fn should_render(&self, _msg: &Self::Msg) -> bool {
        // HymnalSearch only has one event
        // the nested fetch state will cause a rerender when it changes
        // the event itself should never cause a rerender
        false
    }

    fn view(&self) -> Host {
        let state = self.state.with(|state| match &state.status {
            FetchStatus::Idle => view! {
                <p class="idle search-state">{t!("hymnal.search_instruction")}</p>
            },
            FetchStatus::Loading => view! {
                <p class="loading search-state">{t!("loading")}</p>
            },
            FetchStatus::Error(e) => view! {
                <p class="error search-state">{e.to_string()}</p>
            },
            FetchStatus::Success(hymns) => {
                let hymns = hymns
                    .iter()
                    .group_by(|hymn| hymn.source)
                    .into_iter()
                    .flat_map(|(hymnal_id, hymns)| {
                        let hymnal = self
                            .hymnals
                            .iter()
                            .find(|s_hymnal| {
                                s_hymnal.id == hymnal_id
                                    && (self.hymnal.is_none() || self.hymnal == Some(hymnal_id))
                            })
                            .map(hymnal_metadata);

                        let hymns = hymns.into_iter().filter_map(|hymn| {
                            if self.hymnal.is_none() || self.hymnal == Some(hymn.source) {
                                Some(hymn_metadata(&self.locale, hymn))
                            } else {
                                None
                            }
                        });

                        hymnal.into_iter().chain(hymns)
                    })
                    .collect::<Vec<_>>();

                view! {
                    <section>{hymns}</section>
                }
            }
        });

        let current_hymnal = self.hymnal.clone();

        view! {
            <Host>
                <style>
                {include_str!("../../components/toggle.css")}
                {include_str!("./hymnal_search.css")}
                </style>
                // Choose a hymnal
                {if self.hymnals.len() == 1 {
                    None
                } else {
                    {Some(self.choose_hymnal().to_node())}
                }}

                // Search bar
                <label class="stacked">
                    {t!("search")}
                    <input
                        type="search"
                        value={&self.search}
                        prop:value={self.search.clone()}
                        on:input=move |ev| HymnalSearchMsg::Search(event_target_value(&ev), current_hymnal)
                    />
                </label>

                {state}
            </Host>
        }
    }
}

impl HymnalSearch {
    fn is_loading(&self) -> bool {
        self.state.with(|state| state.is_loading())
    }

    fn choose_hymnal(&self) -> SegmentButton<Option<Hymnals>, HymnalSearchMsg> {
        let current_search = self.search.clone();

        SegmentButton {
            name: "search-hymnal".to_string(),
            legend: t!("menu.hymnal"),
            options: vec![
                (None, t!("hymnal.any"), None),
                (Some(Hymnals::Hymnal1982), t!("hymnal.h82_abbrev"), None),
                (Some(Hymnals::LEVAS), t!("hymnal.levas_abbrev"), None),
                (Some(Hymnals::WLP), t!("hymnal.wlp_abbrev"), None),
                (Some(Hymnals::ElHimnario), t!("hymnal.el_himnario"), None),
            ],
            value: self.hymnal,
            msg_fn: Rc::new(move |val| HymnalSearchMsg::Search(current_search.clone(), *val)),
        }
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
        <article class="hymnal">
            {title}
            {subtitle}
            {copyright}
        </article>
    }
}

fn hymn_metadata(locale: &str, hymn: &HymnMetadata) -> Node {
    let link = format!("/{}/hymn/{:#?}/{}", locale, hymn.source, hymn.number);

    let number_str = match hymn.number {
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
        <article class="hymn-listing">
            <a id={&format!("{:#?}-{}", hymn.source, number_str)}></a>
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
                                <a href=&format!("?q=tag:{}", tag)>{tag}</a>
                            </li>
                        })
                        .collect::<Vec<_>>()
                }
            </ul>
        </article>
    }
}
