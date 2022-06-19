use std::pin::Pin;

use futures::{future::join_all, Future};
use leptos2::*;
use liturgy::{BiblicalCitation, BiblicalReading, BiblicalReadingIntro, Version};
use reference_parser::{BibleVerse, BibleVersePart, Book};
use serde::{Deserialize, Serialize};

use reqwest::Client;
//use reqwest_middleware::ClientBuilder;
//use reqwest_middleware_cache::{managers::CACacheManager, Cache, CacheMode};

use crate::utils::{
    encode_uri,
    fetch::{fetch, FetchError},
};

pub type ReadingFuture =
    Pin<Box<dyn Future<Output = Result<BiblicalReading, FetchError>> + Send + Sync>>;

lazy_static::lazy_static! {
    static ref CLIENT: Client = Client::new();
}
pub async fn load_reading(
    citation: String,
    version: Version,
    intro: Option<BiblicalReadingIntro>,
) -> Result<BiblicalReading, FetchError> {
    let url = reading_url(&citation, version);

    CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            eprintln!("\n\n(load_reading request) error \n{:#?}", e);
            if e.is_connect() {
                FetchError::Connection
            } else {
                FetchError::Server
            }
        })?
        .json::<BibleReadingFromAPI>()
        .await
        .map_err(|e| {
            eprintln!("\n\n(load_reading JSON) error \n{:#?}", e);
            FetchError::Json
        })
        .map(|reading| reading.api_data_to_biblical_reading(&citation, &intro))
}

#[derive(Deserialize, Clone, Default, Debug, PartialEq, Serialize)]
pub struct BibleReadingFromAPI {
    pub citation: String,
    pub label: String,
    pub version: Version,
    pub value: Vec<serde_json::value::Value>,
}

#[derive(Debug)]
enum ReadingContent {
    Verse(BibleVerse, String),
    ParagraphBreak,
}

impl BibleReadingFromAPI {
    pub fn api_data_to_biblical_reading(
        &self,
        citation: &str,
        intro: &Option<BiblicalReadingIntro>,
    ) -> BiblicalReading {
        let mut text = Vec::new();
        let parts = self
            .value
            .iter()
            .filter_map(|line| {
                let book = line.get("book");
                let chapter = line.get("chapter");
                let verse = line.get("verse");
                let text = line.get("text");
                let ldf_type = line.get("type");
                match (ldf_type, book, chapter, verse, text) {
                    (Some(value), ..) => {
                        if value == &serde_json::Value::String("heading".to_string()) {
                            Some(ReadingContent::ParagraphBreak)
                        } else {
                            None
                        }
                    }
                    (_, Some(book), Some(chapter), Some(verse), Some(text)) => {
                        let text = text.as_str().unwrap_or_default().to_string();
                        let book = Book::from(book.as_str().unwrap_or_default());
                        let chapter = chapter
                            .as_str()
                            .unwrap_or_default()
                            .parse()
                            .unwrap_or_default();
                        let verse = verse
                            .as_str()
                            .unwrap_or_default()
                            .parse()
                            .unwrap_or_default();

                        Some(ReadingContent::Verse(
                            BibleVerse {
                                book,
                                chapter,
                                verse,
                                verse_part: BibleVersePart::All,
                            },
                            strip_entities(text),
                        ))
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        for (idx, piece) in parts.iter().enumerate() {
            let peeked = parts.get(idx + 1);
            let this_verse = match (piece, peeked) {
                (ReadingContent::Verse(verse, text), None) => {
                    Some((verse.to_owned(), text.to_owned()))
                }
                (ReadingContent::Verse(verse, text), Some(ReadingContent::Verse(..))) => {
                    Some((verse.to_owned(), text.to_owned()))
                }
                (ReadingContent::Verse(verse, text), Some(ReadingContent::ParagraphBreak)) => {
                    Some((verse.to_owned(), text.to_owned() + "\n\n"))
                }
                (ReadingContent::ParagraphBreak, _) => None,
            };
            if let Some(data) = this_verse {
                text.push(data);
            }
        }

        BiblicalReading {
            citation: citation.to_string(),
            intro: intro.clone(),
            text,
        }
    }
}

pub fn reading_url(citation: &str, version: Version) -> String {
    format!(
        "https://us-central1-venite-2.cloudfunctions.net/bible?citation={}&version={}",
        encode_uri(citation),
        version
    )
}

#[cfg(target_arch = "wasm32")]
fn strip_entities(text: String) -> String {
    // textarea hack — avoids additional WASM size/Rust code at the expense of slow JS interop
    match document().create_element("textarea") {
        Ok(textarea) => {
            textarea.set_inner_html(&text);
            textarea.text_content().unwrap_or(text)
        }
        Err(_) => text,
    }
}

fn strip_entities(text: String) -> String {
    htmlentity::entity::decode(&text).iter().collect()
}
