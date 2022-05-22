use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "_type")]
pub enum BingSearchResult {
    Videos(Videos),
    ErrorResponse(ErrorResponse),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Videos {
    #[serde(alias = "webSearchUrl")]
    pub web_search_url: String,
    pub value: Vec<VideoResult>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<BingError>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BingError {
    pub code: String,
    #[serde(alias = "subCode")]
    pub subcode: String,
    pub message: String,
    #[serde(alias = "moreDetails")]
    pub more_details: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct VideoResult {
    pub duration: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    #[serde(alias = "publisher")]
    pub publisher: Option<Vec<Named>>,
    #[serde(alias = "thumbnailUrl")]
    pub creator: Option<Named>,
    #[serde(alias = "datePublished")]
    pub date_published: Option<String>,
    #[serde(alias = "isAccessibleForFree", default = "default_bool_field")]
    pub free: bool,
    #[serde(alias = "contentUrl")]
    pub content_url: Option<String>,
    #[serde(alias = "embedHtml")]
    pub embed_html: Option<String>,
    #[serde(alias = "allowHttpsEmbed", default = "default_bool_field")]
    pub allow_https_embed: bool,
    #[serde(alias = "allowMobileEmbed", default = "default_bool_field")]
    pub allow_mobile_embed: bool,
    #[serde(alias = "viewCount")]
    pub view_count: Option<u64>,
}

fn default_bool_field() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Named {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Thumbnail {
    pub width: u32,
    pub height: u32,
}
