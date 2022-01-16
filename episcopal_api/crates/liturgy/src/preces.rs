use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// An explanatory sentence or direction for the liturgy
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Preces(Vec<(String, String)>);

impl Preces {
    pub fn iter(&self) -> impl Iterator<Item = &(String, String)> {
        self.0.iter()
    }

    pub fn into_vec(self) -> Vec<(String, String)> {
        self.0
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
