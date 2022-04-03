use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Content, Heading, Rubric, Sentence, Text};

/// A brief passage or verse, usually extracted from a psalm.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Antiphon(String);

impl From<String> for Antiphon {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl From<&str> for Antiphon {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}

impl Display for Antiphon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Conversions
impl From<Content> for Antiphon {
    fn from(f: Content) -> Self {
        match f {
            Content::Antiphon(c) => c,
            Content::Heading(c) => Self::from(c),
            Content::Rubric(c) => Self::from(c),
            Content::Sentence(c) => Self::from(c),
            Content::Text(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Heading> for Antiphon {
    fn from(f: Heading) -> Self {
        match f {
            Heading::Text(_, text) => Self(text),
            _ => Self::default(),
        }
    }
}

impl From<Rubric> for Antiphon {
    fn from(f: Rubric) -> Self {
        Self(f.to_string())
    }
}

impl From<Sentence> for Antiphon {
    fn from(f: Sentence) -> Self {
        Self(f.text)
    }
}

impl From<Text> for Antiphon {
    fn from(f: Text) -> Self {
        Self(f.text)
    }
}
