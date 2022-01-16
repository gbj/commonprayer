use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A brief passage or verse, usually extracted from a psalm.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
