use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::BiblicalReadingIntro;

/// A reference to a passage of the Bible, which will be inserted as a
/// [BibleReading](crate::BibleReading) by the compilation process.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalCitation {
    pub citation: String,
    pub intro: Option<BiblicalReadingIntro>,
}

impl BiblicalCitation {
    pub fn as_str(&self) -> &str {
        &self.citation
    }

    pub fn intro(mut self, intro: BiblicalReadingIntro) -> Self {
        self.intro = Some(intro);
        self
    }
}

impl Display for BiblicalCitation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.citation)
    }
}

impl From<String> for BiblicalCitation {
    fn from(text: String) -> Self {
        Self {
            citation: text,
            intro: None,
        }
    }
}

impl From<&str> for BiblicalCitation {
    fn from(text: &str) -> Self {
        Self::from(text.to_string())
    }
}
