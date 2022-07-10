use std::pin::Pin;

use futures::Future;
use leptos2::*;
use liturgy::{BiblicalReading, BiblicalReadingIntro, Document, Version};
use reference_parser::{BibleVerse, BibleVersePart, Book};
use serde::{Deserialize, Serialize};

use reqwest::Client;

use crate::routes::document::views::biblical_reading;
use crate::utils::{encode_uri, fetch::FetchError};

struct Cache<K, V>(moka::sync::Cache<K, V>)
where
    K: std::hash::Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static;

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Send + Sync,
    V: Clone + Send + Sync,
{
    fn new() -> Self {
        Self(
            moka::sync::Cache::builder()
                .time_to_idle(std::time::Duration::from_secs(604_800)) // after one week without get() or insert(), will expire
                .max_capacity(50) // if we have more than 50 cached readings, start dropping them based on use
                .build(),
        )
    }

    fn get(&self, key: &K) -> Option<V> {
        self.0.get(key)
    }

    fn insert(&self, key: K, value: V) {
        self.0.insert(key, value);
    }
}

lazy_static::lazy_static! {
    static ref CLIENT: Client = Client::new();

    // the assumption is that we'll typically want to cache a few days of Daily Office readings,
    // and maybe a few Sundays of RCL readings, but not much more
    static ref READINGS_CACHE: Cache<(String, Version), BiblicalReading> = Cache::new();
}

pub type ReadingFuture =
    Pin<Box<dyn Future<Output = Result<BiblicalReading, FetchError>> + Send + Sync>>;

pub enum ReadingLoader {
    Sync(BiblicalReading),
    Async {
        citation: String,
        reading: ReadingFuture,
    },
}

impl ReadingLoader {
    pub fn into_future(self) -> Pin<Box<dyn Future<Output = Result<BiblicalReading, FetchError>>>> {
        match self {
            ReadingLoader::Sync(reading) => Box::pin(async { Ok(reading) })
                as Pin<Box<dyn Future<Output = Result<BiblicalReading, FetchError>>>>,
            ReadingLoader::Async { reading, .. } => Box::pin(async { reading.await }),
        }
    }
}

impl std::fmt::Debug for ReadingLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync(arg0) => f.debug_tuple("Sync").field(arg0).finish(),
            Self::Async { citation, .. } => {
                f.debug_struct("Async").field("citation", citation).finish()
            }
        }
    }
}

impl ReadingLoader {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(citation: &str, version: Version, intro: Option<BiblicalReadingIntro>) -> Self {
        let citation = citation.to_string();
        match READINGS_CACHE.get(&(citation.clone(), version)) {
            Some(reading) => ReadingLoader::Sync(reading),
            None => {
                let url = reading_url(&citation, version);

                let reading = Box::pin({
                    let citation = citation.clone();
                    async move {
                        let reading = CLIENT
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
                            .map(|reading| reading.api_data_to_biblical_reading(&citation, &intro));
                        if let Ok(reading) = &reading {
                            READINGS_CACHE.insert((citation, version), reading.clone());
                        }
                        reading
                    }
                })
                    as Pin<
                        Box<dyn Future<Output = Result<BiblicalReading, FetchError>> + Send + Sync>,
                    >;

                ReadingLoader::Async { citation, reading }
            }
        }
    }

    // TODO ReadingLoader on WASM — Reqwest WASM issues
    // a) Futures not Send/Sync
    // b) Error::is_connect doesn't exist
    #[cfg(target_arch = "wasm32")]
    pub fn new(citation: &str, version: Version, intro: Option<BiblicalReadingIntro>) -> Self {
        let reading = Box::pin({
            let citation = citation.clone();
            async move { Err(FetchError::Connection) }
        })
            as Pin<Box<dyn Future<Output = Result<BiblicalReading, FetchError>> + Send + Sync>>;

        ReadingLoader::Async {
            citation: citation.to_string(),
            reading,
        }
    }

    pub fn as_citation(&self) -> &str {
        match self {
            ReadingLoader::Sync(reading) => reading.citation.as_str(),
            ReadingLoader::Async { citation, .. } => citation.as_str(),
        }
    }

    pub fn view(self, locale: &str, path: Vec<usize>) -> Vec<Node> {
        self.view_config(locale, path, true)
    }

    pub fn view_without_header(self, locale: &str, path: Vec<usize>) -> Vec<Node> {
        self.view_config(locale, path, false)
    }

    fn view_config(self, locale: &str, path: Vec<usize>, with_header: bool) -> Vec<Node> {
        match self {
            ReadingLoader::Sync(reading) => {
                let (header, main) = biblical_reading(locale, path.clone(), &reading);
                view! {
                    <>
                        <a id={&reading.citation}></a>
                        <article class="document">
                            <header>{header}</header>
                            <main>{main}</main>
                        </article>
                        {reading_loaded_script(&path, &reading)}
                    </>
                }
            }
            ReadingLoader::Async { citation, reading } => {
                let locale = locale.to_string();
                let reading = Node::AsyncElement(AsyncElement {
                    pending: Box::new(view! { <p>{t!("loading")}</p> }),
                    ready: Some(Box::pin({
                        let citation = citation.clone();
                        async move {
                            match reading.await {
                                Ok(reading) => view! {
                                    <div>
                                        {biblical_reading(&locale, path.clone(), &reading).1}
                                        {reading_loaded_script(&path, &reading)}
                                    </div>
                                },
                                Err(_) => view! {
                                    <p class="error">{t!("biblical_citation.error", citation = &citation)}</p>
                                },
                            }
                        }
                    })),
                });

                if with_header {
                    view! {
                        <>
                            <a id={&citation}></a>
                            <article class="document">
                                <header><h3 class="citation">{&citation}</h3></header>
                                <main>{reading}</main>
                            </article>
                        </>
                    }
                } else {
                    vec![reading]
                }
            }
        }
    }
}

fn reading_loaded_script(path: &[usize], reading: &BiblicalReading) -> Node {
    let data =
        serde_json::to_string(&(path, reading)).expect("could not serialize loaded reading data");
    view! {
        <script>
        {format!(r#"customElements.whenDefined("l-export-links").then(() => window.dispatchEvent(new CustomEvent("readingloaded", {{ detail: JSON.parse({data:?}) }})));"#)}
        </script>
    }
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

#[cfg(not(target_arch = "wasm32"))]
fn strip_entities(text: String) -> String {
    // HTML handles certain entities in a way that's not actually Unicode,
    // so we need to fix them manually — we can't actually just use a normal entity replacement
    // see https://stackoverflow.com/questions/7031633/146-is-getting-converted-as-u0092-by-nokogiri-in-ruby-on-rails
    text.replace("&#141;", "‘")
        .replace("&#142;", "’")
        .replace("&#143;", "“")
        .replace("&#144;", "”")
        .replace("&#146;", "’")
        .replace("&#147;", "“")
        .replace("&#148;", "”")
        .replace("&#149;", "‘")
        .replace("&#150;", "’")
        .replace("&#151;", "“")
        .replace("&#152;", "”")
        .replace("&#153;", "–")
        .replace("&#154;", "—")
}
