use serde::{Deserialize, Serialize};

/// Different modes of displaying a [Document](crate::Document)
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum DisplayFormat {
    /// Show the document with its default formatting
    Default,
    /// Show only the opening and closing words
    Abbreviated,
    /// Omit the text itself, showing only labels/headings
    Omit,
    /// Style the text to indicate it should be said in unison (e.g., bolded)
    Unison,
}

impl DisplayFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            DisplayFormat::Default => "default",
            DisplayFormat::Abbreviated => "abbreviated",
            DisplayFormat::Omit => "omit",
            DisplayFormat::Unison => "unison",
        }
    }
}
