use crate::{
    components::biblical_reading,
    preferences,
    utils::fetch::{Fetch, FetchStatus},
};
use episcopal_api::{
    liturgy::{
        BiblicalCitation, BiblicalReading, GlobalPref, PreferenceKey, PreferenceValue, Version,
    },
    reference_parser::{BibleVerse, BibleVersePart, Book},
};
use futures::StreamExt;
use leptos::*;
use serde::Deserialize;
use wasm_bindgen::UnwrapThrowExt;

use super::DocumentController;

pub fn biblical_citation(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    citation: &BiblicalCitation,
    version: Version,
) -> View {
    let version = if version.is_bible_translation() {
        version
    } else {
        preferences::get(&PreferenceKey::from(GlobalPref::BibleVersion))
            .and_then(|value| match value {
                PreferenceValue::Version(v) => Some(v),
                _ => None,
            })
            .unwrap_or(Version::NRSV)
    };
    let fetch = Fetch::<BibleReadingFromAPI>::new(reading_url(&citation.citation, version));

    fetch.send();

    let main = fetch
        .state
        .stream()
        .map({
            let locale = locale.to_string();
            let controller = controller.clone();
            let citation = citation.clone();
            move |state| match state {
                FetchStatus::Idle => View::Empty,
                FetchStatus::Loading => View::StaticText(t!("loading")),
                FetchStatus::Error(_) => view! {
                    <p class="error">{t!("biblical_citation.error", citation = &citation.citation)}</p>
                },
                FetchStatus::Success(reading) => {
                    let reading = reading.api_data_to_biblical_reading(&citation);
                    biblical_reading(&locale, &controller, path.clone(), &reading).1
                }
            }
        })
        .boxed_local();

    view! {
        <article class="biblical-citation">
            <header>
                <h4 class="citation">{citation.to_string()}</h4>
            </header>
             <main>
                {main}
            </main>
        </article>
    }
}

#[derive(Deserialize, Clone)]
pub struct BibleReadingFromAPI {
    pub citation: String,
    pub label: String,
    pub version: Version,
    pub value: Vec<serde_json::value::Value>,
}

impl BibleReadingFromAPI {
    fn api_data_to_biblical_reading(&self, citation: &BiblicalCitation) -> BiblicalReading {
        BiblicalReading {
            citation: self.citation.clone(),
            intro: citation.intro.clone(),
            text: self
                .value
                .iter()
                .filter_map(|line| {
                    let book = line.get("book");
                    let chapter = line.get("chapter");
                    let verse = line.get("verse");
                    let text = line.get("text");
                    let ldf_type = line.get("type");
                    match (ldf_type, book, chapter, verse, text) {
                        (Some(_), _, _, _, _) => None,
                        (_, Some(book), Some(chapter), Some(verse), Some(text)) => {
                            let text = text.as_str().unwrap().to_string();
                            let book = Book::from(book.as_str().unwrap());
                            let chapter = chapter.as_str().unwrap().parse().unwrap();
                            let verse = verse.as_str().unwrap().parse().unwrap();

                            Some((
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
                .collect(),
        }
    }
}

fn reading_url(citation: &str, version: Version) -> String {
    format!(
        "https://us-central1-venite-2.cloudfunctions.net/bible?citation={}&version={}",
        citation, version
    )
}

fn strip_entities(text: String) -> String {
    // textarea hack — avoids additiheonal WASM size/Rust code at the expense of slow JS interop
    let textarea = document().create_element("textarea").unwrap_throw();
    textarea.set_inner_html(&text);
    textarea.text_content().unwrap_or(text)
}
