use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::Document;

/// A short Biblical reading, with an optional response.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Sentence {
    pub text: String,
    pub citation: Option<String>,
    pub response: Option<Box<Document>>,
}

impl Sentence {
    pub fn citation<T>(mut self, citation: T) -> Self
    where
        T: Display,
    {
        self.citation = Some(citation.to_string());
        self
    }

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
