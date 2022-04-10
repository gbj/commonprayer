use language::Language;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Inserts all documents filed under this category in the library.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Category {
    pub name: Categories,
    /// If `true`, the compiler will only show one [Document](crate::Document),
    /// rotating by day in a deterministic way. If `false`, it will show them all
    /// as a [Choice](crate::Choice).
    pub rotate: bool,
}

impl From<Categories> for Category {
    fn from(name: Categories) -> Self {
        Self { name, rotate: true }
    }
}

impl Category {
    pub fn rotate(mut self) -> Self {
        self.rotate = true;
        self
    }
}

#[derive(
    Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display,
)]
pub enum Categories {
    OpeningSentences,
    ClosingSentences,
    OffertorySentences,
    InvitatoryAntiphons,
    PrayersAndThanksgivings,
    AdditionalPrayers,
}

impl Categories {
    // TODO i18n when we add other languages
    pub fn localized_name(&self, _language: Language) -> &'static str {
        match self {
            Categories::OpeningSentences => "Opening Sentences",
            Categories::ClosingSentences => "Closing Sentences",
            Categories::OffertorySentences => "Offertory Sentences",
            Categories::InvitatoryAntiphons => "Invitatory Antiphons",
            Categories::PrayersAndThanksgivings => "Prayers and Thanksgivings",
            Categories::AdditionalPrayers => "Additional Prayers",
        }
    }
}
