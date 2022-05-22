use serde::{Deserialize, Serialize};

use crate::{Choice, Content, Document, Liturgy, Parallel};

/// Multiple [Document](crate::Document)s that are displayed in order.
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Series {
    children: Vec<Document>,
    //#[serde(skip_serializing_if = "crate::is_false", default)]
    indivisible: bool,
}

impl Series {
    pub fn iter(&self) -> impl Iterator<Item = &Document> {
        self.children.iter()
    }

    pub fn into_vec(self) -> Vec<Document> {
        self.children
    }

    pub fn as_slice(&self) -> &[Document] {
        &self.children
    }

    pub fn as_mut_slice(&mut self) -> &mut [Document] {
        &mut self.children
    }

    pub fn is_indivisible(&self) -> bool {
        self.indivisible
    }

    pub fn indivisible(mut self) -> Self {
        self.indivisible = true;
        self
    }

    pub fn push(&mut self, doc: Document) {
        self.children.push(doc)
    }

    pub fn remove_at_index(&mut self, index: usize) -> Document {
        self.children.remove(index)
    }

    pub fn insert_at(&mut self, index: usize, doc: Document) {
        self.children.insert(index, doc)
    }
}

impl<T, U> From<T> for Series
where
    T: IntoIterator<Item = U>,
    U: Into<Document>,
{
    fn from(items: T) -> Self {
        Self {
            children: items.into_iter().map(|item| item.into()).collect(),
            indivisible: false,
        }
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
