use std::io::{Seek, Write};
use thiserror::Error;

use docx_rs::{BreakType, Docx, Header, Paragraph, Run};
use liturgy::*;

pub struct DocxDocument(Docx);

#[derive(Error, Debug)]
pub enum DocxError {
    #[error("error writing DOCX file")]
    Write,
}

impl DocxDocument {
    pub fn write<W>(&mut self, w: W) -> Result<(), DocxError>
    where
        W: Write + Seek,
    {
        self.0.build().pack(w).map_err(|_| DocxError::Write)
    }
}

impl From<Document> for DocxDocument {
    fn from(doc: Document) -> Self {
        let docx = Docx::new();
        Self(add_content(docx, &doc))
    }
}

fn add_content(docx: Docx, doc: &Document) -> Docx {
    match &doc.content {
        Content::Liturgy(liturgy) => liturgy.body.iter().fold(docx, add_content),
        Content::Series(series) => series.iter().fold(docx, add_content),
        Content::Parallel(_parallel) => docx.add_paragraph(paragraph_with_text("[TODO parallel]")),
        Content::Choice(choice) => {
            if let Some(selected_doc) = choice.options.get(choice.selected) {
                add_content(docx, selected_doc)
            } else {
                docx
            }
        }
        Content::Category(content) => content.add_to_docx(docx),
        Content::CollectOfTheDay { allow_multiple: _ } => {
            docx.header(Header::new().add_paragraph(paragraph_with_text("The Collect of the Day")))
        }
        Content::Empty => docx,
        Content::Error(content) => content.add_to_docx(docx),
        Content::Antiphon(content) => content.add_to_docx(docx),
        Content::BiblicalCitation(content) => content.add_to_docx(docx),
        Content::BiblicalReading(content) => content.add_to_docx(docx),
        Content::Canticle(content) => content.add_to_docx(docx),
        Content::CanticleTableEntry(content) => content.add_to_docx(docx),
        Content::GloriaPatri(content) => content.add_to_docx(docx),
        Content::Heading(content) => content.add_to_docx(docx),
        Content::Invitatory(content) => content.add_to_docx(docx),
        Content::LectionaryReading(content) => content.add_to_docx(docx),
        Content::Litany(content) => content.add_to_docx(docx),
        Content::Preces(content) => content.add_to_docx(docx),
        Content::Psalm(content) => content.add_to_docx(docx),
        Content::PsalmCitation(content) => content.add_to_docx(docx),
        Content::ResponsivePrayer(content) => content.add_to_docx(docx),
        Content::Rubric(content) => content.add_to_docx(docx),
        Content::Sentence(content) => content.add_to_docx(docx),
        Content::Text(content) => content.add_to_docx(docx),
        Content::HymnLink(_) => docx,
    }
}

fn paragraph_with_text(text: impl std::fmt::Display) -> Paragraph {
    let para = Paragraph::new();
    let text = text.to_string();
    let lines = text.split('\n').collect::<Vec<_>>();
    let lines_len = lines.len();

    lines.iter().enumerate().fold(para, |para, (idx, text)| {
        let run = Run::new().add_text(*text);
        if idx == lines_len - 1 {
            para.add_run(run)
        } else {
            para.add_run(run.add_break(BreakType::TextWrapping))
        }
    })
}

trait ToParagraph {
    fn to_paragraph(&self) -> Paragraph;
}

trait AddToDocx {
    fn add_to_docx(&self, docx: Docx) -> Docx;
}

impl AddToDocx for Category {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx
    }
}

impl AddToDocx for DocumentError {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self).style("Error"))
    }
}

impl AddToDocx for Antiphon {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self).style("Antiphon"))
    }
}

impl AddToDocx for BiblicalCitation {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self))
    }
}

impl AddToDocx for BiblicalReading {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        // Add intro
        let docx = if let Some(intro) = &self.intro {
            let doc = Document::from(intro.clone());
            add_content(docx, &doc)
        } else {
            docx
        };

        let docx = docx.add_paragraph(paragraph_with_text(&self.citation).style("Heading 3"));

