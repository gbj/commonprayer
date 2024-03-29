use crate::{
    utils::fetch::{Fetch, FetchMsg},
    WebView,
};
use hymnal::{HymnNumber, Hymnals};
use leptos2::*;
use liturgy::Text;
use strum_macros::{Display, EnumString};

use crate::{
    api::bing::BingSearchResult,
    utils::fetch::{fetch, FetchError, FetchStatus},
    views::Icon,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct HymnMedia {
    pub locale: String,
    pub hymnal: Hymnals,
    pub number: HymnNumber,
    pub copyright: bool,
    pub text: String,
    pub mode: HymnMediaShowing,
    pub page: u32,
    page_scan_expanded: bool,
    video_results: NestedState<Fetch<BingSearchResult>>,
    video_player_embed_code: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, EnumString, Display)]
pub enum HymnMediaShowing {
    Text,
    PageScan,
    Video,
}

impl Default for HymnMediaShowing {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum HymnMediaMsg {
    ChangeMode(String),
    PageScanBack,
    PageScanForward,
    PageScanExpandedToggle,
    SetEmbedCode(usize),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum HymnMediaCmd {
    ScrollToVideoPlayer,
}

impl State for HymnMedia {
    type Msg = HymnMediaMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            HymnMediaMsg::ChangeMode(mode) => {
                let mode = match mode.as_str() {
                    "text" => HymnMediaShowing::Text,
                    "page" => HymnMediaShowing::PageScan,
                    "video" => HymnMediaShowing::Video,
                    _ => HymnMediaShowing::Text,
                };
                self.mode = mode;
                if self.mode == HymnMediaShowing::Video {
                    self.fetch_videos();
                }
            }
            HymnMediaMsg::PageScanBack => {
                let new_page = self.page - 1;
                if new_page >= 1 {
                    self.page = new_page;
                }
            }
            HymnMediaMsg::PageScanForward => self.page += 1,
            HymnMediaMsg::PageScanExpandedToggle => {
                self.page_scan_expanded = !self.page_scan_expanded
            }
            HymnMediaMsg::SetEmbedCode(idx) => {
                let embed = self.video_results.with(|res| match &res.status {
                    FetchStatus::Success(res) => match &**res {
                        BingSearchResult::Videos(videos) => videos
                            .value
                            .get(idx)
                            .and_then(|video| video.embed_html.clone()),
                        BingSearchResult::ErrorResponse(_) => None,
                        BingSearchResult::Empty => None,
                    },
                    _ => None,
                });
                let has_video = embed.is_some();
                self.video_player_embed_code = embed;
                if has_video {
                    return Some(self.scroll_to_video_player());
                }
            }
        }
        None
    }

    fn nested_states(&mut self) -> Vec<&mut dyn StateMachine> {
        vec![&mut self.video_results]
    }
}

impl HymnMedia {
    fn fetch_videos(&self) {
        if self.videos_idle() {
            let url = format!(
                "/api/hymnal/videos?hymnal={:#?}&number={}",
                self.hymnal, self.number
            );
            self.video_results.send(FetchMsg::SetUrlAndGet(url));
        }
    }

    fn videos_idle(&self) -> bool {
        self.video_results
            .with(|state| state.status == FetchStatus::Idle)
    }
}

