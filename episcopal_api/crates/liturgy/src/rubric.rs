use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Antiphon, Content, Heading, Sentence, Text};

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rubric {
    pub text: String,
    /// The BCP formats some longer rubrics as smaller-than-usual text
    /// but without italicizing or coloring them red
    pub long: bool,
}

impl From<String> for Rubric {
    fn from(text: String) -> Self {
        Self { text, long: false }
    }
}

impl From<&str> for Rubric {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            long: false,
        }
    }
}

impl Display for Rubric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Rubric {
    pub fn long(mut self) -> Self {
        self.long = true;
        self
    }
}

// Conversions
impl From<Content> for Rubric {
    fn from(f: Content) -> Self {
        match f {
            Content::Rubric(c) => c,
            Content::Antiphon(c) => Self::from(c),
            Content::Heading(c) => Self::from(c),
            Content::Sentence(c) => Self::from(c),
            Content::Text(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Antiphon> for Rubric {
    fn from(f: Antiphon) -> Self {
        Self::from(f.to_string())
    }
}

impl From<Heading> for Rubric {
    fn from(f: Heading) -> Self {
        match f {
            Heading::Text(_, text) => Self::from(text),
            _ => Self::default(),
        }
    }
}

impl From<Sentence> for Rubric {
    fn from(f: Sentence) -> Self {
        Self::from(f.text)
    }
}

impl From<Text> for Rubric {
    fn from(f: Text) -> Self {
        Self::from(f.text)
    }
}
