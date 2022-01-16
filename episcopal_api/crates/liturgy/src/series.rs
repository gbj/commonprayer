use serde::{Deserialize, Serialize};

use crate::{Choice, Document, Parallel};

/// Multiple [Document](crate::Document)s that are displayed in order.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Series(Vec<Document>);

impl Series {
    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.0.iter()
    }

    pub fn into_vec(self) -> Vec<Document> {
        self.0
    }

    pub fn as_slice(&self) -> &[Document] {
        &self.0
    }

    pub fn as_mut_slice(&mut self) -> &mut [Document] {
        &mut self.0
    }
}

impl<T, U> From<T> for Series
where
    T: IntoIterator<Item = U>,
    U: Into<Document>,
{
    fn from(items: T) -> Self {
        Self(items.into_iter().map(|item| item.into()).collect())
    }
}

impl From<Choice> for Series {
    fn from(choice: Choice) -> Self {
        Self::from(choice.options)
    }
}

impl From<Parallel> for Series {
    fn from(parallel: Parallel) -> Self {
        Self::from(parallel.into_vec())
    }
}
