use crate::{Content, Document, GlobalPref, Lectionaries, Preces, ResponsivePrayer, Text};
use lectionary::ReadingType;
use reference_parser::{BibleReference, Book};
use serde::{Deserialize, Serialize};

use crate::PreferenceKey;

/// A generic reference to a lectionary reading (i.e., “First Reading” from the Daily Office Lectionary).
/// The [Library](library::Library) will compile this into a [BiblicalReading](crate::BiblicalReading).
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct LectionaryReading {
    pub reading_type: ReadingTypeTable,
    pub reading_type_overridden_by: Option<ReadingType>,
    pub lectionary: LectionaryTableChoice,
    pub intro: Option<BiblicalReadingIntroTemplate>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReadingTypeTable {
    /// Dynamically the lectionary selected in the specified preference
    Preference(PreferenceKey),
    /// Statically uses the chosen lectionary
    Selected(ReadingType),
}

impl Default for ReadingTypeTable {
    fn default() -> Self {
        Self::Selected(ReadingType::FirstReading)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum LectionaryTableChoice {
    /// Dynamically loads the lectionary selected in the specified preference
    Preference(PreferenceKey),
    /// Statically uses the chosen lectionary
    Selected(Lectionaries),
}

impl Default for LectionaryTableChoice {
    fn default() -> Self {
        Self::Preference(PreferenceKey::Global(GlobalPref::Lectionary))
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
/// Template for the introduction to a Biblical reading, which needs to be compiled
/// to replace {{short_name}} or {{long_name}} with the name of the relevant [Book](reference_parser::Book).
pub struct BiblicalReadingIntroTemplate(Box<Document>);

impl From<Document> for BiblicalReadingIntroTemplate {
    fn from(document: Document) -> Self {
        Self(Box::new(document))
    }
}

impl From<BiblicalReadingIntroTemplate> for Document {
    fn from(template: BiblicalReadingIntroTemplate) -> Self {
        *template.0
    }
}

impl From<Text> for BiblicalReadingIntroTemplate {
    fn from(content: Text) -> Self {
        Self::from(Document::from(content))
    }
}

impl BiblicalReadingIntroTemplate {
    /// Replaces {{short_name}} or {{long_name}} in the template with the name of the relevant [Book](reference_parser::Book).
    /// ```
    /// # use crate::liturgy::{BiblicalReading, BiblicalReadingIntroTemplate, Document, Preces, Text};
    /// let intro = BiblicalReadingIntroTemplate::from(Document::from(Text::from("A Reading from {{short_name}}.")));
    /// assert_eq!(
    ///     intro.compile("Ecclus. 1:1-14"),
    ///     Document::from(Text::from("A Reading from Sirach."))
    /// );
    /// let intro = BiblicalReadingIntroTemplate::from(Document::from(Text::from("A Reading from {{short_name}}.")));
    /// assert_eq!(
    ///     intro.compile("Mark 1:1-14"),
    ///     Document::from(Text::from("A Reading from Mark."))
    /// );
    ///
    /// let intro = BiblicalReadingIntroTemplate::from(Document::from(Text::from("A Reading from {{long_name}}.")));
    /// assert_eq!(
    ///     intro.compile("Mark 1:1-14"),
    ///     Document::from(Text::from("A Reading from the Gospel According to Mark."))
    /// );
    ///
    /// let intro = BiblicalReadingIntroTemplate::from(Document::from(Preces::from([
    ///     (
    ///         "Celebrant",
    ///         "The Holy Gospel of our Lord Jesus Christ according to {{short_name}}.",
    ///     ),
    ///     ("People", "Glory to you, Lord Christ."),
    /// ])));
    /// assert_eq!(
    ///     intro.compile("Mark 1:1-14"),
    ///     Document::from(Preces::from([
    ///         (
    ///             "Celebrant",
    ///             "The Holy Gospel of our Lord Jesus Christ according to Mark."
    ///         ),
    ///         ("People", "Glory to you, Lord Christ.")
    ///     ]))
    /// );
    /// ```
    #[cfg(any(feature = "browser", feature = "server"))]
    pub fn compile(&self, citation: &str) -> Document {
        let template = *self.0.clone();
        let citation = BibleReference::from(citation);
        let book = citation
            .ranges
            .get(0)
            .and_then(|range| range.start.book)
            .unwrap_or(Book::None);
        let short_name = book.book_short_name(template.language);
        let long_name = book.book_long_name(template.language);

        fn replace_names(base: &str, short_name: &str, long_name: &str) -> String {
            base.replace("{{short_name}}", short_name)
                .replace("{{long_name}}", long_name)
                // replace internal "The" (i.e., "A Reading from The Gospel" => "A Reading from the Gospel")
                .replace(" The", " the")
        }

        match &template.content {
            Content::Preces(content) => Document {
                content: Content::Preces(Preces::from(
                    content
                        .iter()
                        .map(|(label, text)| (label, replace_names(text, short_name, long_name))),
                )),
                ..template
            },
            Content::ResponsivePrayer(content) => Document {
                content: Content::ResponsivePrayer(ResponsivePrayer::from(
                    content
                        .iter()
                        .map(|text| replace_names(text, short_name, long_name)),
                )),
                ..template
            },
            Content::Text(content) => Document {
                content: Content::Text(Text {
                    text: replace_names(&content.text, short_name, long_name),
                    ..content.clone()
                }),
                ..template
            },
            _ => template,
        }
    }
}
