use reference_parser::BibleVerse;
use serde::{Deserialize, Serialize};

use crate::Document;

/// A reading that contains the text of a portion of the Bible.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalReading {
    /// A citation for the book/chapters/verses included.
    pub citation: String,
    /// The text
    pub text: BiblicalReadingText,
    /// Introduction to the reading. The introduction begins as a [BiblicalReadingIntroTemplate](crate::BiblicalReadingIntroTemplate).
    pub intro: Option<BiblicalReadingIntro>,
}

pub type BiblicalReadingText = Vec<(BibleVerse, String)>;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalReadingIntro(Box<Document>);

impl BiblicalReadingIntro {
    pub fn as_document(&self) -> &Document {
        &self.0
    }
}

impl From<Document> for BiblicalReadingIntro {
    fn from(document: Document) -> Self {
        Self(Box::new(document))
    }
}

impl<T> From<T> for BiblicalReadingIntro
where
    T: std::fmt::Display,
{
    fn from(t: T) -> Self {
        Self(Box::new(Document::from(t.to_string())))
    }
}

impl From<BiblicalReadingIntro> for Document {
    fn from(intro: BiblicalReadingIntro) -> Self {
        *intro.0
    }
}
