use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A responsive prayer in which the same response is given to every petition
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
