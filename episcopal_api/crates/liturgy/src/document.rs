use std::fmt::Display;

use calendar::{Calendar, LiturgicalDay};
use language::Language;
use serde::{Deserialize, Serialize};
use status::Status;

use crate::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Document {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub condition: Option<Condition>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subtitle: Option<String>,
    #[serde(skip_serializing_if = "Language::is_default", default)]
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<Reference>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub alternate_sources: Vec<Reference>,
    #[serde(skip_serializing_if = "Status::is_default", default)]
    pub status: Status,
    #[serde(skip_serializing_if = "Show::is_default", default)]
    pub display: Show,
    #[serde(skip_serializing_if = "Version::is_default", default)]
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version_label: Option<String>,
    #[serde(skip_serializing_if = "crate::is_false", default)]
    pub optional: bool,
    pub content: Content,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub explainer: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    pub is_compiled: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            condition: None,
            label: None,
            subtitle: None,
            language: Language::default(),
            source: None,
            alternate_sources: Vec::new(),
            status: Status::default(),
            display: Show::default(),
            version: Version::default(),
            version_label: None,
            optional: false,
            content: Content::Empty,
            explainer: None,
            is_compiled: false,
            tags: Vec::new()
        }
    }

    /// Whether a `Document` should be included in the liturgy or omitted, based on its included [Condition]s.
    pub fn include(
        &self,
        calendar: &Calendar,
        day: &LiturgicalDay,
        prefs: &impl ClientPreferences,
        original_prefs: &LiturgyPreferences,
    ) -> bool {
        match &self.condition {
            None => true,
            Some(condition) => condition.include(calendar, day, prefs, original_prefs),
        }
    }

    /// Transforms a [Document], which can nest sub-documents in a [Liturgy], [Series], [Choice], or [Parallel],
    /// into a flattened list of children
    pub fn flatten(&self) -> Vec<&Document> {
        let children = match &self.content {
            Content::Liturgy(liturgy) => Some(liturgy.body.iter().collect::<Vec<_>>()),
            Content::Series(series) => Some(series.iter().collect::<Vec<_>>()),
            Content::Parallel(parallel) => Some(parallel.iter().collect::<Vec<_>>()),
            Content::Choice(choice) => Some(choice.options.iter().collect::<Vec<_>>()),
            _ => None,
        };
        if let Some(children) = children {
            children.iter().flat_map(|child| child.flatten()).collect()
        } else {
            vec![self]
        }
    }

    /// Transforms a [Document], which can nest sub-documents in a [Liturgy], [Series], [Choice], or [Parallel],
    /// into a flattened list of [Document]s paired with the paths to get to them
    /// ```
    /// # use crate::liturgy::*;
    /// let doc = Document::from(Series::from(vec![Document::from(Text::from("Child #1")), Document::from(Text::from("Child #2"))]));
    /// let paths = doc.flatten_with_path().map(|(path, _)| path).collect::<Vec<_>>();
    /// assert_eq!(paths[0], vec![0]);
    /// assert_eq!(paths[1], vec![1]);
    /// ```
    pub fn flatten_with_path(&self, as_template: bool) -> impl Iterator<Item = (Vec<usize>, &Document)> {
        fn flatten_doc_with_path_starting_from_path(doc: &Document, starting_path: Vec<usize>, as_template: bool) -> impl Iterator<Item = (Vec<usize>, &Document)>  {
            let children = match &doc.content {
                Content::Liturgy(liturgy) => Some(liturgy.body.iter().collect::<Vec<_>>()),
                Content::Series(series) => Some(series.iter().collect::<Vec<_>>()),
                Content::Parallel(parallel) => Some(parallel.iter().collect::<Vec<_>>()),
                Content::Choice(choice) => Some(choice.options.iter().collect::<Vec<_>>()),
                _ => None,
            };
            if let Some(children) = children {
                Box::new(
                    children
                        .into_iter()
                        .filter(move |doc| doc.display != Show::CompiledOnly || !as_template)
                        .enumerate()
                        .flat_map({
                            move |(idx, child)| {
                                let mut new_path = starting_path.clone();
                                new_path.push(idx);
                                flatten_doc_with_path_starting_from_path(child, new_path, as_template)
                            }
                        })
                ) as Box<dyn Iterator<Item = (Vec<usize>, &Document)>>
            } else if matches!(doc.content, Content::DocumentLink { .. } | Content::HymnLink(_) | Content::LectionaryReading(_)) {
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = (Vec<usize>, &Document)>>
            }
            else if doc.display == Show::CompiledOnly && as_template {
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = (Vec<usize>, &Document)>>
            } else {
                Box::new(std::iter::once((starting_path, doc))) as Box<dyn Iterator<Item = (Vec<usize>, &Document)>>
            }
        }

        flatten_doc_with_path_starting_from_path(self, Vec::new(), as_template)
    }

    /// Builds a new Document from an iterator of Documents; either a [Choice] (if multiple Documents) or a single Document.
    pub fn choice_or_document<I>(docs: &mut I) -> Option<Document>
    where
        I: Iterator<Item = Document>,
    {
        match (docs.next(), docs.next()) {
            (None, None) => None,
            (None, Some(doc)) => Some(doc),
            (Some(doc), None) => Some(doc),
            (Some(a), Some(b)) => Some(Document::from(Choice::from(
                std::iter::once(a).chain(std::iter::once(b)).chain(docs),
            ))),
        }
    }

    /// Builds a new Document from an iterator of Documents; either a [Series] (if multiple Documents) or a single Document.
    pub fn series_or_document<I>(docs: &mut I) -> Option<Document>
    where
        I: Iterator<Item = Document>,
    {
        match (docs.next(), docs.next()) {
            (None, None) => None,
            (None, Some(doc)) => Some(doc),
            (Some(doc), None) => Some(doc),
            (Some(a), Some(b)) => Some(Document::from(Series::from(
                std::iter::once(a).chain(std::iter::once(b)).chain(docs),
            ))),
        }
    }

    /// Builds a new Document from an iterator of Documents; either a [Parallel] (if multiple Documents) or a single Document.
    pub fn parallel_or_document<I>(docs: &mut I) -> Option<Document>
    where
        I: Iterator<Item = Document>,
    {
        match (docs.next(), docs.next()) {
            (None, None) => None,
            (None, Some(doc)) => Some(doc),
            (Some(doc), None) => Some(doc),
            (Some(a), Some(b)) => Some(Document::from(Parallel::from(
                std::iter::once(a).chain(std::iter::once(b)).chain(docs),
            ))),
        }
    }

    /// Whether the content of the document changes at all depending on the [LiturgicalDay](calendar::LiturgicalDay)
    /// ```
    /// # use crate::liturgy::{Document, Text, Series, Condition};
    /// use library::conditions::NOT_LENT;
    /// assert!(NOT_LENT.is_date_condition());
    /// let doc = Document::from(Text::from("Alleluia.")).condition(NOT_LENT.clone());
    /// assert!(doc.has_date_condition());
    /// let series = Document::from(Series::from(vec![Document::from(Text::from("Alleluia.")).condition(NOT_LENT.clone())]));
    /// assert!(series.has_date_condition());
    /// assert!(library::bcp1979::office::COMPLINE.has_date_condition());
    /// ```
    pub fn has_date_condition(&self) -> bool {
        let has_own_date_condition = self.condition.as_ref().map(|condition| condition.is_date_condition()).unwrap_or(false);
        let has_child_date_condition = match &self.content {
            Content::Series(series) => series.iter().any(|doc| doc.has_date_condition()),
            Content::Parallel(parallel) => parallel.iter().any(|doc| doc.has_date_condition()),
            Content::Choice(choice) => choice.options.iter().any(|doc| doc.has_date_condition()),
            Content::Liturgy(liturgy) => liturgy.body.iter().any(|doc| doc.has_date_condition()),
            Content::CollectOfTheDay { allow_multiple: _ } => true,
            _ => false
        };
        has_own_date_condition || has_child_date_condition
    }

    /// Returns a document stripped of all sub-documents that are [Show::CompiledOnly]
    pub fn into_template(self) -> Option<Self> {
        if self.display == Show::CompiledOnly {
            None
        } else {
			// move out all the subfields, to reconstruct later
            // this prevents us from having to clone all the content, because it's now locally owned

			let Self {
				condition, 
				label, 
				subtitle, 
				language, 
				source, 
                alternate_sources,
				status, 
				display, 
				version, 
				version_label, 
                optional,
				content, 
                explainer,
				is_compiled, 
				tags
			} = self;

            let content = match content {
                Content::Series(content) => Content::Series(Series::from(content.into_vec().into_iter().filter_map(|child| child.into_template()))),
                Content::Parallel(content) => Content::Parallel(Parallel::from(content.into_vec().into_iter().filter_map(|child| child.into_template()))),
                Content::Choice(choice) => Content::Choice(Choice {
                    options: choice.options.into_iter().filter_map(|child| child.into_template()).collect(),
                    ..choice
                }),
                Content::Liturgy(liturgy) => Content::Liturgy(Liturgy {
                    body: Series::from(liturgy.body.into_vec().into_iter().filter_map(|child| child.into_template())),
                    ..liturgy
                }),
                _ => content
            };

			Some(Self {
				condition,
				label,
				subtitle,
				language,
				source,
                alternate_sources,
				status,
				display,
				version,
				version_label,
                optional,
				content,
                explainer,
				is_compiled,
				tags,
			})
        }
    }

    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn as_text(&self) -> String {
        self.content.as_text()
    }

    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn as_metadata_text(&self) -> String {
        self.content.as_metadata_text()
    }

    #[must_use]
    pub fn content(mut self, content: impl Into<Content>) -> Self {
        self.content = content.into();
        self
    }

    #[must_use]
    pub fn explainer(mut self, text: impl Display) -> Self {
        self.explainer = Some(text.to_string());
        self
    }

    #[must_use]
    pub fn label(mut self, label: impl Display) -> Self {
        self.label = Some(label.to_string());
        self
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Display) -> Self {
        self.subtitle = Some(subtitle.to_string());
        self
    }

    #[must_use]
    pub fn condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    #[must_use]
    pub fn display(mut self, show: Show) -> Self {
        self.display = show;
        self
    }

    /// Marks the document's source as being a particular page of the 1979 BCP. Shorthand for `.source(Reference::from(page))`
    #[must_use]
    pub fn page(mut self, page: u16) -> Self {
        self.source = Some(Reference::from(page));
        self
    }

        /// Marks the document's source as being a particular page of the 1979 Libro de Oracíon Común.
    #[must_use]
    pub fn loc_page(mut self, page: u16) -> Self {
        self.source = Some(Reference {
            source: Source::LibroDeOracionComun,
            page
        });
        self
    }

    #[must_use]
    pub fn source(mut self, source: Reference) -> Self {
        self.source = Some(source);
        self
    }

    #[must_use]
    pub fn alternate_source(mut self, source: Reference) -> Self {
        self.alternate_sources.push(source);
        self
    }

    #[must_use]
    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    #[must_use]
    pub fn tags<T>(mut self, tags: impl IntoIterator<Item = T>) -> Self where T: std::fmt::Display {
        self.tags = tags.into_iter().map(|n| n.to_string()).collect();
        self
    }

    #[must_use]
    pub fn language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    #[must_use]
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    #[must_use]
    pub fn version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    #[must_use]
    pub fn version_label(mut self, version_label: impl Display) -> Self {
        self.version_label = Some(version_label.to_string());
        self
    }

    /// Whether any of the content is a parallel
    pub fn contains_parallels(&self) -> bool {
        match &self.content {
            Content::Parallel(_) => true,
            Content::Series(series) => series.iter().any(|child| child.contains_parallels()),
            Content::Choice(choice) => choice.options.iter().any(|child| child.contains_parallels()),
            Content::Liturgy(liturgy) => liturgy.body.iter().any(|child| child.contains_parallels()),
            _ => false
        }
    }

    /// Whether any of the document's fields, or its content, contains the given text
    pub fn contains(&self, text: &str) -> bool {
        let label_contains = self
            .label
            .as_ref()
            .map(|label| label.contains(text))
            .unwrap_or(false);
        let subtitle_contains = self
            .subtitle
            .as_ref()
            .map(|subtitle| subtitle.contains(text))
            .unwrap_or(false);
        let version_label_contains = self
            .version_label
            .as_ref()
            .map(|label| label.contains(text))
            .unwrap_or(false);
        let source_contains = self
            .source
            .map(|reference| reference.page.to_string() == text)
            .unwrap_or(false);
        let tags_contain = self.tags.iter().any(|tag| tag.contains(text));
        let content_contains = self.content.contains(text);
        label_contains || subtitle_contains || version_label_contains || source_contains || tags_contain || content_contains
    }

    /// Whether any of the document's fields, or its content, contains the given text, ignoring the search string case.
    /// An empty `String` returns `true`.
    pub fn contains_case_insensitive(&self, text: &str) -> bool {
        let text = text.to_lowercase();
        let label_contains = self
            .label
            .as_ref()
            .map(|label| label.to_lowercase().contains(&text))
            .unwrap_or(false);
        let subtitle_contains = self
            .subtitle
            .as_ref()
            .map(|subtitle| subtitle.to_lowercase().contains(&text))
            .unwrap_or(false);
        let version_label_contains = self
            .version_label
            .as_ref()
            .map(|label| label.to_lowercase().contains(&text))
            .unwrap_or(false);
        let source_contains = self
            .source
            .map(|reference| reference.page.to_string().to_lowercase() == text)
            .unwrap_or(false);
        let tags_contain = self.tags.iter().any(|tag| tag.to_lowercase().contains(&text));
        let content_contains = self.content.contains_case_insensitive(&text);
        label_contains || subtitle_contains || version_label_contains || source_contains || tags_contain || content_contains
    }

    /// An iterator of all child Documents whose `tags` field contains the given tag
    /// ```
    /// # use library::marriage_alternatives::liturgical_resources_1::WITNESSING_AND_BLESSING_OF_A_MARRIAGE;
    /// # use library::marriage_alternatives::parallels;
    /// # use library::bcp1979::marriage::CELEBRATION_AND_BLESSING_OF_A_MARRIAGE;
    /// let children = CELEBRATION_AND_BLESSING_OF_A_MARRIAGE.children_with_tag(parallels::TITLE.into()).collect::<Vec<_>>();
    /// assert_eq!(children.len(), 1);
    /// let children = WITNESSING_AND_BLESSING_OF_A_MARRIAGE.children_with_tag(parallels::TITLE.into()).collect::<Vec<_>>();
    /// assert_eq!(children.len(), 1);
    /// ```
    pub fn children_with_tag(&self, tag: String) -> impl Iterator<Item = &Document> {
        let matches =  if self.tags.contains(&tag) {
            Box::new(std::iter::once(self)) as Box<dyn Iterator<Item = &Document>>
        } else {
            match &self.content {
                Content::Series(series) => Box::new(series.iter().flat_map(move |doc| doc.children_with_tag(tag.clone()))) as Box<dyn Iterator<Item = &Document>>,
                Content::Liturgy(liturgy) => Box::new(liturgy.body.iter().flat_map(move |doc| doc.children_with_tag(tag.clone()))) as Box<dyn Iterator<Item = &Document>>,
                _ => Box::new(std::iter::empty()) as Box<dyn Iterator<Item = &Document>>
            }
        };

        matches
    }

    /// Provides an iterator over all the serial children of the `Document` without flattening any of the child types.
    pub fn children(&self) -> impl Iterator<Item = &Document> {
        let boxed = match &self.content {
            Content::Series(series) => Box::new(series.iter()) as Box<dyn Iterator<Item = &Document>>,
            Content::Liturgy(liturgy) => Box::new(liturgy.body.iter()) as Box<dyn Iterator<Item = &Document>>,
            _ => Box::new(std::iter::once(self)) as Box<dyn Iterator<Item = &Document>>
        };
        boxed
    }

    /// Reduces any `Document` into an iterator of its smallest parts. For example, a `Text` will be turned into 
    /// a series of `Document`s, each of which represents one paragraph.
    pub fn to_smallest_chunks(self) -> impl Iterator<Item = Document> {
        let content = self.content.clone();
        let iterator = match content {
            Content::Series(series) => {
                if series.is_indivisible() {
                    Box::new(std::iter::once(self)) as Box<dyn Iterator<Item = Document>>
                } else {
                    Box::new(series.into_vec().into_iter().flat_map(|child| child.to_smallest_chunks())) as Box<dyn Iterator<Item = Document>>
                }
            },
            Content::Parallel(parallel) => {
                let chunked_children = parallel.into_vec().into_iter().filter_map(|child| Document::series_or_document(&mut child.to_smallest_chunks())).collect::<Vec<_>>();
                Box::new(std::iter::once(Document {
                    content: Content::Parallel(Parallel::from(chunked_children)),
                    ..self
                })) as Box<dyn Iterator<Item = Document>>
            },
            Content::Choice(choice) => {
                let chunked_options = choice.options.clone().into_iter().filter_map(|child| Document::series_or_document(&mut child.to_smallest_chunks())).collect::<Vec<_>>();
                Box::new(std::iter::once(Document {
                    content: Content::Choice(Choice {
                        options: chunked_options,
                        ..choice
                    }),
                    ..self
                })) as Box<dyn Iterator<Item = Document>>
            },
            Content::Text(text) => {
                // allowed because of lifetime issues if we remove this and try to use the iterator instead
                #[allow(clippy::needless_collect)]
                let paragraphs = text.text.split("\n\n").map(String::from).collect::<Vec<_>>();
                Box::new(paragraphs.into_iter().map(move |paragraph| Document {
                    content: Content::Text(Text {
                        text: paragraph,
                        ..text.clone()
                    }),
                    ..self.clone()
                })) as Box<dyn Iterator<Item = Document>> 
            },
            Content::Rubric(rubric) => {
                // allowed because of lifetime issues if we remove this and try to use the iterator instead
                #[allow(clippy::needless_collect)]
                let paragraphs = rubric.text.split("\n\n").map(String::from).collect::<Vec<_>>();
                Box::new(paragraphs.into_iter().map(move |paragraph| Document {
                    content: Content::Rubric(Rubric {
                        text: paragraph,
                        ..rubric.clone()
                    }),
                    ..self.clone()
                })) as Box<dyn Iterator<Item = Document>> 
            },
            Content::Litany(litany) => {
                let response = litany.response.clone();
                Box::new(litany.into_vec().into_iter().map(move |line| Document {
                    content: Content::Litany(Litany {
                        lines: vec![line],
                        response: response.clone()
                    }),
                    ..self.clone()
                }))
            }
            // TODO add some other chunked types?
            /* Content::Canticle(_) => todo!(),
            Content::CanticleTableEntry(_) => todo!(),
            Content::DocumentLink(_, _, _, _) => todo!(),
            Content::GloriaPatri(_) => todo!(),
            Content::Heading(_) => todo!(),
            Content::HymnLink(_) => todo!(),
            Content::Invitatory(_) => todo!(),
            Content::LectionaryReading(_) => todo!(),
            Content::Litany(_) => todo!(),
            Content::Liturgy(_) => todo!(),
            Content::Preces(_) => todo!(),
            Content::Psalm(_) => todo!(),
            Content::PsalmCitation(_) => todo!(),
            Content::ResponsivePrayer(_) => todo!(),
            Content::Rubric(_) => todo!(),
            Content::Sentence(_) => todo!(), */
            _ => Box::new(std::iter::once(self)) as Box<dyn Iterator<Item = Document>>
        };
        iterator
    }

    pub fn best_label(&self) -> Option<String> {
        if self.label.is_none() {
            match &self.content {
                Content::Canticle(canticle) => Some(format!("{}. {}", canticle.number, canticle.local_name)),
                _ => None
            }
        } else {
            self.label.clone()
        }
    }

    pub fn as_citation(&self) -> Option<String> {
        match &self.content {
            Content::BiblicalCitation(c) => Some(c.citation.clone()),
            Content::BiblicalReading(c) => Some(c.citation.clone()),
            Content::Canticle(c) => Some(c.citation.clone().unwrap_or_else(|| format!("Canticle {}", c.number))),
            Content::HymnLink(h) => match h {
                HymnLink::Hymnal(hymnal) => Some(hymnal.to_string()),
                HymnLink::Hymn(hymnal, number) => Some(format!("{} {}", hymnal, number)),
                _ => None
            },
            Content::Psalm(c) => Some(c.citation.clone().unwrap_or_else(|| format!("Psalm {}", c.number))),
            Content::PsalmCitation(c) => Some(c.0.clone()),
            Content::Sentence(c) => c.citation.clone().map(|c| c),
            _ => None
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Content {
    /// # Structural Variants
    /// A set of multiple [Document]s, organized one after the other
    Series(Series),
    /// A set of multiple [Document]s, displayed as parallel options (e.g., in multiple languages or versions)
    Parallel(Parallel),
    /// A set of multiple [Document]s, which are mutually-exclusive choices
    Choice(Choice),
    /// # Lookup Fields
    /// Inserts the Collect of the Day
    CollectOfTheDay { allow_multiple: bool },
    /// # Content Variants
    /// A document with no contents
    Empty,
    /// An error that comes up while compiling a liturgy.
    Error(DocumentError),
    /// A brief passage or verse, usually extracted from a psalm.
    Antiphon(Antiphon),
    /// A reference to a passage of the Bible, which will be inserted as a
    /// [BibleReading](crate::BibleReading) by the compilation process.
    BiblicalCitation(BiblicalCitation),
    /// A reading that contains the text of a portion of the Bible.
    BiblicalReading(BiblicalReading),
    /// A Canticle (i.e., a psalm-like text not found in the Book of Psalms, and used liturgically)
    Canticle(Canticle),
    /// An entry that can be looked up from a [CanticleTable](canticle_table::CanticleTable).
    CanticleTableEntry(CanticleTableEntry),
    /// A link to another document in the table of contents
    DocumentLink {
        label: String,
        path: SlugPath,
        /// The compiler will show every matching [Document](crate::Document) as a
        /// [Choice](crate::Choice), with the first selected by default. If `true`,
        /// the selection will be rotated deterministically by day.
        rotate: bool,
        /// If `true`, the compiler will not insert the documents, but will retain a link.
        link_only: bool
    },
    /// The Gloria Patri is formatted such that it is broken into four lines rather than two if necessary
    GloriaPatri(GloriaPatri),
    /// A title, subtitle, label, or other heading
    Heading(Heading),
    /// A reference to a [Hymnal](hymnal::Hymnal), [HymnNumber](hymnal::HymnNumber), or [Hymn](hymnal::Hymn) tag.
    HymnLink(HymnLink),
    /// An invitatory psalm
    Invitatory(Invitatory),
    /// A generic reference to a lectionary reading (i.e., “First Reading” from the Daily Office Lectionary).
    LectionaryReading(LectionaryReading),
    /// A responsive prayer in which the same response is given to every petition
    Litany(Litany),
    /// A liturgical template that can carry a set of possible preferences and
    /// other metadata, as well as sub-documents.
    Liturgy(Liturgy),
    /// A responsive prayer in which each line has a label and its text: V: ___ / R: ___
    Preces(Preces),
    /// A psalm.
    Psalm(Psalm),
    /// A reference to a [Psalm](crate::Psalm), which will be inserted by the compilation process.
    PsalmCitation(PsalmCitation),
    /// A simple responsive prayer in which the leader and participants alternate.
    ResponsivePrayer(ResponsivePrayer),
    /// An explanatory sentence or direction for the liturgy
    Rubric(Rubric),
    /// A short Biblical reading, with an optional response.
    Sentence(Sentence),
    /// Text, without any additional styling or semantics
    Text(Text),
}

impl Content {
    /// Whether any of the document's fields, or its content, contains the given text.
    /// This does deep/recursive search on [Choice](crate::Choice), [Parallel](crate::Parallel),
    /// [Series](crate::Series), and [Liturgy](crate::Liturgy).
    pub fn contains(&self, text: &str) -> bool {
        match self {
            Content::Series(docs) => docs.iter().any(|doc| doc.contains(text)),
            Content::Parallel(docs) => docs.iter().any(|doc| doc.contains(text)),
            Content::Choice(docs) => docs.options.iter().any(|doc| doc.contains(text)),
            Content::CollectOfTheDay { allow_multiple: _ } => false,
            Content::Empty => false,
            Content::Error(_) => false,
            Content::Antiphon(antiphon) => antiphon.to_string().contains(text),
            Content::BiblicalCitation(_) => false,
            Content::BiblicalReading(reading) => {
                reading.text.iter().any(|(_, verse)| verse.contains(text))
            }
            Content::Canticle(canticle) => {
                // if any canticle metadata includes it
                canticle.number.to_string().contains(text) || 
                canticle.local_name.contains(text) ||
                canticle.latin_name.as_ref().map(|name| name.contains(text)).unwrap_or(false) ||
                canticle.citation.as_ref().map(|name| name.contains(text)).unwrap_or(false) ||
                // if any section title or text contains it
                canticle.sections.iter().any(|section| {
                    // if section title exists and contains it
                    section
                        .title
                        .as_ref()
                        .map(|title| title.contains(text))
                        .unwrap_or(false)
                        // if any verses contain it
                        || section
                            .verses
                            .iter()
                            .any(|verse| verse.a.contains(text) || verse.b.contains(text))
                })
            }
            Content::CanticleTableEntry(_) => false,
            Content::DocumentLink { .. } => false, 
            Content::GloriaPatri(gloria) => gloria.text.0.contains(text) || gloria.text.1.contains(text) || gloria.text.2.contains(text) || gloria.text.3.contains(text),
            Content::Heading(heading) => match heading {
                Heading::Date(s) => s.contains(text),
                Heading::Day { name, proper, holy_days } => name.contains(text) || proper.as_ref().map(|name| name.contains(text)).unwrap_or(false) || holy_days.as_ref().map(|days| days.iter().any(|(_, day)| day.contains(text))).unwrap_or(false),
                Heading::Text(_, s) => s.contains(text),
                _ => false,
            },
            Content::Invitatory(invitatory) => {
                invitatory.citation.as_ref().map(|citation| citation.contains(text)).unwrap_or(false) || invitatory.sections.iter().any(|section| section.verses.iter().any(|verse| verse.a.contains(text) || verse.b.contains(text)))
            },
            Content::LectionaryReading(_) => false,
            Content::Litany(litany) => litany.response.contains(text) || litany.iter().any(|line| line.contains(text)),
            Content::Liturgy(liturgy) => liturgy.body.iter().any(|doc| doc.contains(text)),
            Content::Preces(preces) => preces.iter().any(|(a, b)| a.contains(text) || b.contains(text)),
            Content::Psalm(psalm) => {
                psalm.number.to_string().contains(text) || psalm.citation.as_ref().map(|citation| citation.contains(text)).unwrap_or(false) || psalm.sections.iter().any(|section| section.verses.iter().any(|verse| verse.number.to_string().contains(text) || verse.a.contains(text) || verse.b.contains(text)))
            },
            Content::PsalmCitation(_) => false,
            Content::ResponsivePrayer(lines) => lines.iter().any(|line| line.contains(text)),
            Content::Rubric(rubric) => rubric.to_string().contains(text),
            Content::Sentence(sentence) => sentence.text.contains(text) || sentence.citation.as_ref().map(|citation| citation.contains(text)).unwrap_or(false),
            Content::Text(t) => t.to_string().contains(text),
            Content::HymnLink(_) => false,
        }
    }

    pub fn contains_case_insensitive(&self, text: &str) -> bool {
        match self {
            Content::Series(docs) => docs.iter().any(|doc| doc.contains_case_insensitive(text)),
            Content::Parallel(docs) => docs.iter().any(|doc| doc.contains_case_insensitive(text)),
            Content::Choice(docs) => docs.options.iter().any(|doc| doc.contains_case_insensitive(text)),
            Content::CollectOfTheDay { allow_multiple: _ } => false,
            Content::Empty => false,
            Content::Error(_) => false,
            Content::Antiphon(antiphon) => antiphon.to_string().to_lowercase().contains(text),
            Content::BiblicalCitation(_) => false,
            Content::BiblicalReading(reading) => {
                reading.text.iter().any(|(_, verse)| verse.to_lowercase().contains(text))
            }
            Content::Canticle(canticle) => {
                // if any canticle metadata includes it
                canticle.number.to_string().to_lowercase().contains(text) || 
                canticle.local_name.to_lowercase().contains(text) ||
                canticle.latin_name.as_ref().map(|name| name.to_lowercase().contains(text)).unwrap_or(false) ||
                canticle.citation.as_ref().map(|name| name.to_lowercase().contains(text)).unwrap_or(false) ||
                // if any section title or text contains it
                canticle.sections.iter().any(|section| {
                    // if section title exists and contains it
                    section
                        .title
                        .as_ref()
                        .map(|title| title.to_lowercase().contains(text))
                        .unwrap_or(false)
                        // if any verses contain it
                        || section
                            .verses
                            .iter()
                            .any(|verse| verse.a.to_lowercase().contains(text) || verse.b.to_lowercase().contains(text))
                })
            }
            Content::CanticleTableEntry(_) => false,
            Content::DocumentLink { .. } => false,
            Content::GloriaPatri(gloria) => gloria.text.0.to_lowercase().contains(text) || gloria.text.1.to_lowercase().contains(text) || gloria.text.2.to_lowercase().contains(text) || gloria.text.3.to_lowercase().contains(text),
            Content::Heading(heading) => match heading {
                Heading::Date(s) => s.to_lowercase().contains(text),
                Heading::Day { name, proper, holy_days } => name.to_lowercase().contains(text) || proper.as_ref().map(|name| name.to_lowercase().contains(text)).unwrap_or(false) || holy_days.as_ref().map(|days| days.iter().any(|(_, day)| day.to_lowercase().contains(text))).unwrap_or(false),
                Heading::Text(_, s) => s.to_lowercase().contains(text),
                _ => false,
            },
            Content::Invitatory(invitatory) => {
                invitatory.citation.as_ref().map(|citation| citation.to_lowercase().contains(text)).unwrap_or(false) || invitatory.sections.iter().any(|section| section.verses.iter().any(|verse| verse.a.to_lowercase().contains(text) || verse.b.to_lowercase().contains(text)))
            },
            Content::LectionaryReading(_) => false,
            Content::Litany(litany) => litany.response.to_lowercase().contains(text) || litany.iter().any(|line| line.to_lowercase().contains(text)),
            Content::Liturgy(liturgy) => liturgy.body.iter().any(|doc| doc.contains_case_insensitive(text)),
            Content::Preces(preces) => preces.iter().any(|(a, b)| a.to_lowercase().contains(text) || b.to_lowercase().contains(text)),
            Content::Psalm(psalm) => {
                psalm.number.to_string().to_lowercase().contains(text) || psalm.citation.as_ref().map(|citation| citation.to_lowercase().contains(text)).unwrap_or(false) || psalm.sections.iter().any(|section| section.verses.iter().any(|verse| verse.number.to_string().to_lowercase().contains(text) || verse.a.to_lowercase().contains(text) || verse.b.to_lowercase().contains(text)))
            },
            Content::PsalmCitation(_) => false,
            Content::ResponsivePrayer(lines) => lines.iter().any(|line| line.to_lowercase().contains(text)),
            Content::Rubric(rubric) => rubric.to_string().to_lowercase().contains(text),
            Content::Sentence(sentence) => sentence.text.to_lowercase().contains(text) || sentence.citation.as_ref().map(|citation| citation.to_lowercase().contains(text)).unwrap_or(false),
            Content::Text(t) => t.to_string().to_lowercase().contains(text),
            Content::HymnLink(_) => false,
        }
    }

    pub fn is_container(&self) -> bool {
        matches!(self, Content::Liturgy(_) | Content::Series(_) | Content::Parallel(_) | Content::Choice(_))
    }

    pub fn is_leaf(&self) -> bool {
        !self.is_container()
    }

    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn as_text(&self) -> String {
        match self {
            Content::Error(c) => c.to_string(),
            Content::Antiphon(c) => c.to_string(),
            Content::BiblicalReading(c) => c.text.iter().map(|(_, text)| text).cloned().intersperse_with(|| String::from(" ")).collect(),
            Content::Canticle(c) => c.sections.iter().flat_map(|section| section.verses.iter().flat_map(|verse| [&verse.a, &verse.b])).cloned().intersperse_with(|| String::from("\n")).collect(),
            Content::GloriaPatri(c) => [&c.text.0, &c.text.1, &c.text.2, &c.text.3].iter().copied().cloned().intersperse_with(|| String::from(" ")).collect(),
            Content::Heading(c) => match c {
                Heading::Date(s) => s.clone(),
                Heading::Day { name, .. } => name.clone(),
                Heading::Text(_, s) => s.clone(),
                _ => String::new()
            },
            Content::Invitatory(c) => c.sections.iter().flat_map(|section| section.verses.iter()).flat_map(|verse| [&verse.a, &verse.b]).cloned().intersperse_with(|| String::from("\n")).collect(),
            Content::Litany(c) => c.lines.iter().intersperse(&c.response).cloned().intersperse_with(|| String::from("\n")).collect(),
            Content::Preces(c) => c.iter().flat_map(|(v, r)| [v, r]).cloned().intersperse_with(|| String::from(" ")).collect(),
            Content::Psalm(c) => c.as_text(),
            Content::ResponsivePrayer(c) => c.iter().cloned().intersperse_with(|| String::from("\n")).collect(),
            Content::Rubric(c) => c.to_string(),
            Content::Sentence(c) => c.text.clone(),
            Content::Text(c) => c.to_string(),
            _ => String::new()
        }
    }

    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn as_metadata_text(&self) -> String {
        match self {
            Content::BiblicalCitation(c) => c.to_string(),
            Content::BiblicalReading(c) => [Some(c.citation.clone()), Some(String::from(" ")), c.intro.clone().map(|n| n.as_document().as_text())].iter().flatten().cloned().collect(),
            Content::Canticle(c) => [Some(format!("Canticle {}", c.number)), Some(String::from(" ")), c.citation.clone(), Some(String::from(" ")), Some(c.local_name.clone()), Some(String::from(" ")), c.latin_name.clone(), Some(String::from(" ")), c.rubric.clone()].iter().chain(c.sections.iter().map(|section| &section.title)).flatten().cloned().collect(),
            Content::Invitatory(c) => [c.citation.clone(), Some(String::from(" ")), Some(c.local_name.clone()), Some(String::from(" ")), c.latin_name.clone(), Some(String::from(" ")), match &c.antiphon {
                SeasonalAntiphon::Antiphon(a) => Some(a.to_string()),
                _ => None
            }].iter().flatten().cloned().collect(),
            Content::Psalm(c) => c.as_metadata_text(),
            Content::PsalmCitation(c) => c.to_string(),
            Content::Sentence(c) => [c.citation.clone(), Some(String::from(" ")), c.response.clone().map(|r| r.as_text())].iter().flatten().cloned().collect(),
            _ => String::new()
        }
    }
}

// Create Document from a Content enum
impl From<Content> for Document {
    fn from(content: Content) -> Self {
        Self::new().content(content)
    }
}

// Plain strings are converted into Text
impl From<&str> for Document {
    fn from(text: &str) -> Self {
        Document::from(Text::from(text))
    }
}

impl From<String> for Document {
    fn from(text: String) -> Self {
        Document::from(Text::from(text))
    }
}

// Create Documents from various content types
impl From<Antiphon> for Document {
    fn from(content: Antiphon) -> Self {
        Self::from(Content::Antiphon(content))
    }
}

impl From<BiblicalCitation> for Document {
    fn from(content: BiblicalCitation) -> Self {
        Self::from(Content::BiblicalCitation(content))
    }
}

impl From<BiblicalReading> for Document {
    fn from(content: BiblicalReading) -> Self {
        Self::from(Content::BiblicalReading(content))
    }
}

impl From<Canticle> for Document {
    fn from(content: Canticle) -> Self {
        Self::from(Content::Canticle(content))
    }
}

impl From<CanticleTableEntry> for Document {
    fn from(content: CanticleTableEntry) -> Self {
        Self::from(Content::CanticleTableEntry(content))
    }
}

impl From<Choice> for Document {
    fn from(content: Choice) -> Self {
        Self::from(Content::Choice(content))
    }
}

impl From<DocumentError> for Document {
    fn from(content: DocumentError) -> Self {
        Self::from(Content::Error(content))
    }
}

impl From<GloriaPatri> for Document {
    fn from(content: GloriaPatri) -> Self {
        Self::from(Content::GloriaPatri(content))
    }
}

impl From<Heading> for Document {
    fn from(content: Heading) -> Self {
        Self::from(Content::Heading(content))
    }
}

impl From<HymnLink> for Document {
    fn from(content: HymnLink) -> Self {
        Self::from(Content::HymnLink(content))
    }
}

impl From<Invitatory> for Document {
    fn from(content: Invitatory) -> Self {
        Self::from(Content::Invitatory(content))
    }
}

impl From<Litany> for Document {
    fn from(content: Litany) -> Self {
        Self::from(Content::Litany(content))
    }
}

impl From<Preces> for Document {
    fn from(content: Preces) -> Self {
        Self::from(Content::Preces(content))
    }
}

impl From<LectionaryReading> for Document {
    fn from(content: LectionaryReading) -> Self {
        Self::from(Content::LectionaryReading(content))
    }
}

impl From<Liturgy> for Document {
    fn from(content: Liturgy) -> Self {
        Self::from(Content::Liturgy(content))
    }
}

impl From<Parallel> for Document {
    fn from(content: Parallel) -> Self {
        Self::from(Content::Parallel(content))
    }
}

impl From<Psalm> for Document {
    fn from(content: Psalm) -> Self {
        Self::from(Content::Psalm(content))
    }
}

impl From<PsalmCitation> for Document {
    fn from(content: PsalmCitation) -> Self {
        Self::from(Content::PsalmCitation(content))
    }
}

impl From<ResponsivePrayer> for Document {
    fn from(content: ResponsivePrayer) -> Self {
        Self::from(Content::ResponsivePrayer(content))
    }
}

impl From<Rubric> for Document {
    fn from(content: Rubric) -> Self {
        Self::from(Content::Rubric(content))
    }
}

impl From<Sentence> for Document {
    fn from(content: Sentence) -> Self {
        Self::from(Content::Sentence(content))
    }
}

impl From<Series> for Document {
    fn from(content: Series) -> Self {
        Self::from(Content::Series(content))
    }
}

impl From<Text> for Document {
    fn from(content: Text) -> Self {
        Self::from(Content::Text(content))
    }
}

impl From<Series> for Content {
	fn from(content: Series) -> Self {
        Self::Series(content)
    }
}

impl From<Parallel> for Content {
	fn from(content: Parallel) -> Self {
        Self::Parallel(content)
    }
}

impl From<Choice> for Content {
	fn from(content: Choice) -> Self {
        Self::Choice(content)
    }
}

impl From<DocumentError> for Content {
	fn from(content: DocumentError) -> Self {
        Self::Error(content)
    }
}

impl From<Antiphon> for Content {
	fn from(content: Antiphon) -> Self {
        Self::Antiphon(content)
    }
}

impl From<BiblicalCitation> for Content {
	fn from(content: BiblicalCitation) -> Self {
        Self::BiblicalCitation(content)
    }
}

impl From<BiblicalReading> for Content {
	fn from(content: BiblicalReading) -> Self {
        Self::BiblicalReading(content)
    }
}

impl From<Canticle> for Content {
	fn from(content: Canticle) -> Self {
        Self::Canticle(content)
    }
}

impl From<CanticleTableEntry> for Content {
	fn from(content: CanticleTableEntry) -> Self {
        Self::CanticleTableEntry(content)
    }
}

impl From<GloriaPatri> for Content {
	fn from(content: GloriaPatri) -> Self {
        Self::GloriaPatri(content)
    }
}

impl From<Heading> for Content {
	fn from(content: Heading) -> Self {
        Self::Heading(content)
    }
}

impl From<HymnLink> for Content {
	fn from(content: HymnLink) -> Self {
        Self::HymnLink(content)
    }
}

impl From<Invitatory> for Content {
	fn from(content: Invitatory) -> Self {
        Self::Invitatory(content)
    }
}

impl From<LectionaryReading> for Content {
	fn from(content: LectionaryReading) -> Self {
        Self::LectionaryReading(content)
    }
}

impl From<Litany> for Content {
	fn from(content: Litany) -> Self {
        Self::Litany(content)
    }
}

impl From<Liturgy> for Content {
	fn from(content: Liturgy) -> Self {
        Self::Liturgy(content)
    }
}

impl From<Preces> for Content {
	fn from(content: Preces) -> Self {
        Self::Preces(content)
    }
}

impl From<Psalm> for Content {
	fn from(content: Psalm) -> Self {
        Self::Psalm(content)
    }
}

impl From<PsalmCitation> for Content {
	fn from(content: PsalmCitation) -> Self {
        Self::PsalmCitation(content)
    }
}

impl From<ResponsivePrayer> for Content {
	fn from(content: ResponsivePrayer) -> Self {
        Self::ResponsivePrayer(content)
    }
}

impl From<Rubric> for Content {
	fn from(content: Rubric) -> Self {
        Self::Rubric(content)
    }
}

impl From<Sentence> for Content {
	fn from(content: Sentence) -> Self {
        Self::Sentence(content)
    }
}

impl From<Text> for Content {
	fn from(content: Text) -> Self {
        Self::Text(content)
    }
}
