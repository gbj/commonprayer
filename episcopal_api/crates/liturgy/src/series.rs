use serde::{Deserialize, Serialize};

use crate::{Choice, Content, Document, Liturgy, Parallel};

/// Multiple [Document](crate::Document)s that are displayed in order.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Series(Vec<Document>, bool);

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

    pub fn is_indivisible(&self) -> bool {
        self.1
    }

    pub fn indivisible(mut self) -> Self {
        self.1 = true;
        self
    }

    pub fn push(&mut self, doc: Document) {
        self.0.push(doc)
    }
}

impl<T, U> From<T> for Series
where
    T: IntoIterator<Item = U>,
    U: Into<Document>,
{
    fn from(items: T) -> Self {
        Self(items.into_iter().map(|item| item.into()).collect(), false)
    }
}

// Conversions
impl From<Content> for Series {
    fn from(content: Content) -> Self {
        match content {
            Content::Series(c) => c,
            Content::Choice(c) => Self::from(c),
            Content::Parallel(c) => Self::from(c),
            Content::Liturgy(c) => Self::from(c),
            _ => Self::from([Document::from(content)]),
        }
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

impl From<Liturgy> for Series {
    fn from(liturgy: Liturgy) -> Self {
        liturgy.body
    }
}
