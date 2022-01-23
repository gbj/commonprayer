use crate::components::biblical_reading;
use episcopal_api::{
    liturgy::{BiblicalCitation, BiblicalReading, Content, Version},
    reference_parser::{BibleVerse, BibleVersePart, Book},
};
use futures::StreamExt;
use leptos::*;
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;

use super::DocumentController;

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    Idle,
    Loading,
    Error,
    Success(Box<BiblicalReading>),
}

pub fn biblical_citation(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    citation: &BiblicalCitation,
) -> View {
    let state = Behavior::new(State::Idle);

    if !is_server!() {
        {
            let state = state.clone();
            let citation = citation.clone();
            let controller = controller.clone();
            let path = path.clone();
            spawn_local(async move {
                if state.get() == State::Idle {
                    state.set(State::Loading);
                    match fetch_reading(&citation, Version::NRSV).await {
                        Ok(reading) => {
                            state.set(State::Success(Box::new(reading.clone())));
                            if let Err(e) = controller
                                .update_content_at_path(path, Content::BiblicalReading(reading))
                            {
                                log(&format!(
                                    "(biblical_citation) error updating controller {:#?}",
                                    e
                                ))
                            };
                        }
                        Err(_) => state.set(State::Error),
                    }
                }
            });
        }
    }

    let main = state
        .stream()
        .map({
            let citation = citation.citation.clone();
            let locale = locale.to_string();
            let controller = controller.clone();
            move |state| match state {
                State::Idle => View::Empty,
                State::Loading => View::StaticText(t!("loading")),
                State::Error => view! {
                    <p class="error">{t!("biblical_citation.error", citation = &citation)}</p>
                },
                State::Success(reading) => {
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

async fn fetch_reading(
    citation: &BiblicalCitation,
    version: Version,
) -> Result<BiblicalReading, ()> {
    let url = reading_url(&citation.citation, version);

    let reading = Request::get(&url)
        .send()
        .await
        .map_err(|_| ())?
        .json::<BibleReadingFromAPI>()
        .await
        .map_err(|_| ())?
        .api_data_to_biblical_reading(citation);
    Ok(reading)
}

#[derive(Deserialize)]
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
