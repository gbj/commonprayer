use liturgy::*;
use sauron::html::attributes::inner_html;
use sauron::html::text;
use sauron::*;

use crate::{
    lookup::{lookup_links, LookupType},
    Msg,
};

pub struct DocumentComponent {
    pub document: Document,
    pub dynamic: bool, // whether this is the server side version (dynamic = false) or the client (dynamic = true)
    pub top_level: bool,
    pub path: Vec<usize>,
    pub lookup_links: fn(&LookupType) -> String,
}

impl From<Document> for DocumentComponent {
    fn from(document: Document) -> Self {
        Self {
            document,
            dynamic: false,
            top_level: false,
            path: Vec::new(),
            lookup_links,
        }
    }
}

#[derive(Debug)]
pub enum DocumentMsg {
    LoadCitation(Vec<usize>, BiblicalCitation),
    SelectOption(Vec<usize>, InputEvent),
}

impl Component<DocumentMsg, Msg> for DocumentComponent {
    fn update(&mut self, _msg: DocumentMsg) -> Effects<DocumentMsg, Msg> {
        Effects::none()
    }

    fn view(&self) -> Node<DocumentMsg> {
        let label: Option<Node<DocumentMsg>> = self.document.label.as_ref().and_then(|label| {
            if self.top_level {
                None
            } else {
                Some(node! {
                  <h3 class="label">{text(label)}</h3>
                })
            }
        });

        let source: Option<Node<DocumentMsg>> = self
            .document
            .source
            .map(|reference| self.reference(&reference));

        let (header, content) = match &self.document.content {
            Content::Error(error) => self.error(error),
            Content::BiblicalCitation(content) => self.biblical_citation(content),
            Content::BiblicalReading(content) => self.biblical_reading(content),
            Content::Canticle(content) => self.canticle(content),
            Content::CanticleTableEntry(content) => self.canticle_table_entry(content),
            Content::Empty => self.empty(),
            Content::GloriaPatri(content) => self.gloria_patri(content),
            Content::Heading(content) => self.heading(content),
            Content::Liturgy(content) => self.series(&content.body),
            Content::Preces(content) => self.preces(content),
            Content::Psalm(content) => self.psalm(content),
            Content::ResponsivePrayer(content) => self.responsive_prayer(content),
            Content::Rubric(content) => self.rubric(content),
            Content::Sentence(content) => self.sentence(content),
            Content::Text(content) => self.text(content),
            Content::Series(content) => self.series(content),
            Content::Parallel(content) => self.parallel(content),
            Content::Choice(content) => self.choice(content),
            Content::CollectOfTheDay { allow_multiple } => self.collect_of_the_day(*allow_multiple),
            Content::PsalmCitation(content) => self.psalm_citation(content),
            Content::LectionaryReading(content) => self.lectionary_reading(content),
            Content::Antiphon(content) => self.antiphon(content),
            Content::Litany(content) => self.litany(content),
            Content::Category(content) => self.category(content),
        };

        let header = match (label, source, header) {
            (None, None, None) => text(""),
            (label, source, Some(headers)) => {
                node! {
                    <header>
                        {label.unwrap_or_else(|| text(""))}
                        {source.unwrap_or_else(|| text(""))}
                        {for header in headers {
                            header
                        }}
                    </header>
                }
            }
            (label, source, None) => {
                node! {
                    <header>
                        {label.unwrap_or_else(|| text(""))}
                        {source.unwrap_or_else(|| text(""))}
                    </header>
                }
            }
        };

        node! {
            <article class="document">
                {header}
                {content}
            </article>
        }
    }
}

impl DocumentComponent {
    // Helpers
    fn display_format_as_class(&self, display_format: DisplayFormat) -> String {
        match display_format {
            DisplayFormat::Default => "default",
            DisplayFormat::Abbreviated => "abbreviated",
            DisplayFormat::Omit => "omit",
            DisplayFormat::Unison => "unison",
        }
        .to_string()
    }

    fn reference(&self, reference: &Reference) -> Node<DocumentMsg> {
        node! {
            <a class="reference" href={reference.as_url()}>
                {text(reference.to_string())}
            </a>
        }
    }

    fn i18n(&self, text: &str) -> String {
        self.document.language.i18n(text)
    }

