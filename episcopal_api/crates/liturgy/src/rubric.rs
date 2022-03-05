use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
