use crate::Book;
use serde::{Deserialize, Serialize};

pub type Chapter = u16;
pub type Verse = u16;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
/// A possibly-incomplete reference to a Bible book, chapter, or verse
/// (i.e., It can be as specific as "John 1:1a" or as vague as "John")
pub struct BibleReferenceQuery {
    pub book: Option<Book>,
    pub chapter: Option<Chapter>,
    pub verse: Option<Verse>,
    pub verse_part: BibleVersePart,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
/// A complete reference to a specific Bible verse
pub struct BibleVerse {
    pub book: Book,
    pub chapter: Chapter,
    pub verse: Verse,
    pub verse_part: BibleVersePart,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Serialize, Deserialize)]
pub enum BibleVersePart {
    All,
    A,
    B,
    C,
    D,
}
