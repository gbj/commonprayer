use std::collections::HashSet;

use liturgy::{BiblicalReading, Content, Document, Version};
use reference_parser::{BibleReference, BibleVerse, BibleVersePart, Book};

pub mod rv09;
pub use rv09::*;
mod usx_book_codes;
pub use usx_book_codes::usx_book_code;

use thiserror::Error;
use usx_book_codes::usx_code_to_book;

#[derive(Error, Debug)]
pub enum UsxError {
    #[error("book not found in this translation")]
    BookNotFound(Book),
    #[error("citations that span multiple books are not supported")]
    CrossBookCitation,
    #[error("error parsing USX file")]
    XmlParsing(roxmltree::Error),
}

pub trait OfflineBible {
    fn load_book(book: Book) -> Result<String, UsxError>;

    fn version() -> Version;

    /// The main function you will call convert a Biblical citation into a liturgical document
    fn get_citation(citation: &str) -> Result<Document, UsxError> {
        let reference = BibleReference::from(citation);

        let books = reference
            .ranges
            .iter()
            .flat_map(|range| {
                let start = range.start.book;
                let end = range.end.and_then(|end| end.book);
                [start, end].into_iter()
            })
            .flatten()
            .collect::<HashSet<_>>();

        if books.len() > 1 {
            return Err(UsxError::CrossBookCitation);
        }

        let book = reference.ranges.get(0).and_then(|range| range.start.book);
        match book {
            None => Ok(Document::from(Content::Empty)),
            Some(book) => {
                let text = Self::load_book(book)?;
                let xml = roxmltree::Document::parse(&text).map_err(UsxError::XmlParsing)?;
                let document = UsxDocument(xml);

                Ok(document.to_document(Self::version(), citation, &reference))
            }
        }
    }
}

pub struct UsxDocument<'a>(roxmltree::Document<'a>);

impl<'a> UsxDocument<'a> {
    pub fn to_document(
        &self,
        version: Version,
        citation: &str,
        reference: &BibleReference,
    ) -> Document {
        let mut text: Vec<(BibleVerse, String)> = Vec::new();

        for node in self
            .0
            .descendants()
            .filter(|para| para.tag_name().name() == "verse")
            .filter(|para| reference_contains_vid(reference, para.attribute("sid")))
        {
            let vid = node.attribute("sid").unwrap();
            let verse = bible_verse_from_vid(vid);
            let mut verse_text = String::new();
            for sibling in node.next_siblings() {
                if sibling.tag_name().name() == "verse" && sibling.attribute("eid") == Some(vid) {
                    break;
                } else if let Some(text_node) = sibling.text() {
                    verse_text.push_str(text_node);
                }
            }
            verse_text.push(' ');
            text.push((verse, verse_text));
        }

        // build return object
        let content = BiblicalReading {
            citation: citation.to_string(),
            text,
            intro: None,
        };

        Document::from(content).version(version)
    }
}

fn verse_id(book: Book, chapter: u16, verse: u16) -> String {
    format!("{} {}:{}", usx_book_code(book), chapter, verse)
}

fn reference_contains_vid(reference: &BibleReference, vid: Option<&str>) -> bool {
    vid.map(|vid| reference.contains(bible_verse_from_vid(vid)))
        .unwrap_or(false)
}

fn bible_verse_from_vid(vid: &str) -> BibleVerse {
    let mut book_name_parts = vid.split(' ');
    let book = book_name_parts.next().unwrap();
    let book = usx_code_to_book(book);
    let cv = book_name_parts.next().unwrap();
    let mut cv_parts = cv.split(':');
    let chapter = cv_parts.next().unwrap().parse::<u16>().unwrap();
    let verse = cv_parts.next().unwrap().parse::<u16>().unwrap();
    BibleVerse {
        book,
        chapter,
        verse,
        verse_part: BibleVersePart::All,
    }
}
