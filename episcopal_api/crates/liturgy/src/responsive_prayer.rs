use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Content, Litany, Preces, Text};

/// A simple responsive prayer in which the leader and participants alternate.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponsivePrayer(Vec<String>);

impl ResponsivePrayer {
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn into_vec(self) -> Vec<String> {
        self.0
    }

    pub fn push(&mut self, line: String) {
        self.0.push(line);
    }

    pub fn remove_at_index(&mut self, index: usize) -> String {
        self.0.remove(index)
    }
}

impl<T, A> From<T> for ResponsivePrayer
where
    T: IntoIterator<Item = A>,
    A: Display,
{
    fn from(content: T) -> Self {
        Self(content.into_iter().map(|line| line.to_string()).collect())
    }
}

// Conversions
impl From<Content> for ResponsivePrayer {
    fn from(content: Content) -> Self {
        match content {
            Content::ResponsivePrayer(c) => c,
            Content::Litany(c) => Self::from(c),
            Content::Preces(c) => Self::from(c),
            Content::Text(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Litany> for ResponsivePrayer {
    fn from(content: Litany) -> Self {
        let response = content.response.clone();
        Self(
            content
                .into_vec()
                .into_iter()
                .flat_map(|line| std::iter::once(line).chain(std::iter::once(response.clone())))
                .collect(),
        )
    }
}

impl From<Preces> for ResponsivePrayer {
    fn from(content: Preces) -> Self {
        Self(content.into_vec().into_iter().map(|(_, r)| r).collect())
    }
}

impl From<Text> for ResponsivePrayer {
    fn from(content: Text) -> Self {
        Self(content.text.split('\n').map(String::from).collect())
    }
}
