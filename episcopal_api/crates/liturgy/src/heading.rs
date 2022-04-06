use std::fmt::Display;

use calendar::Feast;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{Antiphon, Content, Rubric, Sentence, Text};

/// A title, subtitle, label, or other heading; can be used to automatically insert date/liturgical day name, or text with a level.
#[derive(
    Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, IntoStaticStr, EnumIter, EnumString,
)]
pub enum Heading {
    InsertDate,
    InsertDay,
    Date(String),
    Day {
        name: String,
        proper: Option<String>,
        holy_days: Option<Vec<(Feast, String)>>,
    },
    Text(HeadingLevel, String),
}

#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    IntoStaticStr,
    Display,
    EnumIter,
    EnumString,
)]
pub enum HeadingLevel {
    Heading1 = 5,
    Heading2 = 4,
    Heading3 = 3,
    Heading4 = 2,
    Heading5 = 1,
}

impl Default for HeadingLevel {
    fn default() -> Self {
        Self::Heading3
    }
}

impl<T> From<(HeadingLevel, T)> for Heading
where
    T: Display,
{
    fn from((level, text): (HeadingLevel, T)) -> Self {
        Self::Text(level, text.to_string())
    }
}

// Conversions
impl From<Content> for Heading {
    fn from(content: Content) -> Self {
        match content {
            Content::Heading(c) => c,
            Content::Antiphon(c) => Self::from(c),
            Content::Rubric(c) => Self::from(c),
            Content::Sentence(c) => Self::from(c),
            Content::Text(c) => Self::from(c),
            _ => Self::Text(HeadingLevel::default(), String::new()),
        }
    }
}

impl From<Antiphon> for Heading {
    fn from(content: Antiphon) -> Self {
        Self::Text(HeadingLevel::default(), content.to_string())
    }
}

impl From<Rubric> for Heading {
    fn from(content: Rubric) -> Self {
        Self::Text(HeadingLevel::default(), content.to_string())
    }
}

impl From<Sentence> for Heading {
    fn from(content: Sentence) -> Self {
        Self::Text(HeadingLevel::default(), content.text)
    }
}

impl From<Text> for Heading {
    fn from(content: Text) -> Self {
        Self::Text(HeadingLevel::default(), content.text)
    }
}