        let para = Paragraph::new();
        let para = self.text.iter().fold(para, |para, (_verse_number, text)| {
            para.add_run(Run::new().add_text(text))
        });

        docx.add_paragraph(para)
    }
}

impl AddToDocx for Canticle {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text("TODO"))
    }
}

impl AddToDocx for CanticleTableEntry {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx
    }
}

impl AddToDocx for GloriaPatri {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let (a, b, c, d) = &self.text;
        docx.add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .add_text(a)
                    .add_text(b)
                    .add_break(BreakType::TextWrapping)
                    .add_tab()
                    .add_text(c)
                    .add_text(d),
            ),
        )
    }
}

impl AddToDocx for Heading {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        match self {
            Heading::InsertDate => docx,
            Heading::InsertDay => docx,
            Heading::Date(text) => docx.add_paragraph(paragraph_with_text(text).style("Date")),
            Heading::Day {
                name,
                proper,
                holy_days,
            } => {
                let docx = docx.add_paragraph(paragraph_with_text(name).style("Day"));
                let docx = if let Some(proper) = proper {
                    docx.add_paragraph(paragraph_with_text(proper).style("Day"))
                } else {
                    docx
                };
                if let Some(holy_days) = holy_days {
                    let para = Paragraph::new();
                    let holy_days_len = holy_days.len();
                    let para =
                        holy_days
                            .iter()
                            .enumerate()
                            .fold(para, |para, (idx, (_, holy_day))| {
                                para.add_run(if idx == holy_days_len - 1 {
                                    Run::new().add_text(holy_day)
                                } else {
                                    Run::new()
                                        .add_text(holy_day)
                                        .add_break(BreakType::TextWrapping)
                                })
                            });
                    docx.add_paragraph(para)
                } else {
                    docx
                }
            }
            Heading::Text(level, text) => {
                docx.add_paragraph(paragraph_with_text(text).style(&format!(
                    "Heading{}",
                    match level {
                        HeadingLevel::Heading1 => 1,
                        HeadingLevel::Heading2 => 2,
                        HeadingLevel::Heading3 => 3,
                        HeadingLevel::Heading4 => 4,
                        HeadingLevel::Heading5 => 5,
                    }
                )))
            }
        }
    }
}

impl AddToDocx for Invitatory {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text("TODO"))
    }
}

impl AddToDocx for LectionaryReading {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text("TODO"))
    }
}

impl AddToDocx for Litany {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        self.iter().fold(docx, |docx, line| {
            docx.add_paragraph(
                Paragraph::new().add_run(Run::new().add_text(line)).add_run(
                    Run::new()
                        .add_text(self.response.clone())
                        .bold()
                        .add_break(BreakType::TextWrapping),
                ),
            )
        })
    }
}

impl AddToDocx for Preces {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let para = Paragraph::new();
        let para = self.iter().enumerate().fold(para, |para, (idx, (v, r))| {
            let r = if idx % 2 == 1 {
                Run::new()
                    .add_text(r)
                    .bold()
                    .add_break(BreakType::TextWrapping)
            } else {
                Run::new().add_text(r).add_break(BreakType::TextWrapping)
            };
            para.add_run(Run::new().add_text(v).italic().add_tab())
                .add_run(r)
        });
        docx.add_paragraph(para)
    }
}

impl AddToDocx for Psalm {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text("TODO"))
    }
}

impl AddToDocx for PsalmCitation {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text("TODO"))
    }
}

impl AddToDocx for ResponsivePrayer {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text("TODO"))
    }
}

impl AddToDocx for Rubric {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self).style("Rubric"))
    }
}

impl AddToDocx for Sentence {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text("TODO"))
    }
}

impl AddToDocx for Text {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let para = paragraph_with_text(&self.text);

        let para = if let Some(response) = &self.response {
            para.add_run(Run::new().add_text(" "))
                .add_run(Run::new().add_text(response).bold())
        } else {
            para
        };

        docx.add_paragraph(para)
    }
}
