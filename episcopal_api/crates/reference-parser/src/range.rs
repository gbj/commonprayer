use crate::{BibleReferenceQuery, BibleVerse, BibleVersePart};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct BibleReferenceRange {
    pub start: BibleReferenceQuery,
    pub end: Option<BibleReferenceQuery>,
    pub bracketed: bool,
}

impl BibleReferenceRange {
    pub(crate) fn contains(&self, verse: BibleVerse) -> bool {
        if let Some(start_book) = self.start.book {
            let start_chapter = self.start.chapter.unwrap_or(1);
            let start_verse = self.start.verse.unwrap_or(1);
            let start_verse_part = self.start.verse_part;

            let end_book = self.end.and_then(|end| end.book).unwrap_or(start_book);
            let end_chapter = self
                .end
                .and_then(|end| end.chapter)
                .unwrap_or(start_chapter);
            let end_verse = self.end.and_then(|end| end.verse);
            let end_verse_part = self.end.map(|end| end.verse_part);

            verse.book >= start_book
                && verse.book <= end_book
                && verse.chapter >= start_chapter
                && verse.chapter <= end_chapter
                && verse.verse >= start_verse
                && (end_verse.is_none() || verse.verse <= end_verse.unwrap())
                && (verse.verse_part == BibleVersePart::All
                    || (start_verse_part == BibleVersePart::All
                        && (end_verse_part.is_none()
                            || end_verse_part.unwrap() == BibleVersePart::All))
                    || (verse.verse_part >= start_verse_part
                        && (end_verse_part.is_none()
                            || verse.verse_part <= end_verse_part.unwrap())))
        } else {
            false
        }
    }
}
