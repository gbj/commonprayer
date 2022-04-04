use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Antiphon, Content, DisplayFormat, Heading, Preces, Rubric, Sentence};

/// Text, without any additional styling or semantics
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    pub display_format: DisplayFormat,
    pub response: Option<String>,
}

impl Text {
    #[must_use]
    pub fn display_format(mut self, display_format: DisplayFormat) -> Self {
        self.display_format = display_format;
        self
    }

    #[must_use]
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

// Conversions
impl From<Content> for Text {
    fn from(f: Content) -> Self {
        match f {
            Content::Text(c) => c,
            Content::Antiphon(c) => Self::from(c),
            Content::Heading(c) => Self::from(c),
            Content::Preces(c) => Self::from(c),
            Content::Rubric(c) => Self::from(c),
            Content::Sentence(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Antiphon> for Text {
    fn from(f: Antiphon) -> Self {
        Self::from(f.to_string())
    }
}

impl From<Heading> for Text {
    fn from(f: Heading) -> Self {
        match f {
            Heading::Text(_, text) => Self::from(text),
            _ => Self::default(),
        }
    }
}

impl From<Preces> for Text {
    fn from(f: Preces) -> Self {
        Self::from(
            f.into_vec()
                .into_iter()
                .map(|(v, r)| format!("{v}\t{r}"))
                .intersperse_with(|| String::from("\n"))
                .collect::<String>(),
        )
    }
}

impl From<Rubric> for Text {
    fn from(f: Rubric) -> Self {
        Self::from(f.to_string())
    }
}

impl From<Sentence> for Text {
    fn from(f: Sentence) -> Self {
        Self::from(f.text)
    }
}
