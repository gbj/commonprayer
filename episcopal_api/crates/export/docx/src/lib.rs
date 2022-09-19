use std::io::{Seek, Write};
use thiserror::Error;

use docx_rs::{
    AlignmentType, BreakType, DocumentChild, Docx, PageMargin, Paragraph, Run, Table, TableCell,
    TableRow,
};
use liturgy::*;

mod styles;
pub use styles::*;

pub struct DocxDocument(Docx);

// Docx uses "TWIPS" (twentieth of a point, where a point = 1/72 of an inch) as its basic measure
const ONE_INCH: u32 = 72 * 20;
const HALF_INCH: u32 = 36 * 20;

#[derive(Error, Debug)]
pub enum DocxError {
    #[error("error writing DOCX file")]
    Write,
}

impl DocxDocument {
    pub fn write<W>(self, w: W) -> Result<(), DocxError>
    where
        W: Write + Seek,
    {
        self.0.build().pack(w).map_err(|_| DocxError::Write)
    }
}

impl From<Document> for DocxDocument {
    fn from(doc: Document) -> Self {
        let docx = Docx::new()
            .inject_styles()
            .page_size(17 * HALF_INCH, 11 * ONE_INCH)
            .page_margin(
                PageMargin::new()
                    .top(ONE_INCH.try_into().unwrap())
                    .left(ONE_INCH.try_into().unwrap())
                    .bottom(ONE_INCH.try_into().unwrap())
                    .right(ONE_INCH.try_into().unwrap()),
            );

        Self(add_content(docx, &doc))
    }
}

impl DocxDocument {
    pub fn new() -> Self {
        Self(
            Docx::new()
                .inject_styles()
                .page_size(17 * HALF_INCH, 11 * ONE_INCH)
                .page_margin(
                    PageMargin::new()
                        .top(ONE_INCH.try_into().unwrap())
                        .left(ONE_INCH.try_into().unwrap())
                        .bottom(ONE_INCH.try_into().unwrap())
                        .right(ONE_INCH.try_into().unwrap()),
                ),
        )
    }

    #[must_use]
    pub fn add_content(self, doc: &Document) -> Self {
        Self(add_content(self.0, doc))
    }
}

impl Default for DocxDocument {
    fn default() -> Self {
        Self::new()
    }
}

fn add_content(docx: Docx, doc: &Document) -> Docx {
    match &doc.content {
        Content::Liturgy(liturgy) => liturgy.body.iter().fold(docx, add_content),
        Content::Series(series) => series.iter().fold(docx, add_content),
        Content::Parallel(parallel) => docx.add_table(Table::without_borders(vec![TableRow::new(
            parallel
                .iter()
                .map(|child| {
                    let fake_docx = add_content(Docx::new(), child);
                    fake_docx
                        .document
                        .children
                        .into_iter()
                        .fold(TableCell::new(), |cell, child| {
                            if let DocumentChild::Paragraph(paragraph) = child {
                                cell.add_paragraph(*paragraph)
                            } else {
                                cell
                            }
                        })
                })
                .collect(),
        )])),
        Content::Choice(choice) => {
            if let Some(selected_doc) = choice.options.get(choice.selected) {
                add_content(docx, selected_doc)
            } else {
                docx
            }
        }
        Content::CollectOfTheDay { allow_multiple: _ } => {
            docx.add_paragraph(paragraph_with_text("The Collect of the Day").style(HEADING_3))
        }
        Content::DocumentLink { .. } => docx,
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
        #[cfg(any(feature = "server", feature = "browser"))]
        Content::Psalm(content) => content.add_to_docx(docx),
        Content::PsalmCitation(content) => content.add_to_docx(docx),
        Content::ResponsivePrayer(content) => content.add_to_docx(docx),
        Content::Rubric(content) => content.add_to_docx(docx),
        Content::Sentence(content) => content.add_to_docx(docx),
        Content::Text(content) => content.add_to_docx(docx),
        _ => docx,
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
trait AddToDocx {
    fn add_to_docx(&self, docx: Docx) -> Docx;
}

impl AddToDocx for DocumentError {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self).style(ERROR))
    }
}

impl AddToDocx for Antiphon {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self).style(ANTIPHON))
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

        let docx = docx.add_paragraph(paragraph_with_text(&self.citation).style(HEADING_3));

        let para = Paragraph::new();
        let para = self.text.iter().fold(para, |para, (_verse_number, text)| {
            para.add_run(Run::new().add_text(text))
        });

        docx.add_paragraph(para)
    }
}

impl AddToDocx for Canticle {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let header = Paragraph::new()
            .add_run(
                Run::new()
                    .add_text(format!("{}\t", self.number))
                    .size(36)
                    .bold(),
            )
            .add_run(Run::new().add_text(format!("{}\t", self.local_name)).bold())
            .add_run(
                Run::new()
                    .add_text(self.latin_name.clone().unwrap_or_default())
                    .italic()
                    .add_break(BreakType::TextWrapping),
            );

        let header = if let Some(citation) = &self.citation {
            header.add_run(Run::new().add_text(format!("\t{}", citation)))
        } else {
            header
        };

        let docx = docx.add_paragraph(header);

