use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Antiphon, BiblicalReading, BiblicalReadingIntro, Content, Rubric, Text};

/// A reference to a passage of the Bible, which will be inserted as a
/// [BibleReading](crate::BibleReading) by the compilation process.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct BiblicalCitation {
    pub citation: String,
    pub intro: Option<BiblicalReadingIntro>,
}

impl BiblicalCitation {
    pub fn as_str(&self) -> &str {
        &self.citation
    }

    #[must_use]
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

// Conversions
impl From<Content> for BiblicalCitation {
    fn from(content: Content) -> Self {
        match content {
            Content::BiblicalCitation(c) => c,
            Content::Antiphon(c) => Self::from(c),
            Content::BiblicalReading(c) => Self::from(c),
            Content::Rubric(c) => Self::from(c),
            Content::Text(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Antiphon> for BiblicalCitation {
    fn from(content: Antiphon) -> Self {
        Self::from(content.to_string())
    }
}

impl From<BiblicalReading> for BiblicalCitation {
    fn from(content: BiblicalReading) -> Self {
        Self::from(content.citation)
    }
}

impl From<Rubric> for BiblicalCitation {
    fn from(content: Rubric) -> Self {
        Self::from(content.to_string())
    }
}

impl From<Text> for BiblicalCitation {
    fn from(content: Text) -> Self {
        Self::from(content.to_string())
    }
}
