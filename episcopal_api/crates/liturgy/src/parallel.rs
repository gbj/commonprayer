use serde::{Deserialize, Serialize};

use crate::{Choice, Content, Document, Liturgy, Series};

/// Multiple [Document](crate::Document)s that are displayed side by side.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Parallel(Vec<Document>);

impl Parallel {
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

    pub fn push(&mut self, document: Document) {
        self.0.push(document)
    }

    pub fn remove_at_index(&mut self, index: usize) -> Document {
        self.0.remove(index)
    }

    pub fn insert_at(&mut self, index: usize, doc: Document) {
        self.0.insert(index, doc)
    }
}

impl<T, U> From<T> for Parallel
where
    T: IntoIterator<Item = U>,
    U: Into<Document>,
{
    fn from(items: T) -> Self {
        Self(items.into_iter().map(|item| item.into()).collect())
    }
}

// Conversions
impl From<Content> for Parallel {
    fn from(content: Content) -> Self {
        match content {
            Content::Parallel(c) => c,
            Content::Series(c) => Self::from(c),
            Content::Liturgy(c) => Self::from(c),
            Content::Choice(c) => Self::from(c),
            _ => Self::from([content]),
        }
    }
}

impl From<Liturgy> for Parallel {
    fn from(content: Liturgy) -> Self {
        Self::from(content.body)
    }
}

impl From<Series> for Parallel {
    fn from(series: Series) -> Self {
        Self::from(series.into_vec())
    }
}

impl From<Choice> for Parallel {
    fn from(choice: Choice) -> Self {
        Self::from(choice.options)
    }
}
