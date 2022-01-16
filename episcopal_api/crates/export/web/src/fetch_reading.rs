use liturgy::{BiblicalCitation, BiblicalReading, Content, DocumentError, Version};
use log::error;
use reference_parser::{BibleVerse, BibleVersePart, Book};
use sauron::{Cmd, Http};
use serde::Deserialize;

use crate::{Msg, Viewer};

#[derive(Deserialize)]
pub struct BibleReadingFromAPI {
    pub citation: String,
    pub label: String,
    pub version: Version,
    pub value: Vec<serde_json::value::Value>,
}

impl Viewer {
    pub fn fetch_biblical_reading(
        &self,
        path: Vec<usize>,
        citation: &BiblicalCitation,
    ) -> Cmd<Self, Msg> {
        let url = format!(
            "https://us-central1-venite-2.cloudfunctions.net/bible?citation={}&version=NRSV",
            citation.citation
        );
        Http::fetch_with_text_response_decoder(
            &url,
            {
                let citation = citation.clone();
                let path = path.clone();
                move |v: String| match serde_json::from_str::<BibleReadingFromAPI>(&v) {
                    Ok(data) => {
                        let reading = BiblicalReading {
                            citation: data.citation.clone(),
                            intro: citation.intro.clone(),
                            text: data
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
                                            let chapter =
                                                chapter.as_str().unwrap().parse().unwrap();
                                            let verse = verse.as_str().unwrap().parse().unwrap();

                                            Some((
                                                BibleVerse {
                                                    book,
                                                    chapter,
                                                    verse,
                                                    verse_part: BibleVersePart::All,
                                                },
                                                text,
                                            ))
                                        }
                                        _ => None,
                                    }
                                })
                                .collect(),
                        };

                        Msg::SetContent(path.clone(), Content::BiblicalReading(reading))
                    }
                    Err(error) => {
                        error!("{:#?}", error);
                        Msg::SetContent(
                            path.clone(),
                            Content::Error(DocumentError::from(format!(
                                "Error loading {}",
                                citation.citation
                            ))),
                        )
                    }
                }
            },
            {
                move |_| {
                    Msg::SetContent(
                        path.clone(),
                        Content::Error(DocumentError::from("Error loading the citation.")),
                    )
                }
            },
        )
    }
}
