mod book_abbrevs;
mod books;
mod query;
mod range;
mod utils;

pub use books::Book;
pub use query::*;
pub use range::BibleReferenceRange;
pub use utils::parse_reference;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BibleReference {
    pub ranges: Vec<BibleReferenceRange>,
}

impl From<&str> for BibleReference {
    fn from(val: &str) -> Self {
        BibleReference {
            ranges: parse_reference(val),
        }
    }
}

impl From<String> for BibleReference {
    fn from(val: String) -> Self {
        BibleReference {
            ranges: parse_reference(&val),
        }
    }
}

impl From<&String> for BibleReference {
    fn from(val: &String) -> Self {
        BibleReference {
            ranges: parse_reference(val),
        }
    }
}

impl BibleReference {
    /// ```
    /// # use crate::reference_parser::{BibleReference, BibleVerse, Book, BibleVersePart};
    /// let verse = BibleVerse { book: Book::Matthew, chapter: 1, verse: 4, verse_part: BibleVersePart::All };
    /// let reference = BibleReference::from("Matt. 1:1-10");
    /// assert_eq!(reference.contains(verse), true);
    /// let reference = BibleReference::from("Matt. 1:1-3");
    /// assert_eq!(reference.contains(verse), false);
    /// let verse = BibleVerse { book: Book::Matthew, chapter: 1, verse: 4, verse_part: BibleVersePart::B };
    /// let reference = BibleReference::from("Matt. 1:1-4b");
    /// assert_eq!(reference.contains(verse), true);
    /// let reference = BibleReference::from("Matt. 1:1-4c");
    /// assert_eq!(reference.contains(verse), true);
    /// let reference = BibleReference::from("Matt. 1:1-4a");
    /// assert_eq!(reference.contains(verse), false);
    /// let reference = BibleReference::from("Matt. 1");
    /// assert_eq!(reference.contains(verse), true);
    /// let reference = BibleReference::from("Psalms 120, 121, 122");
    /// let verse = BibleVerse { book: Book::Psalms, chapter: 122, verse: 3, verse_part: BibleVersePart::All };
    /// assert_eq!(reference.contains(verse), true);
    /// ```
    /// Tests whether the given [BibleVerse] is included within this reference.
    pub fn contains(&self, verse: BibleVerse) -> bool {
        self.ranges.iter().any(|range| range.contains(verse))
    }
}
