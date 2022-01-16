use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Rubric(String);

impl From<String> for Rubric {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl From<&str> for Rubric {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}

impl Display for Rubric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
