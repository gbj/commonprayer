use app::api::bing::BingSearchResult;
use episcopal_api::hymnal::Hymn;
use lazy_static::lazy_static;
use reqwest::header::{HeaderName, HeaderValue};
use std::str::FromStr;

lazy_static! {
    pub static ref BING_SEARCH_API_KEY: String =
        std::env::var("BING_SEARCH_API_KEY").expect("BING_SEARCH_API_KEY not set");
}

const BING_ENDPOINT: &str = "https://api.bing.microsoft.com/v7.0/videos/search";

pub async fn search(hymn: &Hymn) -> Result<BingSearchResult, reqwest::Error> {
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
        .await?
        .json()
        .await?;

    Ok(data)
}
