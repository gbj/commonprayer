#![feature(iter_intersperse)]

use language::Language;
use liturgy::*;
use status::Status;

pub trait ToRustCode {
    fn to_rust_code(&self, start_tabs: usize) -> String;
}

impl ToRustCode for Document {
    fn to_rust_code(&self, start_tabs: usize) -> String {
        // TODO condition
        let condition = String::new();

        // Metadata
        let label = self
            .label
            .as_ref()
            .map(|value| format!("\t.label({:?})", value))
            .unwrap_or_default();
        let version_label = self
            .version_label
            .as_ref()
            .map(|value| format!("\t.version_label({:?})", value))
            .unwrap_or_default();
        let explainer = self
            .explainer
            .as_ref()
            .map(|value| format!("\t.explainer({:?})", value))
            .unwrap_or_default();
        let subtitle = self
            .subtitle
            .as_ref()
            .map(|value| format!("\t.subtitle({:?})", value))
            .unwrap_or_default();
        let language = if Language::is_default(&self.language) {
            String::default()
        } else {
            format!("\t.language(Language::{})", self.language)
        };
        let source = self
            .source
            .map(|reference| {
                if reference.source == Source::BCP1979 {
                    format!("\t.page({})", reference.page)
                } else {
                    format!(
                        "\t.source(Reference {{
                source: Source::{},
                page: {}
            }})",
                        reference.source, reference.page
                    )
                }
            })
            .unwrap_or_default();
        let alternate_sources = if self.alternate_sources.is_empty() {
            String::new()
        } else {
            self.alternate_sources
                .iter()
                .map(|reference| {
                    format!(
                        "\t.alternate_source(Reference {{
                source: Source::{},
                page: {}
            }})",
                        reference.source, reference.page
                    )
                })
                .intersperse_with(|| String::from("\n"))
                .collect()
        };
        let status = if Status::is_default(&self.status) {
            String::default()
        } else {
            format!("\t.status(Status::{})", self.status)
        };
        let display = if Show::is_default(&self.display) {
            String::default()
        } else {
            format!("\t.display(Show::{})", self.display)
        };
        let version = if Version::is_default(&self.version) {
            String::default()
        } else {
            format!("\t.version(Version::{})", self.version)
        };
        let optional = if self.optional { "\t.optional()" } else { "" }.to_string();
        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            let tags: String = self
                .tags
                .iter()
                .map(|tag| format!("{:?}", tag))
                .intersperse_with(|| ", ".to_string())
                .collect();
            format!("\t.tags([{tags}])")
        };

        let metadata: String = [
            label,
            version_label,
            explainer,
            subtitle,
            language,
            source,
            alternate_sources,
            status,
            display,
            version,
            optional,
            tags,
        ]
        .into_iter()
        .filter(|n| !n.is_empty())
        .intersperse_with(|| "\n".to_string())
        .collect();

        let content = self.content.to_rust_code(start_tabs + 1);
        let tabs = (0..start_tabs).map(|_| '\t').collect::<String>();

        if metadata.is_empty() {
            format!("{tabs}Document::from({content})")
        } else {
            format!("{tabs}Document::new(){condition}{metadata}\n\t.content({content})")
        }
        .replace('\n', &format!("\n{tabs}"))
    }
}