    fn set_dynamic(mut self, dynamic: bool) -> Self {
        self.dynamic = dynamic;
        self
    }

    // Content Types
    fn error(&self, error: &DocumentError) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        (
            None,
            node! {
                <article class="document error">
                    <pre>{text(error)}</pre>
                </article>
            },
        )
    }

    fn biblical_citation(
        &self,
        citation: &BiblicalCitation,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let header = Some(vec![node! {
            <h4 class="citation">{text(citation)}</h4>
        }]);

        let path = self.path.clone();
        let citation = citation.clone();

        let main = node! {
            <article class="document biblical-citation" on_mount=move |_| DocumentMsg::LoadCitation(path.clone(), citation.clone())>
            </article>
        };

        (header, main)
    }

    fn biblical_reading(
        &self,
        reading: &BiblicalReading,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let intro = if let Some(intro) = &reading.intro {
            let doc = Document::from(intro.clone());
            DocumentComponent::from(doc)
                .set_dynamic(self.dynamic)
                .view()
        } else {
            text("")
        };

        let header = node! {
            <h3 class="citation">{text(&reading.citation)}</h3>
        };

        let main = node! {
            <main class="biblical-reading">
                {intro}
                {for (verse, verse_text) in &reading.text {
                    node! {
                        <span class="verse">
                            <sup class="verse-number">{text(verse.verse)}</sup>
                            // TODO this has the potential to be insecure if the API is compromised,
                            // or if we eventually allow user-created documents
                            <span {inner_html(verse_text)}></span>
                        </span>
                    }
                }}
            </main>
        };

        (Some(vec![header]), main)
    }

    fn canticle(&self, canticle: &Canticle) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let citation = if let Some(citation) = &canticle.citation {
            node! {
                <h3 class="citation">{text(citation)}</h3>
            }
        } else {
            text("")
        };

        let header = vec![
            node! { <h3 class="canticle-number">{text(canticle.number)}</h3> },
            node! { <h4 class="local-name">{text(&canticle.local_name)}</h4> },
            node! { <em class="latin-name">{text(canticle.latin_name.clone().unwrap_or_default())}</em> },
            citation,
        ];

        let main = node! {
            <main class="canticle">
            {for section in &canticle.sections {
                let header = if let Some(title) = &section.title {
                    node! {
                        <header>
                            <h4 class="canticle-section-title">{text(title)}</h4>
                        </header>
                    }
                } else {
                    text("")
                };

                node! {
                    <section>
                        {header}
                        <main>
                        {for verse in &section.verses {
                            node! {
                                <p class="verse">
                                    <span class="a">{text(&verse.a)}</span>
                                    <span class="b">{text(&verse.b)}</span>
                                </p>
                            }
                        }}
                        </main>
                    </section>
                }
            }}
            </main>
        };

        (Some(header), main)
    }

    fn canticle_table_entry(
        &self,
        entry: &CanticleTableEntry,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let href = (self.lookup_links)(&LookupType::Canticle(entry.clone()));
        let main = node! {
            <main class="lookup canticle-table-entry">
                <a href={href}>{text(self.i18n("Table of Suggested Canticles"))}</a>
            </main>
        };

        (None, main)
    }

    fn category(&self, category: &Category) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let name = category.name.localized_name(self.document.language);
        let href = (self.lookup_links)(&LookupType::Category(
            self.document.version,
            name.to_string(),
        ));
        let main = node! {
            <main class="lookup category">
                <a href={href}>{text(name)}</a>
            </main>
        };

        (None, main)
    }

    fn choice(&self, choice: &Choice) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let header = if self.dynamic {
            let path = self.path.clone();
            Some(vec![node! {
                <nav class="choice-nav">
                    <select
                        class="choice-menu"
                        value={choice.selected}
                        on_change=move |ev| DocumentMsg::SelectOption(path.clone(), ev)
                    >
                        {for (ii, doc) in choice.options.iter().enumerate() {
                            node! {
                                <option value={ii}>
                                    {text(choice.option_label(doc, ii))}
                                </option>
                            }
                        }}
                    </select>
                </nav>
            }])
        } else {
            None
        };

        let main = node! {
            <section class="choice">
                <ol>
                {for (ii, doc) in choice.options.iter().enumerate() {
                    let class = if !self.dynamic || choice.selected == ii { "selected" } else { "hidden" };
                    node! {
                        <li class={class}>
                        {
                            let mut component = DocumentComponent::from(doc.clone());
                            let mut path = self.path.clone();
                            path.push(ii);
                            component.path = path;
                            component.dynamic = self.dynamic;
                            component.view()
                        }
                        </li>
                    }
                }}
                </ol>
            </section>
        };

        (header, main)
    }

    fn collect_of_the_day(
        &self,
        _allow_multiple: bool,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let href = (self.lookup_links)(&LookupType::Collect(self.document.version));
        let main = node! {
            <main class="lookup collect-of-the-day">
                <a href={href}>{text(self.i18n("The Collect of the Day"))}</a>
            </main>
        };

        (None, main)
    }

    fn empty(&self) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="empty"></main>
        };

        (None, main)
    }

    fn gloria_patri(
        &self,
        content: &GloriaPatri,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let display_format = self.display_format_as_class(content.display_format);
        let main = node! {
            <main class={format!("gloria-patri {}", display_format)}>
                <p>
                    <span class="a">{text(&content.text.0)}</span>
                    <span class="b">{text(&content.text.1)}</span>
                    <br/>
                    <span class="c">{text(&content.text.2)}</span>
                    <span class="d">{text(&content.text.3)}</span>
                </p>
                /* <p></p>
                <p>{text(format!("{}<wb>{}", content.text.0, content.text.1))}</p>
                <p>{text(format!("{}<wb>{}", content.text.2, content.text.3))}</p> */
            </main>
        };

        (None, main)
    }

    fn heading(&self, heading: &Heading) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = match heading {
            Heading::Date(date) => node! {
                <main class="heading">
                    <h2 class="date">{text(date)}</h2>
                </main>
            },
            Heading::Day {
                name,
                proper,
                holy_days,
            } => node! {
                <main class="heading day">
                    <h2 class="day-name">{text(name)}</h2>
                    {
                        if let Some(proper) = proper {
                            node! { <h3 class="proper">{text(proper)}</h3> }
                        } else {
                            text("")
                        }
                    }
                    {
                        if let Some(holy_days) = holy_days {
                            node! {
                                <ul>
                                {for holy_day in holy_days {
                                    node! {
                                        <li>{text(holy_day)}</li>
                                    }
                                }}
                                </ul>
                            }
                        } else {
                            text("")
                        }
                    }
                </main>
            },
            Heading::Text(level, content) => {
                let h = match level {
                    HeadingLevel::Heading1 => node! { <h1>{text(content)}</h1> },
                    HeadingLevel::Heading2 => node! { <h2>{text(content)}</h2> },
                    HeadingLevel::Heading3 => node! { <h3>{text(content)}</h3> },
                    HeadingLevel::Heading4 => node! { <h4>{text(content)}</h4> },
                    HeadingLevel::Heading5 => node! { <h5>{text(content)}</h5> },
                };
                node! {
                    <main class="heading">{h}</main>
                }
            }

            // InsertDay and InsertDate can be ignored
            _ => text(""),
        };

        (None, main)
    }

    fn lectionary_reading(
        &self,
        entry: &LectionaryReading,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let href = (self.lookup_links)(&LookupType::Lectionary(entry.lectionary.clone()));

        let main = node! {
            <main class="lookup lectionary">
                <a href={href}>{text(self.i18n("Lectionary Readings"))}</a>
            </main>
        };

        (None, main)
    }

    fn litany(&self, litany: &Litany) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="litany">
                {for line in litany.iter() {
                    node! {
                        <p>
                            <span>{text(line)}</span>
                            <strong class="response">{text(&litany.response)}</strong>
                        </p>
                    }
                }}
            </main>
        };

        (None, main)
    }

    fn preces(&self, preces: &Preces) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="preces">{
                for (label, prayer) in preces.iter() {
                    node! {
                        <p class="line">
                            <em class="label">{text(label)}</em>
                            <span class="text">{text(prayer)}</span>
                        </p>
                    }
                }
            }
            </main>
        };

        (None, main)
    }

    fn psalm(&self, psalm: &Psalm) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="psalm">
            {for section in psalm.filtered_sections() {
                node! {
                    <section>
                        <header>
                            <h3 class="local-name">{text(section.local_name)}</h3>
                            <em class="latin-name">{text(section.latin_name)}</em>
                            {self.reference(&section.reference)}
                        </header>
                        <main>
                        {for verse in section.verses {
                            node! {
                                <p class="verse">
                                    <sup class="number">{text(verse.number)}</sup>
                                    <span class="a">{text(verse.a)}</span>
                                    <span class="b">{text(verse.b)}</span>
                                </p>
                            }
                        }}
                        </main>
                    </section>
                }
            }}
            </main>
        };

        (None, main)
    }

    fn psalm_citation(
        &self,
        psalm_citation: &PsalmCitation,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="psalm-citation"><h3>{text(psalm_citation)}</h3></main>
        };

        (None, main)
    }

    fn responsive_prayer(
        &self,
        responsive_prayer: &ResponsivePrayer,
    ) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="responsive-prayer">
                <p>
                    {for (n, line) in responsive_prayer.iter().enumerate() {
                        if n % 2 == 1 {
                            node! {
                                <span>
                                    <strong class="response">{text(line)}</strong>
                                    <br/>
                                </span>
                            }
                        } else {
                            node! {
                                <span>
                                    {text(line)}
                                    <br/>
                                </span>
                            }
                        }

                    }}
                </p>
            </main>
        };

        (None, main)
    }

    fn rubric(&self, rubric: &Rubric) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="rubric">{text(rubric)}</main>
        };

        (None, main)
    }

    fn sentence(&self, sentence: &Sentence) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let short_text_response = sentence
            .response
            .as_ref()
            .and_then(|doc| match &doc.content {
                Content::Text(text) => {
                    if text.text.len() <= 5 && text.response.is_none() {
                        Some(text)
                    } else {
                        None
                    }
                }
                _ => None,
            });

        let citation: Node<DocumentMsg> = match &sentence.citation {
            Some(citation) => node! { <span class="citation">{text(citation)}</span> },
            None => text(""),
        };

        let main = node! {
            <main class="sentence">
            {match (&sentence.response, short_text_response) {
                // No response
                (None, _) => node! {
                    <p>
                        {text(&sentence.text)}
                        {citation}
                    </p>
                },
                // With a short enough response to be shown inline
                (_, Some(response)) => node! {
                    <p>
                        {text(&sentence.text)}
                        <strong class="response">{text(response)}</strong>
                        {citation}
                    </p>
                },
                // With a longer response, which should be shown on its own level
                (Some(response), None) => node! {
                    <div>
                        <p>
                            {text(&sentence.text)}
                            {citation}
                        </p>
                        {DocumentComponent::from(*response.clone()).set_dynamic(self.dynamic).view()}
                    </div>
                },
            }}
            </main>
        };

        (None, main)
    }

    fn series(&self, series: &Series) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <section class="series">{
                for (ii, doc) in series.iter().enumerate() {
                    let mut component = DocumentComponent::from(doc.clone());
                    let mut path = self.path.clone();
                    path.push(ii);
                    component.path = path;
                    component.dynamic = self.dynamic;
                    component.view()
                }
            }</section>
        };
        (None, main)
    }

    fn parallel(&self, parallel: &Parallel) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <section class="parallel">{
                for (ii, doc) in parallel.iter().enumerate() {
                    let mut component = DocumentComponent::from(doc.clone());
                    let mut path = self.path.clone();
                    path.push(ii);
                    component.path = path;
                    component.dynamic = self.dynamic;
                    component.view()
                }
            }</section>
        };
        (None, main)
    }

    fn text(&self, content: &liturgy::Text) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let paragraphs = content.text.split("\n\n");
        let display_format = self.display_format_as_class(content.display_format);
        let main = node! {
            <main class={format!("text {}", display_format)}>
                {for paragraph in paragraphs {
                    node! {
                        <p>{text(paragraph)}</p>
                    }
                }}
            </main>
        };

        (None, main)
    }

    fn antiphon(&self, antiphon: &Antiphon) -> (Option<Vec<Node<DocumentMsg>>>, Node<DocumentMsg>) {
        let main = node! {
            <main class="antiphon">{text(antiphon)}</main>
        };

        (None, main)
    }
}
