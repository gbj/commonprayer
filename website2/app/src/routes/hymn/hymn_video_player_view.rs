use std::{pin::Pin, str::FromStr};

use cached::proc_macro::cached;
use futures::Future;
use leptos2::*;
use reqwest::header::{HeaderName, HeaderValue};

use crate::api::bing::VideoResult;

use super::hymn_video_view::{BING_SEARCH_API_KEY, CACHED_VIDEO_EMBED_CODES};

pub struct HymnVideoPlayerView {
    result: EmbedCode,
}

pub enum EmbedCode {
    Cached(String),
    Async(Pin<Box<dyn Future<Output = Result<VideoResult, Arc<reqwest::Error>>> + Send + Sync>>),
}

#[derive(Params)]
pub struct HymnVideoPlayerQuery {
    id: String,
}

#[async_trait(?Send)]
impl Loader for HymnVideoPlayerView {
    type Params = ();
    type Query = HymnVideoPlayerQuery;

    async fn loader(
        _locale: &str,
        _req: Arc<dyn Request>,
        _params: Self::Params,
        query: Self::Query,
    ) -> Option<Self> {
        let result = match CACHED_VIDEO_EMBED_CODES.get(&query.id) {
            Some(cached) => EmbedCode::Cached(cached),
            None => EmbedCode::Async(Box::pin(bing_video_detail(query.id))),
        };
        Some(Self { result })
    }
}

impl View for HymnVideoPlayerView {
    fn title(&self) -> String {
        String::new()
    }

    fn styles(&self) -> Styles {
        vec![include_str!("hymn_video_view.css").into()]
    }

    fn body(self: Box<Self>, _nested_view: Option<Node>) -> Body {
        let player = match self.result {
            EmbedCode::Cached(embed) => Node::Element(Element::new("div").inner_html(embed)),
            EmbedCode::Async(pending) => {
                let pending = async {
                    match pending.await {
                        Err(e) => {
                            eprintln!("[HymnVideoPlayerView] {}", e);
                            view! { <p class="error">{t!("hymnal.video_error")}</p> }
                        }
                        Ok(res) => match res.embed_html {
                            Some(embed) => Node::Element(Element::new("div").inner_html(embed)),
                            None => view! { <p class="error">{t!("hymnal.video_error")}</p> },
                        },
                    }
                };
                Node::AsyncElement(AsyncElement::from((
                    view! { <p class="loading">{t!("loading")}</p> },
                    pending,
                )))
            }
        };

        view! {
            <div class="video-view">
                {player}
            </div>
        }
    }
}

// cache search results for one day
#[cached(time = 86_400)]
pub async fn bing_video_detail(id: String) -> Result<VideoResult, Arc<reqwest::Error>> {
    let url =
        format!("https://api.bing.microsoft.com/v7.0/videos/details?id={id}&modules=VideoResult");

    let data: BingVideoDetails = reqwest::Client::new()
        .get(&url)
        .header(
            HeaderName::from_str("Ocp-Apim-Subscription-Key").expect("couldn't use header name"),
            HeaderValue::from_static(&BING_SEARCH_API_KEY),
        )
        .send()
        .await
        .map_err(Arc::new)?
        .json()
        .await
        .map_err(Arc::new)?;

    Ok(data.video_result)
}

#[derive(Deserialize, Hash, PartialEq, Eq)]
pub struct BingVideoDetails {
    #[serde(alias = "videoResult")]
    video_result: VideoResult,
}
