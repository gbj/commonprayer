use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Content, Litany, ResponsivePrayer, Text};

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Preces(Vec<(String, String)>);

impl Preces {
    pub fn iter(&self) -> impl Iterator<Item = &(String, String)> {
        self.0.iter()
    }

    pub fn into_vec(self) -> Vec<(String, String)> {
        self.0
    }

    pub fn push(&mut self, line: (String, String)) {
        self.0.push(line);
    }
}

impl<T, A, B> From<T> for Preces
where
    T: IntoIterator<Item = (A, B)>,
    A: Display,
    B: Display,
{
    fn from(source: T) -> Self {
        Self(
            source
                .into_iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect(),
        )
    }
}

// Conversions
impl From<Content> for Preces {
    fn from(content: Content) -> Self {
        match content {
            Content::Preces(c) => c,
            Content::Text(c) => Self::from(c),
            Content::Litany(c) => Self::from(c),
            Content::ResponsivePrayer(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Text> for Preces {
    fn from(content: Text) -> Self {
        Self(
            content
                .text
                .split('\n')
                .map(|line| {
                    let mut parts = line.splitn(2, [' ', '\t']);
                    (
                        parts.next().unwrap_or_default().trim().to_string(),
                        parts.next().unwrap_or_default().trim().to_string(),
                    )
                })
                .collect(),
        )
    }
}

impl From<ResponsivePrayer> for Preces {
    fn from(content: ResponsivePrayer) -> Self {
        Self(
            content
                .into_vec()
                .into_iter()
                .map(|line| (String::new(), line))
                .collect(),
        )
    }
}

impl From<Litany> for Preces {
    fn from(content: Litany) -> Self {
        let response = content.response.clone();
        Self(
            content
                .into_vec()
                .into_iter()
                .flat_map({
                    move |line| {
                        std::iter::once((String::new(), line))
                            .chain(std::iter::once((String::new(), response.clone())))
                    }
                })
                .collect(),
        )
    }
}
