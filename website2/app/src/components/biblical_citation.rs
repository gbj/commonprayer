use leptos2::*;
use liturgy::{BiblicalCitation, BiblicalReading, Version};
use reference_parser::{BibleVerse, BibleVersePart, Book};

use crate::{
    utils::fetch::{fetch, Fetch, FetchError, FetchMsg, FetchStatus},
    views::document::biblical_reading_verses,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct BiblicalCitationLoader {
    pub locale: String,
    #[prop]
    pub citation: BiblicalCitation,
    pub version: Version,
    #[prop]
    pub path: Vec<usize>,
    state: FetchStatus<BiblicalReading>,
}

#[derive(Clone, Debug)]
pub enum BiblicalCitationMsg {
    LoadReading,
    FetchError(FetchError),
    FetchResult(BiblicalReading),
}

#[derive(Debug)]
pub enum BiblicalCitationCmd {
    Init,
    LoadReading(BiblicalCitation, Version),
}

#[async_trait(?Send)]
impl State for BiblicalCitationLoader {
    type Msg = BiblicalCitationMsg;
    type Cmd = BiblicalCitationCmd;

    fn init(&self) -> Option<Self::Cmd> {
        Some(Self::Cmd::Init)
    }

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        match msg {
            BiblicalCitationMsg::LoadReading => {
                self.state = FetchStatus::Loading;
                return Some(Self::Cmd::LoadReading(self.citation.clone(), self.version));
            }
            BiblicalCitationMsg::FetchError(e) => self.state = FetchStatus::Error(e),
            BiblicalCitationMsg::FetchResult(r) => self.state = FetchStatus::Success(Box::new(r)),
        };
        None
    }

    async fn cmd(cmd: Self::Cmd, _host: web_sys::HtmlElement) -> Option<Self::Msg> {
        match cmd {
            BiblicalCitationCmd::Init => Some(Self::Msg::LoadReading),
            BiblicalCitationCmd::LoadReading(citation, version) => {
                let url = reading_url(&citation.citation, version);
                match fetch::<BibleReadingFromAPI>(&url, None).await {
                    Ok(res) => Some(BiblicalCitationMsg::FetchResult(
                        res.api_data_to_biblical_reading(&citation),
                    )),
                    Err(e) => Some(BiblicalCitationMsg::FetchError(e)),
                }
            }
        }
    }
}

impl Component for BiblicalCitationLoader {
    fn view(&self) -> Host {
        let content = match &self.state {
            FetchStatus::Idle => view! { <p class="loading">{t!("loading")}</p> },
            FetchStatus::Loading => view! { <p class="loading">{t!("loading")}</p> },
            FetchStatus::Error(_) => {
                view! { <p class="error">{t!("biblical_citation.error", citation = &self.citation.citation)}</p> }
            }
            FetchStatus::Success(reading) => {
                let verses = biblical_reading_verses(reading);
                view! {
                    <div>{verses}</div>
                }
            }
        };

        view! {
            <Host>
                {content}
            </Host>
        }
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
    fn api_data_to_biblical_reading(&self, citation: &BiblicalCitation) -> BiblicalReading {
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
                            log(&format!("non-verse, non-heading entry = {:#?}", line));
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
            citation: self.citation.clone(),
            intro: citation.intro.clone(),
            text,
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
    // textarea hack — avoids additional WASM size/Rust code at the expense of slow JS interop
    match document().create_element("textarea") {
        Ok(textarea) => {
            textarea.set_inner_html(&text);
            textarea.text_content().unwrap_or(text)
        }
        Err(_) => text,
    }
}
