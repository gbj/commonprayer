use reference_parser::{BibleReference, BibleVerse, BibleVersePart, Book};
use serde::{Deserialize, Serialize};

use crate::Reference;

/// Represents an entire psalm
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Psalm {
    /// The psalm number (e.g., 8 for Psalm 8)
    pub number: u8,
    /// Present when only a subset of verses should be displayed
    pub citation: Option<String>,
    /// The content of the psalm, by section
    pub sections: Vec<PsalmSection>,
}

impl Psalm {
    /// Returns only the verses and sections of a psalm that are included in its citation.
    /// ```
    /// # use psalter::bcp1979::{PSALM_1, PSALM_119};
    /// # use reference_parser::BibleReference;
    /// // simple filtering within a single-section psalm
    /// let mut psalm_1 = PSALM_1.clone();
    /// psalm_1.citation = Some(String::from("Psalm 1:1-4"));
    /// assert_eq!(psalm_1.filtered_sections().len(), 1);
    /// assert_eq!(psalm_1.filtered_sections()[0].verses.len(), 4);
    ///
    /// // filtering across multiple sections of a single psalm with a single citation
    /// let mut psalm_119 = PSALM_119.clone();
    /// psalm_119.citation = Some(String::from("Psalm 119:145-176"));
    /// assert_eq!(psalm_119.filtered_sections().len(), 4);
    /// assert_eq!(psalm_119.filtered_sections()[0].verses.len(), 8);
    /// assert_eq!(psalm_119.filtered_sections()[0].local_name, "Qoph");
    /// assert_eq!(psalm_119.filtered_sections()[0].verses[0].a, "I call with my whole heart; *");
    /// ```
    pub fn filtered_sections(&self) -> Vec<PsalmSection> {
        let citation = self.citation.as_ref().map(BibleReference::from);
        if let Some(citation) = citation {
            self.sections
                .iter()
                .map(|section| PsalmSection {
                    reference: section.reference,
                    local_name: section.local_name.clone(),
                    latin_name: section.latin_name.clone(),
                    verses: section
                        .verses
                        .iter()
                        .filter_map(|verse| {
                            let contains_a = citation.contains(BibleVerse {
                                book: Book::Psalms,
                                chapter: self.number as u16,
                                verse: verse.number as u16,
                                verse_part: BibleVersePart::A,
                            });
                            let contains_b = citation.contains(BibleVerse {
                                book: Book::Psalms,
                                chapter: self.number as u16,
                                verse: verse.number as u16,
                                verse_part: BibleVersePart::B,
                            });

                            match (contains_a, contains_b) {
                                (true, true) => Some(PsalmVerse {
                                    number: verse.number,
                                    a: verse.a.clone(),
                                    b: verse.b.clone(),
                                }),
                                (true, false) => Some(PsalmVerse {
                                    number: verse.number,
                                    a: verse.a.clone(),
                                    b: "".into(),
                                }),
                                (false, true) => Some(PsalmVerse {
                                    number: verse.number,
                                    a: "".into(),
                                    b: verse.b.clone(),
                                }),
                                (false, false) => None,
                            }
                        })
                        .collect(),
                })
                .filter(|section| !section.verses.is_empty())
                .collect()
        } else {
            self.sections.clone()
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmSection {
    /// Reference to e.g., a BCP page
    pub reference: Reference,
    /// Name for the section in the psalm's own language (e.g., "Part I" or "Aleph")
    pub local_name: String,
    /// Latin name for the section (e.g., "Beatus vir qui non abiit")
    pub latin_name: String,
    /// The set of verses included in this section
    pub verses: Vec<PsalmVerse>,
}

impl PsalmSection {
    /// Verse number of the first verse in this section
    pub fn first_verse(&self) -> u8 {
        self.verses.first().map_or(0, |verse| verse.number)
    }

    /// Verse number of the last verse in this section
    pub fn last_verse(&self) -> u8 {
        self.verses.last().map_or(0, |verse| verse.number)
    }

    /// Tests whether this section includes the given verse number
    pub fn includes_verse(&self, verse_number: u8) -> bool {
        self.verses.iter().any(|verse| verse.number == verse_number)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmVerse {
    /// Verse number
    pub number: u8,
    /// Text of the first half of the verse, up to the asterisk
    pub a: String,
    /// Text of the second half of the verse, after the asterisk
    pub b: String,
}
