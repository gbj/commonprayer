use itertools::Itertools;
use liturgy::{parallel_table::ParallelDocument, *};

use crate::{CollectData, CollectId};

pub struct TableOfContents<'a>(Vec<(Slug, Contents<'a>)>);

impl<'a> From<Vec<(Slug, Contents<'a>)>> for TableOfContents<'a> {
    fn from(toc: Vec<(Slug, Contents<'a>)>) -> Self {
        Self(toc)
    }
}

impl<'a> TableOfContents<'a> {
    /// Finds the content of the table of contents by traversing the tree
    /// and searching for each part of the path in sequence.
    /// ```
    /// # use library::{Library, CommonPrayer, Contents, Slug};
    /// # use library::bcp1979::office::NOONDAY_PRAYER;
    /// # use library::rite2::office::MORNING_PRAYER_II;
    /// # use liturgy::Version;
    /// let toc = CommonPrayer::contents();
    /// assert_eq!(toc.contents_at_path(&[Slug::Office, Slug::NoondayPrayer]), Some(Contents::Document(&*NOONDAY_PRAYER)));
    /// let mp2 = toc.contents_at_path(&[Slug::Office, Slug::MorningPrayer, Slug::Version(Version::RiteII)]);
    /// match mp2 {
    ///     Some(Contents::Document(doc)) => assert_eq!(doc.label, Some("Morning Prayer".to_string())),
    ///     _ => panic!("did not find Document node at [Slug::Office, Slug::MorningPrayer, Slug::Version(Version::RiteII)]\n\n{:#?}", mp2)
    /// }
    /// ```
    pub fn contents_at_path(&self, path: &[Slug]) -> Option<Contents<'a>> {
        let first_path_part = path.get(0)?;
        let (_, content_at_first_path_part) = self
            .0
            .iter()
            .find(|(s_slug, _)| s_slug == first_path_part)?;
        content_at_first_path_part.contents_at_path(&path[1..])
    }
}

/// The nodes in a table-of-contents tree for a library
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Contents<'a> {
    /// A labeled branch that contains other [Contents]
    Category {
        label: String,
        contents: Vec<(Slug, Contents<'a>)>,
    },
    Sections {
        label: String,
        contents: Vec<Section<'a>>,
    },
    /// A leaf node containing a single [Document](liturgy::Document) (e.g., Morning Prayer or Eucharistic Prayer A)
    Document(&'a Document),
    /// Multiple equivalent documents distinguished by their [Version](liturgy::Version) fields
    /// (e.g., Morning Prayer Rite I vs. Rite II)
    ByVersion {
        label: String,
        documents: Vec<&'a Document>,
    },
    /// A leaf node containing multiple [Document](liturgy::Document)s that can either
    /// a) be rendered on a single page and searched or
    /// b) be randomized
    /// (e.g., Offertory Sentences).
    MultiDocument {
        label: String,
        documents: Vec<Document>,
        hidden_in_toc: bool,
    },
    /// Information to build a page of parallels
    Parallels {
        intro: String,
        parallels: Vec<Vec<(ParallelDocument, usize)>>,
    },
    /// A static page in the website, which can be ignored for other purposes
    Page(&'static str),
}

impl<'a> Contents<'a> {
    pub fn as_documents(&self) -> impl Iterator<Item = &Document> {
        match self {
            Contents::Page(_) | Contents::Parallels { .. } => {
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = &Document>>
            }
            Contents::Category { contents, .. } => Box::new(
                contents
                    .iter()
                    .flat_map(|(_, contents)| Box::new(contents.as_documents())),
            ),
            Contents::Sections { contents, .. } => Box::new(
                contents
                    .iter()
                    .flat_map(|section| &section.contents)
                    .flat_map(|(_, contents)| Box::new(contents.as_documents())),
            ),
            Contents::Document(doc) => Box::new(std::iter::once(*doc)),
            Contents::ByVersion { documents, .. } => Box::new(documents.iter().copied()),
            Contents::MultiDocument { documents, .. } => Box::new(documents.iter()),
        }
    }

    pub fn contents_at_path(&self, path: &[Slug]) -> Option<Contents<'a>> {
        match self {
            Contents::Category { contents, .. } => path.get(0).and_then(|slug| {
                contents
                    .iter()
                    .find(|(s_slug, _)| s_slug == slug)
                    .and_then(|(_, contents)| contents.contents_at_path(&path[1..]))
            }),
            Contents::Sections { contents, .. } => path.get(0).and_then(|slug| {
                contents
                    .iter()
                    .flat_map(|section| section.contents.iter())
                    .find(|(s_slug, _)| s_slug == slug)
                    .and_then(|(_, contents)| contents.contents_at_path(&path[1..]))
            }),
            Contents::ByVersion { documents, .. } => {
                if let Some(Slug::Version(version)) = path.get(0) {
                    documents
                        .iter()
                        .find(|doc| doc.version == *version)
                        .copied()
                        .map(Contents::Document)
                } else {
                    Some(self.clone())
                }
            }
            Contents::MultiDocument {
                label,
                documents,
                hidden_in_toc,
            } => {
                if let Some(Slug::Version(version)) = path.get(0) {
                    Some(Contents::MultiDocument {
                        label: label.clone(),
                        hidden_in_toc: *hidden_in_toc,
                        documents: documents
                            .iter()
                            .filter(|doc| doc.version == *version)
                            .cloned()
                            .collect(),
                    })
                } else {
                    Some(self.clone())
                }
            }
            // No children to be sorted these leaf-node variants, so simply return this leaf
            Contents::Page(_) | Contents::Document(_) | Contents::Parallels { .. } => {
                Some(self.clone())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section<'a> {
    pub label: Option<String>,
    pub contents: Vec<(Slug, Contents<'a>)>,
}

impl<'a> From<(String, &[(CollectId, CollectData)])> for Contents<'a> {
    fn from((label, collects): (String, &[(CollectId, CollectData)])) -> Self {
        let grouped_by_category = collects
            .iter()
            .group_by(|(_, data)| data.document.tags.get(0));
        let documents = grouped_by_category
            .into_iter()
            .flat_map(|(category, data)| {
                std::iter::once(Document::from(Heading::from((
                    HeadingLevel::Heading2,
                    category.cloned().unwrap_or_default(),
                ))))
                .chain(data.dedup_by(|a, b| a.1.document == b.1.document).map(
                    |(_, data)| {
                        let mut pieces = Vec::new();

                        if let Some(text) = &data.rubric_before {
                            pieces.push(Document::from(Rubric::from(text.clone())))
                        }
                        pieces.push(Document {
                            label: None,
                            subtitle: None,
                            ..data.document.clone()
                        });
                        if !data.preface.is_empty() {
                            pieces.push(Document::from(Rubric::from(data.preface.clone())));
                        }
                        if let Some(text) = &data.rubric_after {
                            pieces.push(Document::from(Rubric::from(text.clone())))
                        }

                        let mut series = Document::from(Series::from(pieces));
                        series.label = data.document.label.clone();
                        series.subtitle = data.document.subtitle.clone();
                        series
                    },
                ))
            })
            .collect();
        Contents::MultiDocument {
            label,
            documents,
            hidden_in_toc: false,
        }
    }
}