impl ToRustCode for Content {
    fn to_rust_code(&self, start_tabs: usize) -> String {
        match &self {
            Content::Liturgy(content) => {
                let children: String = content
                    .body
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                format!("Liturgy::from(vec![\n{children}\n])")
            }
            Content::Series(content) => {
                let children: String = content
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                format!("Series::from(vec![\n{children}\n])")
            }
            Content::Choice(content) => {
                let children: String = content
                    .options
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                let selected = if content.selected == 0 {
                    String::new()
                } else {
                    format!(".selected({})", content.selected)
                };
                let should_rotate = if content.should_rotate {
                    ".should_rotate()"
                } else {
                    ""
                };
                format!("Choice::from(vec![\n{children}\n]){selected}{should_rotate}")
            }
            Content::Parallel(content) => {
                let children: String = content
                    .iter()
                    .map(|doc| doc.to_rust_code(start_tabs + 1))
                    .intersperse_with(|| String::from(",\n"))
                    .collect();
                format!("Parallel::from(vec![\n{children}\n])")
            }
            Content::Text(content) => {
                let response = if let Some(response) = &content.response {
                    format!(r#".response("{response}")"#)
                } else {
                    String::new()
                };
                let display_format = if content.display_format.is_default() {
                    String::new()
                } else {
                    format!(".display_format(DisplayFormat::{})", content.display_format)
                };
                format!(
                    r#"Text::from({:?}){}{}"#,
                    content.text, response, display_format
                )
            }
            Content::Heading(content) => match content {
                Heading::InsertDate => "Heading::InsertDate".to_string(),
                Heading::InsertDay => "Heading::InsertDay".to_string(),
                Heading::Date(s) => format!("Heading::Date({:?})", s),
                Heading::Day {
                    name,
                    proper,
                    holy_days,
                } => format!(
                    "Heading::Day {{ name: {:?}, proper: {:?}, holy_days: {:?} }}",
                    name, proper, holy_days
                ),
                Heading::Text(level, text) => {
                    format!("Heading::from((HeadingLevel::{}, {:?}))", level, text)
                }
            },
            Content::Antiphon(content) => format!("Antiphon::from({:?})", content),
            Content::Litany(content) => {
                let children = content
                    .iter()
                    .map(|line| format!("{:?}", line))
                    .intersperse_with(|| ",\n\t\t".to_string())
                    .collect::<String>();
                format!(
                    "Litany::from((\n\t{:?},\n\t[\n\t\t{}\n\t]\n))",
                    content.response, children
                )
            }
            Content::ResponsivePrayer(content) => {
                let children = content
                    .iter()
                    .map(|line| format!("{:?}", line))
                    .intersperse_with(|| ",\n\t".to_string())
                    .collect::<String>();
                format!("ResponsivePrayer::from([\n\t{}\n])", children)
            }
            Content::Rubric(content) => {
                let long = if content.long { ".long()" } else { "" };
                format!("Rubric::from({:?}){}", content.text, long)
            }
            Content::Category(content) => {
                let rotate = if content.rotate { ".rotate()" } else { "" };
                format!("Category::from(Categories::{}){}", content.name, rotate)
            }
            Content::CollectOfTheDay { allow_multiple } => format!(
                "Content::CollectOfTheDay {{ allow_multiple: {} }}",
                allow_multiple
            ),
            Content::Empty => "Content::Empty".to_string(),
            Content::Error(content) => {
                format!("Content::Error(DocumentError::from({:?}))", content)
            }
            Content::BiblicalCitation(content) => {
                let intro = content
                    .intro
                    .as_ref()
                    .map(|intro| {
                        let doc: Document = intro.clone().into();
                        format!(
                            "BiblicalReadingIntro::from({})",
                            doc.to_rust_code(start_tabs)
                        )
                    })
                    .unwrap_or_default();
                format!("BiblicalCitation::from({:?}){}", content.citation, intro)
            }
            Content::BiblicalReading(_) => String::new(),
            Content::Canticle(_) => String::new(),
            Content::CanticleTableEntry(_) => String::new(),
            Content::DocumentLink(_, _, _, _) => String::new(),
            Content::GloriaPatri(content) => {
                let display_format = if content.display_format.is_default() {
                    String::new()
                } else {
                    format!(".display_format(DisplayFormat::{})", content.display_format)
                };
                format!(
                    "GloriaPatri::from({:?}, {:?}, {:?}, {:?}){}",
                    content.text.0, content.text.1, content.text.2, content.text.3, display_format
                )
            }
            Content::HymnLink(_) => String::new(),
            Content::Invitatory(_) => String::new(),
            Content::LectionaryReading(_) => String::new(),
            Content::Preces(content) => {
                let children = content
                    .iter()
                    .map(|(v, r)| format!("({:?}, {:?})", v, r))
                    .intersperse_with(|| ",\n\t".to_string())
                    .collect::<String>();
                format!("Preces::from([\n\t{}\n])", children)
            }
            Content::Psalm(_) => String::new(),
            Content::PsalmCitation(_) => String::new(),
            Content::Sentence(content) => {
                let citation = content
                    .citation
                    .as_ref()
                    .map(|citation| format!(".citation({:?})", citation))
                    .unwrap_or_default();
                format!("Sentence::from({:?}){}", content.text, citation)
            }
        }
    }
}
