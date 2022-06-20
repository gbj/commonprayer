use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A reference to a [Psalm](crate::Psalm), which will be inserted by the compilation process.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PsalmCitation(pub String);

impl PsalmCitation {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for PsalmCitation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<String> for PsalmCitation {
    fn from(text: String) -> Self {
        Self(text)
    }
}

impl From<&str> for PsalmCitation {
    fn from(text: &str) -> Self {
        Self(text.to_string())
    }
}
