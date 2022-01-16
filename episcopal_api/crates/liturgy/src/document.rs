use std::fmt::Display;

use calendar::{Calendar, LiturgicalDay};
use language::Language;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub condition: Option<Condition>,
    pub label: Option<String>,
    pub language: Language,
    pub source: Option<Reference>,
    pub status: Status,
    pub display: Show,
    pub version: Version,
    pub version_label: Option<String>,
    pub content: Content,
    pub is_compiled: bool
}

impl Document {
    pub fn new() -> Self {
        Self {
            condition: None,
            label: None,
            language: Language::default(),
            source: None,
            status: Status::Authorized,
            display: Show::Always,
            version: Version::RiteII,
            version_label: None,
            content: Content::Empty,
            is_compiled: false
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
    /// into a flat iterator of [Document]s
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
    /// assert!(library::rite2::office::COMPLINE.has_date_condition());
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

    pub fn content(mut self, content: Content) -> Self {
        self.content = content;
        self
    }

    pub fn label(mut self, label: impl Display) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn display(mut self, show: Show) -> Self {
        self.display = show;
        self
    }

    /// Marks the document's source as being a particular page of the 1979 BCP. Shorthand for `.source(Reference::from(page))`
    pub fn page(mut self, page: u16) -> Self {
        self.source = Some(Reference::from(page));
        self
    }

    pub fn source(mut self, source: Reference) -> Self {
        self.source = Some(source);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    pub fn version_label(mut self, version_label: impl Display) -> Self {
        self.version_label = Some(version_label.to_string());
        self
    }

    /// Whether any of the document's fields, or its content, contains the given text
    pub fn contains(&self, text: &str) -> bool {
        let label_contains = self
            .label
            .as_ref()
            .map(|label| label.contains(text))
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
        let content_contains = self.content.contains(text);
        label_contains || version_label_contains || source_contains || content_contains
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
    /// Inserts all documents filed under this category in the library.
    Category(Category),
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
    /// The Gloria Patri is formatted such that it is broken into four lines rather than two if necessary
    GloriaPatri(GloriaPatri),
    /// A title, subtitle, label, or other heading
    Heading(Heading),
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
            Content::Category(_) => false,
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
            Content::GloriaPatri(gloria) => gloria.text.0.contains(text) || gloria.text.1.contains(text) || gloria.text.2.contains(text) || gloria.text.3.contains(text),
            Content::Heading(heading) => match heading {
                Heading::Date(s) => s.contains(text),
                Heading::Day { name, proper, holy_days } => name.contains(text) || proper.as_ref().map(|name| name.contains(text)).unwrap_or(false) || holy_days.as_ref().map(|days| days.iter().any(|day| day.contains(text))).unwrap_or(false),
                Heading::Text(_, s) => s.contains(text),
                _ => false,
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
            Content::Sentence(sentence) => sentence.text.contains(text),
            Content::Text(t) => t.to_string().contains(text),
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

impl From<Category> for Document {
    fn from(content: Category) -> Self {
        Self::from(Content::Category(content))
    }
}

impl From<Categories> for Document {
    fn from(category: Categories) -> Self {
        Self::from(Category::from(category))
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
