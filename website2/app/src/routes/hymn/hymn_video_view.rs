use std::{pin::Pin, str::FromStr};

use cached::proc_macro::cached;
use futures::Future;
use hymnal::{Hymn, HymnNumber, Hymnal, Hymnals};
use leptos2::*;
use reqwest::header::{HeaderName, HeaderValue};

use crate::api::bing::BingSearchResult;

use super::HymnViewParams;

pub struct HymnVideoView {
    locale: String,
    hymnal: Hymnals,
    number: HymnNumber,
    video_results:
        Pin<Box<dyn Future<Output = Result<BingSearchResult, Arc<reqwest::Error>>> + Send + Sync>>,
}

#[async_trait(?Send)]
impl Loader for HymnVideoView {
    type Params = HymnViewParams;
    type Query = ();

    async fn loader(
        locale: &str,
        _req: Arc<dyn Request>,
        params: Self::Params,
        _query: Self::Query,
    ) -> Option<Self> {
        let hymnal: Hymnal = params.hymnal.into();
        let hymn = hymnal
            .hymns
            .iter()
            .find(|s_hymn| s_hymn.number == params.number)?
            .clone();

        let number = hymn.number;
        let video_results = Box::pin(bing_search(hymn));
        Some(HymnVideoView {
            locale: locale.to_string(),
            hymnal: hymnal.id,
            number,
            video_results,
        })
    }
}

impl View for HymnVideoView {
    fn title(&self) -> String {
        String::new()
    }

    fn styles(&self) -> Styles {
        vec![include_str!("hymn_video_view.css").into()]
    }

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body {
        let pending = async {
            match self.video_results.await {
                Err(e) => {
                    eprintln!("[HymnVideoView {:?} {}] {}", self.hymnal, self.number, e);
                    view! { <p class="error">{t!("hymnal.video_error")}</p> }
                }
                Ok(res) => match res {
                    BingSearchResult::Empty => {
                        view! { <p class="loading">{t!("loading")}</p> }
                    }
                    BingSearchResult::ErrorResponse(_) => {
                        view! { <p class="error">{t!("hymnal.video_error")}</p> }
                    }
                    BingSearchResult::Videos(videos) => {
                        let videos_view = videos
                            .value
                            .iter()
                            .map(|video| {
								let href = if video.allow_https_embed && video.embed_html.is_some() {
									format!("/{}/hymn/{:?}/{}/video/play?id={}", self.locale, self.hymnal, self.number, video.id)
								} else {
									video.content_url.clone().unwrap_or_default()
								};

                                view! {
                                    <li>
                                        <div class="thumbnail">
                                            <a href={href}>
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
                                {nested_view}
                                <ul>{videos_view}</ul>
                                <a class="more" target="_blank" href={&videos.web_search_url}>{t!("hymnal.more_results")}</a>
                            </div>
                        }
                    }
                },
            }
        };

        view! {
            <div class="video-view">
                {(
                    view! { <p class="loading">{t!("loading")}</p> },
                    pending
                )}
            </div>
        }
    }
}

lazy_static::lazy_static! {
    pub static ref BING_SEARCH_API_KEY: String =
        std::env::var("BING_SEARCH_API_KEY").expect("BING_SEARCH_API_KEY not set");

    pub static ref CACHED_VIDEO_EMBED_CODES: moka::sync::Cache<String, String> = moka::sync::Cache::builder()
        // cache embed codes for 10 minutes from the time of the search (or being viewe)
        .time_to_idle(std::time::Duration::from_secs(10*60))
        .build();
}

const BING_ENDPOINT: &str = "https://api.bing.microsoft.com/v7.0/videos/search";

// cache search results for one day
#[cached(time = 86_400)]
pub async fn bing_search(hymn: Hymn) -> Result<BingSearchResult, Arc<reqwest::Error>> {
    let query = format!(r#""{}""#, hymn.title);
    let query = urlencoding::encode(&query);
    let url = format!("{BING_ENDPOINT}?q={query}&count=10");

    let data: BingSearchResult = reqwest::Client::new()
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

    // cache embed codes for the videos, in case we want to watch any of them
    // this saves a second request if we do try to watch one
    if let BingSearchResult::Videos(videos) = &data {
        for video in &videos.value {
            if video.allow_https_embed {
                if let Some(embed) = &video.embed_html {
                    CACHED_VIDEO_EMBED_CODES.insert(video.id.clone(), embed.clone());
                }
            }
        }
    }

    Ok(data)
}
