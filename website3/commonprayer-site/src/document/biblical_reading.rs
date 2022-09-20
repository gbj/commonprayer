use crate::{
    document::{Document, DocumentProps, SmallCaps, SmallCapsProps},
    fetch::fetch,
    i18n::use_i18n,
    i18n_args,
    settings::use_settings,
};
use leptos::*;

#[component]
pub fn BiblicalReading(cx: Scope, reading: liturgy::BiblicalReading) -> Element {
    let intro = reading.intro.as_ref().map(|intro| {
        view! {
            <section class="reading-intro">
                <Document doc=intro.as_document().clone()/>
            </section>
        }
    });

    let verses = reading
        .text
        .into_iter()
        .flat_map(|(number, line)| {
            view! {
                <>
                    <sup class="BiblicalReading-verse-number">{number.verse.to_string()} " "</sup>
                    <span><SmallCaps line/></span>
                </>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <article class="document BiblicalReading">
            <a id=&reading.citation></a>
            <header>
                <h3 class="BiblicalReading-citation">{&reading.citation}</h3>
            </header>
            <main>
                {intro}
                {verses}
            </main>
        </article>
    }
}

#[component]
pub fn BiblicalCitation(
    cx: Scope,
    citation: String,
    intro: Option<liturgy::BiblicalReadingIntro>,
) -> Element {
    let (t, t_with_args, _) = use_i18n(cx);
    let (settings, _) = use_settings(cx);
    let bible_version = move || settings.with(|s| s.bible_version);

    // TODO if on server, cache this
    let reading = create_resource(
        cx,
        {
            let citation = citation.clone();
            move || (citation.to_string(), bible_version())
        },
        {
            move |(citation, version): (String, Version)| async move {
                fetch::<BibleReadingFromAPI>(&reading_url(&citation, version))
                    .await
                    .map_err(|_| ())
            }
        },
    );

    view! {
        <div>
            <Suspense fallback=move || view! { <p class="loading">{t("loading")}</p> }>
                {
                    let citation = citation.clone();
                    let intro = intro.clone();
                    move || {
                        reading.read().map({
                            let citation = citation.clone();
                            let intro = intro.clone();
                            move |res| match res {
                                Err(_) => view! { <p class="error">{t_with_args("biblical-citation-error", &i18n_args!("citation" => citation))}</p> },
                                Ok(reading) => {
                                    let reading = reading.to_biblical_reading(&citation, &intro);
                                    view! { <BiblicalReading reading/> }
                                }
                            }
                        })
                    }
                }
            </Suspense>
        </div>
    }
}

use liturgy::Version;
use reference_parser::{BibleVerse, BibleVersePart, Book};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
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
    pub fn to_biblical_reading(
        &self,
        citation: &str,
        intro: &Option<liturgy::BiblicalReadingIntro>,
    ) -> liturgy::BiblicalReading {
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

        liturgy::BiblicalReading {
            citation: citation.to_string(),
            intro: intro.clone(),
            text,
        }
    }
}

pub fn reading_url(citation: &str, version: Version) -> String {
    format!(
        "https://us-central1-venite-2.cloudfunctions.net/bible?citation={}&version={}",
        escape(citation),
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
        .replace("&#151;", "—")
        .replace("&#152;", "”")
        .replace("&#153;", "–")
        .replace("&#154;", "—")
}