        self.sections.iter().fold(docx, |docx, section| {
            let paragraph = section.verses.iter().fold(
                Paragraph::new().style(PSALM_OR_CANTICLE),
                |para, verse| {
                    verse
                        .a
                        .split('\n')
                        .map(String::from)
                        .chain(verse.b.split('\n').map(|b| format!("\t{b}")))
                        .fold(para, |para, line| {
                            para.add_run(
                                Run::new().add_text(line).add_break(BreakType::TextWrapping),
                            )
                        })
                },
            );
            docx.add_paragraph(paragraph)
        })
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
                    .add_text(" *")
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
            Heading::Date(text) => docx.add_paragraph(paragraph_with_text(text).style(DATE)),
            Heading::Day {
                name,
                proper,
                holy_days,
            } => {
                let docx = docx.add_paragraph(paragraph_with_text(name).style(DAY));
                let docx = if let Some(proper) = proper {
                    docx.add_paragraph(paragraph_with_text(proper).style(DAY))
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
                docx.add_paragraph(paragraph_with_text(text).style(match level {
                    HeadingLevel::Heading1 => HEADING_1,
                    HeadingLevel::Heading2 => HEADING_2,
                    HeadingLevel::Heading3 => HEADING_3,
                    HeadingLevel::Heading4 => HEADING_4,
                    HeadingLevel::Heading5 => HEADING_5,
                }))
            }
        }
    }
}

impl AddToDocx for Invitatory {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let header = Paragraph::new()
            .add_run(Run::new().add_text(format!("{}\t", self.local_name)).bold())
            .add_run(
                Run::new()
                    .add_text(self.latin_name.clone().unwrap_or_default())
                    .italic(),
            );

        let header = if let Some(citation) = &self.citation {
            header.add_run(Run::new().add_text(format!("\t{}", citation)).italic())
        } else {
            header
        };

        let docx = docx.add_paragraph(header);

        self.sections.iter().fold(docx, |docx, section| {
            let paragraph = section.verses.iter().fold(
                Paragraph::new().style(PSALM_OR_CANTICLE),
                |para, verse| {
                    verse
                        .a
                        .split('\n')
                        .map(String::from)
                        .chain(verse.b.split('\n').map(|b| format!("\t{b}")))
                        .fold(para, |para, line| {
                            para.add_run(
                                Run::new().add_text(line).add_break(BreakType::TextWrapping),
                            )
                        })
                },
            );
            docx.add_paragraph(paragraph)
        })
    }
}

impl AddToDocx for LectionaryReading {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx
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
        let table = Table::without_borders(
            self.iter()
                .enumerate()
                .map(|(idx, (v, r))| {
                    TableRow::new(vec![
                        TableCell::new().add_paragraph(
                            Paragraph::new().add_run(Run::new().add_text(v).italic()),
                        ),
                        if idx % 2 == 1 {
                            TableCell::new().add_paragraph(paragraph_with_text(r).style(RESPONSE))
                        } else {
                            TableCell::new().add_paragraph(paragraph_with_text(r))
                        },
                    ])
                })
                .collect(),
        );

        docx.add_table(table)
    }
}

#[cfg(any(feature = "server", feature = "browser"))]
impl AddToDocx for Psalm {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let header = Paragraph::new().add_run(
            Run::new()
                .add_text(format!("{}\t", self.number))
                .size(36)
                .bold(),
        );

        let docx = docx.add_paragraph(header);

        self.filtered_sections().iter().fold(docx, |docx, section| {
            let paragraph = section.verses.iter().fold(
                // TODO add psalm local name/Latin name as well
                Paragraph::new().style(PSALM_OR_CANTICLE),
                |para, verse| {
                    verse
                        .a
                        .split('\n')
                        .map(String::from)
                        .chain(verse.b.split('\n').map(|b| format!("\t{b}")))
                        .fold(para, |para, line| {
                            para.add_run(
                                Run::new().add_text(line).add_break(BreakType::TextWrapping),
                            )
                        })
                },
            );
            docx.add_paragraph(paragraph)
        })
    }
}

impl AddToDocx for PsalmCitation {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self))
    }
}

impl AddToDocx for ResponsivePrayer {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let paragraph = self
            .iter()
            .enumerate()
            .fold(Paragraph::new(), |paragraph, (idx, line)| {
                if idx % 2 == 1 {
                    line.split('\n').fold(paragraph, |paragraph, line| {
                        paragraph.add_run(
                            Run::new()
                                .add_text(line)
                                .bold()
                                .add_break(BreakType::TextWrapping),
                        )
                    })
                } else {
                    line.split('\n').fold(paragraph, |paragraph, line| {
                        paragraph
                            .add_run(Run::new().add_text(line).add_break(BreakType::TextWrapping))
                    })
                }
            });
        docx.add_paragraph(paragraph)
    }
}

impl AddToDocx for Rubric {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        docx.add_paragraph(paragraph_with_text(self).style(RUBRIC))
    }
}

impl AddToDocx for Sentence {
    fn add_to_docx(&self, docx: Docx) -> Docx {
        let text_paragraph = paragraph_with_text(&self.text);

        let docx = if let Some(response) = &self.response {
            if let Content::Text(text) = &response.content {
                if text.text.len() < 10 {
                    docx.add_paragraph(
                        text_paragraph
                            .add_run(Run::new().add_text(" ").add_text(&text.text).bold()),
                    )
                } else {
                    add_content(docx.add_paragraph(text_paragraph), response)
                }
            } else {
                add_content(docx.add_paragraph(text_paragraph), response)
            }
        } else {
            docx.add_paragraph(text_paragraph)
        };

        if let Some(citation) = &self.citation {
            docx.add_paragraph(
                paragraph_with_text(citation)
                    .italic()
                    .align(AlignmentType::Right),
            )
        } else {
            docx
        }
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
