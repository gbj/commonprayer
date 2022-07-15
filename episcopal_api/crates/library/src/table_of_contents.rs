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
    /// # use library::{Library, CommonPrayer, Contents, Slug, SlugPath};
    /// # use library::bcp1979::office::NOONDAY_PRAYER;
    /// # use library::rite2::office::MORNING_PRAYER_II;
    /// # use liturgy::Version;
    /// let toc = CommonPrayer::contents();
    /// assert_eq!(toc.contents_at_path(&SlugPath::from([Slug::Office, Slug::NoondayPrayer]), Some(Contents::Document(&*NOONDAY_PRAYER)));
    /// let mp2 = toc.contents_at_path(&SlugPath::from([Slug::Office, Slug::MorningPrayer, Slug::Version(Version::RiteII)]);
    /// match mp2 {
    ///     Some(Contents::Document(doc)) => assert_eq!(doc.label, Some("Morning Prayer".to_string())),
    ///     _ => panic!("did not find Document node at [Slug::Office, Slug::MorningPrayer, Slug::Version(Version::RiteII)]\n\n{:#?}", mp2)
    /// }
    /// ```
    pub fn contents_at_path(&self, path: &SlugPath) -> Option<Contents<'a>> {
        let path = path.as_slice();
        let first_path_part = path.get(0)?;
        let (_, content_at_first_path_part) = self
            .0
            .iter()
            .find(|(s_slug, _)| s_slug == first_path_part)?;
        content_at_first_path_part.contents_at_path(&path[1..])
    }

    pub fn flatten(&self) -> impl Iterator<Item = (SlugPath, Contents<'_>)> {
        self.0
            .iter()
            .flat_map(|(slug, contents)| contents.flatten_with_starting_path(&[*slug]))
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
        label: String,
        intro: String,
        parallels: Vec<Vec<(ParallelDocument, usize)>>,
    },
    /// A static page in the website, which can be ignored for other purposes
    Page { label: String, url: String },
}

impl<'a> Contents<'a> {
    pub fn as_documents(&self) -> impl Iterator<Item = &Document> {
        match self {
            Contents::Page { .. } | Contents::Parallels { .. } => {
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
        if path.is_empty() {
            Some(self.clone())
        } else {
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
                Contents::Page { .. } | Contents::Document(_) | Contents::Parallels { .. } => {
                    Some(self.clone())
                }
            }
        }
    }

    pub fn label(&self) -> Option<String> {
        match self {
            Contents::Category { label, .. } => Some(label.clone()),
            Contents::Sections { label, .. } => Some(label.clone()),
            Contents::Document(doc) => doc.best_label(),
            Contents::ByVersion { label, .. } => Some(label.clone()),
            Contents::MultiDocument { label, .. } => Some(label.clone()),
            Contents::Parallels { label, .. } => Some(label.clone()),
            Contents::Page { label, .. } => Some(label.clone()),
        }
    }

    pub fn hidden_in_toc(&self) -> bool {
        match self {
            Contents::MultiDocument { hidden_in_toc, .. } => *hidden_in_toc,
            _ => false,
        }
    }

    pub fn flatten_with_starting_path(
        &self,
        starting_path: &[Slug],
    ) -> impl Iterator<Item = (SlugPath, Contents)> {
        let children: Box<dyn Iterator<Item = (SlugPath, Contents)>> = match self {
            Contents::Category { contents, .. } => Box::new(contents.iter().flat_map({
                let starting_path = starting_path.to_vec();
                move |(slug, contents)| {
                    let mut starting_path = starting_path.clone();
                    starting_path.push(*slug);
                    std::iter::once((SlugPath::from(starting_path.clone()), contents.clone()))
                        .chain(contents.flatten_with_starting_path(&starting_path))
                }
            })),
            Contents::Sections { contents, .. } => Box::new(contents.iter().flat_map({
                let starting_path = starting_path.to_vec();
                move |section| {
                    section.contents.iter().flat_map({
                        let starting_path = starting_path.clone();
                        move |(slug, contents)| {
                            let mut starting_path = starting_path.clone();
                            starting_path.push(*slug);
                            std::iter::once((
                                SlugPath::from(starting_path.clone()),
                                contents.clone(),
                            ))
                            .chain(contents.flatten_with_starting_path(&starting_path))
                        }
                    })
                }
            })),
            Contents::ByVersion { documents, .. } => Box::new(documents.iter().map({
                let starting_path = starting_path.to_vec();
                move |document| {
                    let mut starting_path = starting_path.clone();
                    starting_path.push(Slug::Version(document.version));
                    (SlugPath::from(starting_path), Contents::Document(document))
                }
            })),
            Contents::Page { .. }
            | Contents::Document(_)
            | Contents::Parallels { .. }
            | Contents::MultiDocument { .. } => Box::new(std::iter::empty()),
        };
        std::iter::once((SlugPath::from(starting_path.to_vec()), self.clone())).chain(children)
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
