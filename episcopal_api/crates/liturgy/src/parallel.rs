use std::fmt::format;

use serde::{Deserialize, Serialize};

use crate::{version, Choice, Content, Document, Liturgy, Series};

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

pub fn parallelize(doc_a: &Document, doc_b: &Document) -> Document {
    match (&doc_a.content, &doc_b.content) {
        (Content::Liturgy(content_a), Content::Liturgy(content_b)) => {
            let docs = content_a
                .body
                .iter()
                .zip(content_b.body.iter())
                .map(|(a, b)| if a == b { a.clone() } else { parallelize(a, b) });
            let content = Liturgy {
                body: Series::from(docs),
                ..content_a.clone()
            };
            Document {
                content: Content::Liturgy(content),
                ..doc_a.clone()
            }
        }
        (Content::Series(content_a), Content::Series(content_b)) => {
            let docs = content_a.iter().zip(content_b.iter()).map(|(a, b)| {
                if a == b {
                    a.clone()
                } else {
                    parallelize(a, b)
                }
            });
            Document {
                content: Content::Series(Series::from(docs)),
                ..doc_a.clone()
            }
        }
        (Content::Choice(content_a), Content::Choice(content_b)) => {
            let options = content_a
                .options
                .iter()
                .enumerate()
                .zip(content_b.options.iter().enumerate())
                .map(|((a_idx, a), (b_idx, b))| {
                    let version_label_a = content_a.option_label(a, a_idx);
                    let version_label_b = content_b.option_label(b, b_idx);
                    let version_label = if version_label_a == version_label_b {
                        version_label_a
                    } else {
                        format!("{version_label_a}/{version_label_b}")
                    };

                    if a.content == b.content {
                        a.clone()
                    } else {
                        parallelize(a, b)
                    }
                    .version_label(version_label)
                })
                .collect();
            let content = Choice {
                options,
                ..content_a.clone()
            };
            Document {
                content: Content::Choice(content),
                ..doc_a.clone()
            }
        }
        _ => Document::from(Parallel::from(vec![doc_a.clone(), doc_b.clone()])),
    }
}
