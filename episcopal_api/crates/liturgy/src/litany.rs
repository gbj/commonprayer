use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Content, Preces, ResponsivePrayer, Text};

/// A responsive prayer in which the same response is given to every petition
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Litany {
    pub response: String,
    pub lines: Vec<String>,
}

impl Litany {
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.lines.iter()
    }

    pub fn into_vec(self) -> Vec<String> {
        self.lines
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn remove_at_index(&mut self, index: usize) -> String {
        self.lines.remove(index)
    }
}

impl<R, T, A> From<(R, T)> for Litany
where
    T: IntoIterator<Item = A>,
    A: Display,
    R: Display,
{
    fn from((response, lines): (R, T)) -> Self {
        Self {
            response: response.to_string(),
            lines: lines.into_iter().map(|line| line.to_string()).collect(),
        }
    }
}

impl From<Vec<String>> for Litany {
    fn from(lines: Vec<String>) -> Self {
        Self {
            response: String::new(),
            lines,
        }
    }
}

// Conversions
impl From<Content> for Litany {
    fn from(content: Content) -> Self {
        match content {
            Content::Litany(c) => c,
            Content::Preces(c) => Self::from(c),
            Content::ResponsivePrayer(c) => Self::from(c),
            Content::Text(c) => Self::from(c),
            _ => Self::default(),
        }
    }
}

impl From<Preces> for Litany {
    fn from(content: Preces) -> Self {
        let vec = content.into_vec();
        let response = vec.get(1).cloned().unwrap_or_default().1;
        Self {
            response,
            lines: vec
                .into_iter()
                .enumerate()
                .filter_map(|(idx, (_, line))| if idx % 2 == 1 { Some(line) } else { None })
                .collect(),
        }
    }
}

impl From<ResponsivePrayer> for Litany {
    fn from(content: ResponsivePrayer) -> Self {
        let vec = content.into_vec();
        let response = vec.get(1).cloned().unwrap_or_default();
        Self {
            response,
            lines: vec
                .into_iter()
                .enumerate()
                .filter_map(|(idx, line)| if idx % 2 == 1 { Some(line) } else { None })
                .collect(),
        }
    }
}

impl From<Text> for Litany {
    fn from(content: Text) -> Self {
        let mut lines = content.text.split('\n');
        let first_line = lines.next().unwrap_or_default();
        let response = lines.next().unwrap_or_default();
        Self {
            response: response.to_string(),
            lines: std::iter::once(first_line)
                .chain(lines)
                .filter_map(move |line| {
                    if line.trim() == response {
                        None
                    } else {
                        Some(line.to_string())
                    }
                })
                .collect(),
        }
    }
}
