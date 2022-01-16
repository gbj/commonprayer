#[macro_use]
extern crate lazy_static;
pub mod bcp1979;

use itertools::Itertools;
use liturgy::Psalm;
use reference_parser::BibleReference;
use std::{convert::TryInto, iter::once};

/// Defines a version or translation of the psalms, with a single entry per psalm
pub struct Psalter {
    psalms: Vec<(u8, &'static Psalm)>,
}

impl Psalter {
    /// Returns a single psalm, if it exists, by its number.
    /// ```
    /// # use crate::psalter::bcp1979::BCP1979_PSALTER;
    /// assert_eq!(BCP1979_PSALTER.psalm_by_number(1).map(|psalm| psalm.number), Some(1));
    /// ```
    pub fn psalm_by_number(&self, number: u8) -> Option<&Psalm> {
        self.psalms
            .iter()
            .find(|(s_number, _)| *s_number == number)
            .map(|(_, psalm)| *psalm)
    }

    /// Returns the set of psalms covered by a given citation, including filtering verses.
    /// ```
    /// # use crate::psalter::bcp1979::BCP1979_PSALTER;
    /// let three_psalms = BCP1979_PSALTER.psalms_by_citation("Psalms 120, 121, 122:1-3");
    /// assert_eq!(three_psalms.len(), 3);
    /// assert_eq!(three_psalms[0].number, 120);
    /// assert_eq!(three_psalms[1].number, 121);
    /// assert_eq!(three_psalms[2].number, 122);
    /// assert_eq!(three_psalms[2].filtered_sections()[0].verses.len(), 3);
    /// ```
    pub fn psalms_by_citation(&self, citation: &str) -> Vec<Psalm> {
        let reference = BibleReference::from(citation);
        reference
            .ranges
            .iter()
            .map(|range| {
                if let Some(end) = range.end {
                    if let Some(start_chapter) = range.start.chapter {
                        if let Some(end_chapter) = end.chapter {
                            if start_chapter == end_chapter {
                                Box::new(once(start_chapter)) as Box<dyn Iterator<Item = u16>>
                            } else {
                                Box::new(start_chapter..=end_chapter)
                                    as Box<dyn Iterator<Item = u16>>
                            }
                        } else {
                            Box::new(once(start_chapter)) as Box<dyn Iterator<Item = u16>>
                        }
                    } else {
                        Box::new(once(1u16)) as Box<dyn Iterator<Item = u16>>
                    }
                } else {
                    Box::new(once(range.start.chapter.unwrap_or(1)))
                        as Box<dyn Iterator<Item = u16>>
                }
            })
            .flatten()
            .unique()
            .filter_map(|number| {
                // try to convert the psalm number from a u16 to a u8
                number
                    .try_into()
                    .ok()
                    // if that fails, or if no psalm with that number can be found, just filter it out
                    .and_then(|number| self.psalm_by_number(number))
            })
            .map({
                move |psalm| {
                    let mut new_psalm = Psalm {
                        number: psalm.number,
                        // TODO correctly convert this from a section of a citation into Some(String)
                        citation: Some(citation.to_string()),
                        sections: psalm.sections.clone(),
                    };
                    let filtered_sections = new_psalm.filtered_sections();
                    new_psalm.sections = filtered_sections;
                    new_psalm
                }
            })
            .collect::<Vec<_>>()
    }
}
