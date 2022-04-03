use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Content, Document, Heading};

/// A short Biblical reading, with an optional response.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Sentence {
    pub text: String,
    pub citation: Option<String>,
    pub response: Option<Box<Document>>,
}

impl Sentence {
    #[must_use]
    pub fn citation<T>(mut self, citation: T) -> Self
    where
        T: Display,
    {
        self.citation = Some(citation.to_string());
        self
    }

    #[must_use]
    pub fn response<T>(mut self, response: T) -> Self
    where
        T: Into<Document>,
    {
        self.response = Some(Box::new(response.into()));
        self
    }
}

impl<T> From<T> for Sentence
where
    T: Display,
{
    fn from(text: T) -> Self {
        Self {
            text: text.to_string(),
            citation: None,
            response: None,
        }
    }
}

// Conversions
impl From<Content> for Sentence {
    fn from(f: Content) -> Self {
        match f {
            Content::Sentence(c) => c,
            Content::Antiphon(c) => Self::from(c),
            Content::Heading(c) => Self::from(c),
            Content::Rubric(c) => Self::from(c),
            Content::Text(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Heading> for Sentence {
    fn from(f: Heading) -> Self {
        match f {
            Heading::Text(_, text) => Self::from(text),
            _ => Self::default(),
        }
    }
}