impl Component for HymnMedia {
    fn view(&self) -> Host {
        let hymnary_hymnal_id = match self.hymnal {
            Hymnals::Hymnal1982 => "EH1982",
            Hymnals::LEVAS => "LEVS1993",
            Hymnals::WLP => "WLP1997",
            Hymnals::ElHimnario => "EH1998",
        };

        view! {
            <Host>
                <style>
                    {include_str!("hymn_media.css")}
                    {include_str!("../../components/toggle.css")}
                </style>
                <link rel="stylesheet" href="/static/document.css"/>
                // Toggle to select mode
                <fieldset class="toggle segment-button">
                    {if self.text.is_empty() {
                        vec![]
                    } else {
                        view! {
                            <>
                            <input class="toggle" type="radio" id="text-view" name="view-mode"
                                value="text"
                                prop:checked={self.mode == HymnMediaShowing::Text}
                                on:change=|ev| HymnMediaMsg::ChangeMode(event_target_value(&ev))
                            />
                            <label class="toggle" for="text-view">{t!("hymnal.text_view")}</label>
                            </>
                        }
                    }}
                    {if self.copyright {
                        vec![]
                    } else {
                        view!{
                            <>
                                <input class="toggle" type="radio" id="image-view" name="view-mode"
                                    value="page"
                                    prop:checked={self.mode == HymnMediaShowing::PageScan}
                                    on:change=|ev| HymnMediaMsg::ChangeMode(event_target_value(&ev))
                                />
                                <label class="toggle" for="image-view">{t!("hymnal.music_view")}</label>
                            </>
                        }
                    }}
                    <input class="toggle" type="radio" id="video-view" name="view-mode"
                        value="video"
                        prop:checked={self.mode == HymnMediaShowing::Video}
                        on:change=|ev| HymnMediaMsg::ChangeMode(event_target_value(&ev))
                    />
                    <label class="toggle" for="video-view" id="video-view-label">{t!("hymnal.video_view")}</label>
                </fieldset>

                // Text display
                <div class="text-view" class:hidden={self.mode != HymnMediaShowing::Text}>
                    {Text::from(self.text.clone()).view(&self.locale)}
                </div>

                // Page scan display
                <div class="image-view" class:hidden={self.mode != HymnMediaShowing::PageScan}>
                    <div class="overlay"
                        class:expanded={self.page_scan_expanded}
                        on:click=|_| HymnMediaMsg::PageScanExpandedToggle
                    ></div>
                    <div class="page-scan-controls"
                        class:expanded={self.page_scan_expanded}
                    >
                        <button
                            class="page-left"
                            on:click=|_| HymnMediaMsg::PageScanBack
                        >
                            <img src={Icon::Left} alt={t!("hymnal.page_back")}/>
                        </button>
                        <p class="page-scan-number">
                            {t!("hymnal.page_n", number = &self.page.to_string())}
                        </p>
                        <button
                            class="page-right"
                            on:click=|_| HymnMediaMsg::PageScanForward
                        >
                            <img src={Icon::Right.to_string()} alt={t!("hymnal.page_forward")}/>
                        </button>
                    </div>
                    {if self.copyright {
                        view! {
                            <p class="page-scan">{t!("hymnal.copyright_restriction")}</p>
                        }
                    } else {
                        view! {
                            <img
                                src={format!(
                                    "https://hymnary.org/page/fetch/{}/{}/high",
                                    &hymnary_hymnal_id, self.page
                                )}
                                alt={t!("hymnal.alt_text")}
                                class="page-scan"
                                class:expanded={self.page_scan_expanded}
                                on:click=|_| HymnMediaMsg::PageScanExpandedToggle
                            />
                        }
                    }}
                </div>

                // Video display
                <div class="video-view" class:hidden={self.mode != HymnMediaShowing::Video}>
                    {self.video_player()}
                    {self.videos()}
                    <p class="description by-bing">{t!("hymnal.search_by_bing")}</p>
                </div>
            </Host>
        }
    }
}

impl HymnMedia {
    fn videos(&self) -> Node {
        self.video_results.with(|state| match &state.status {
            FetchStatus::Idle => text(t!("loading")),
            FetchStatus::Loading => text(t!("loading")),
            FetchStatus::Error(_) => view! {
                <p class="error">{t!("hymnal.video_error")}</p>
            },
            FetchStatus::Success(result) => match &**result {
                BingSearchResult::Empty => { view! { <p></p> }}
                BingSearchResult::ErrorResponse(_) => {
                    view! { <p class="error">{t!("hymnal.video_error")}</p> }
                }
                BingSearchResult::Videos(videos) => {
                    let videos_view = videos
                            .value
                            .iter()
                            .enumerate()
                            .map(|(idx, video)| {
                                view! {
                                    <li>
                                        <div class="thumbnail">
                                            <a on:click=move |ev: Event| {
                                                    ev.prevent_default();
                                                    HymnMediaMsg::SetEmbedCode(idx)
                                                }>
                                                <img
                                                    alt=""
                                                    src={video.thumbnail_url.clone().unwrap_or_else(|| "/static/assets/icons/tabler-icon-x.svg".to_string())}
                                                />
                                            </a>
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
                        .collect::<Vec<_>>();
                    view! {
                        <div>
                            <ul>{videos_view}</ul>
                            <a class="more" target="_blank" href={&videos.web_search_url}>{t!("hymnal.more_results")}</a>
                        </div>
                    }
                }
            },
        })
    }

    fn video_player(&self) -> Option<Node> {
        self.video_player_embed_code
            .as_ref()
            .and_then(|embed_code| {
                web_sys::DomParser::new()
                    .and_then(|parser| {
                        parser.parse_from_string(embed_code, web_sys::SupportedType::TextHtml)
                    })
                    .and_then(|tree| tree.query_selector("iframe"))
                    .ok()
                    .flatten()
                    .and_then(|iframe| iframe.get_attribute("src"))
                    .map(|src| {
                        view! {
                            <iframe class="player" src={src} allow="fullscreen"></iframe>
                        }
                    })
            })
    }
}

impl HymnMedia {
    fn scroll_to_video_player(&self) -> Cmd<Self> {
        Cmd::new(|host, _| {
            let video_view = host
                .shadow_root()
                .unwrap()
                .get_element_by_id("video-view-label")
                .unwrap();
            video_view.scroll_into_view();
        })
    }
}
