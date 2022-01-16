use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::DisplayFormat;

/// Text, without any additional styling or semantics
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    pub display_format: DisplayFormat,
    pub response: Option<String>,
}

impl Text {
    pub fn display_format(mut self, display_format: DisplayFormat) -> Self {
        self.display_format = display_format;
        self
    }

    pub fn response<T>(mut self, response: T) -> Self
    where
        T: Display,
    {
        self.response = Some(response.to_string());
        self
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self {
            text,
            display_format: DisplayFormat::Default,
            response: None,
        }
    }
}

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            display_format: DisplayFormat::Default,
            response: None,
        }
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}
