use episcopal_api::liturgy::*;
use futures::{Stream, StreamExt};
use leptos::*;
use wasm_bindgen::UnwrapThrowExt;

use crate::components::biblical_citation::biblical_citation;

use super::lookup::{lookup_links, LookupType};

#[derive(Clone)]
pub struct DocumentController {
    // this intentionally only updates in one direction: it should always hold the current
    // state of the document and sub-documents, but updating it won't necessarily rip out
    // and update the whole View for the Document
    state: Behavior<Document>,
}

impl From<Behavior<Document>> for DocumentController {
    fn from(state: Behavior<Document>) -> Self {
        Self { state }
    }
}

impl DocumentController {
    pub fn new(document: Document) -> Self {
        Self {
            state: Behavior::new(document),
        }
    }

    pub fn view(&self, locale: &str) -> View {
        document_view(locale, self, vec![], &self.state.get())
    }

    pub fn get_state(&self) -> Document {
        self.state.get()
    }

    pub fn stream(&self) -> impl Stream<Item = Document> {
        self.state.stream()
    }

    pub fn update_content_at_path(
        &self,
        path: impl IntoIterator<Item = usize>,
        content: Content,
    ) -> Result<(), PathError> {
        let mut current_state = self.get_state();
        let subdoc = current_state.at_path_mut(path)?;
        subdoc.content = content;

        self.state.set(current_state);
        Ok(())
    }
}

pub fn document_view(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    doc: &Document,
) -> View {
    let label = if matches!(doc.content, Content::Liturgy(_)) {
        View::Empty
    } else if let Some(label) = &doc.label {
        view! {
            <h3>{label}</h3>
        }
    } else {
        View::Empty
    };

    let header_and_main = match &doc.content {
        Content::Series(content) => series(locale, controller, path, content),
        Content::Liturgy(content) => series(locale, &controller.clone(), path, &content.body),
        Content::Rubric(content) => rubric(content),
        Content::Text(content) => text(content),
        Content::Choice(content) => choice(locale, &controller.clone(), path, content),
        Content::Parallel(content) => parallel(locale, controller, path, content),
        Content::Category(content) => category(locale, content, doc.version),
        Content::CollectOfTheDay { allow_multiple } => {
            collect_of_the_day(locale, *allow_multiple, doc.version)
        }
        Content::Empty => empty(),
        Content::Error(content) => error(content),
        Content::Antiphon(content) => antiphon(content),
        Content::BiblicalCitation(content) => (
            None,
            view! { <dyn:view view={biblical_citation(locale, controller, path, content)}/>},
        ),
        Content::BiblicalReading(content) => biblical_reading(locale, controller, path, content),
        Content::Canticle(content) => canticle(content),
        Content::CanticleTableEntry(content) => canticle_table_entry(locale, content),
        Content::GloriaPatri(content) => gloria_patri(content),
        Content::Heading(content) => heading(content),
        Content::LectionaryReading(content) => lectionary_reading(locale, content),
        Content::Litany(content) => litany(content),
        Content::Preces(content) => preces(content),
        Content::Psalm(content) => psalm(content),
        Content::PsalmCitation(content) => psalm_citation(content),
        Content::ResponsivePrayer(content) => responsive_prayer(content),
        Content::Sentence(content) => sentence(locale, controller, path, content),
    };

    let header = if let Some(header) = header_and_main.0 {
        header
    } else {
        View::Empty
    };

    view! {
        <>
            {label}
            {header}
            <dyn:view view={header_and_main.1}/>
        </>
    }
}

type HeaderAndMain = (Option<View>, View);

pub fn antiphon(antiphon: &Antiphon) -> HeaderAndMain {
    (
        None,
        view! {
            <main class="antiphon">{antiphon.to_string()}</main>
        },
    )
}

pub fn biblical_reading(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    reading: &BiblicalReading,
) -> HeaderAndMain {
    let intro = if let Some(intro) = &reading.intro {
        let doc = Document::from(intro.clone());
        document_view(locale, controller, path, &doc)
    } else {
        View::Empty
    };

    let header = view! {
        <h3 class="citation">{&reading.citation}</h3>
    };

    let verses = View::Fragment(
        reading
            .text
            .iter()
            .map(|(verse, verse_text)| {
                view! {
                  <sup class="verse-number">{verse.verse.to_string()}</sup>
                  <span>{verse_text}</span>
                }
            })
            .collect(),
    );

    let main = view! {
        <main class="biblical-reading">
            {intro}
            {verses}
        </main>
    };

    (Some(header), main)
}

pub fn canticle(content: &Canticle) -> HeaderAndMain {
    let citation = if let Some(citation) = &content.citation {
        view! {
            <p class="citation">{citation}</p>
        }
    } else {
        View::Empty
    };

    let header = view! {
        <header class="canticle-header">
            <h3 class="canticle-number">{content.number.to_string()}</h3>
            <h4 class="local-name">{&content.local_name}</h4>
            <em class="latin-name">{content.latin_name.as_ref().cloned().unwrap_or_default()}</em>
             <p class="citation">{citation}</p>
        </header>
    };

    let sections = View::Fragment(
        content
            .sections
            .iter()
            .map(|section| {
                let title = section.title.clone();
                let header = if let Some(title) = title {
                    view! {
                        <header>
                            <h4 class="canticle-section-title">{title}</h4>
                        </header>
                    }
                } else {
                    View::Empty
                };

                let verses = View::Fragment(
                    section
                        .verses
                        .iter()
                        .map(|verse| {
                            view! {
                                <p class="verse">
                                    <span class="a">{&verse.a}</span>
                                    <span class="b">{&verse.b}</span>
                                </p>
                            }
                        })
                        .collect(),
                );

                view! {
                    <section>
                        {header}
                        <main>{verses}</main>
                    </section>
                }
            })
            .collect(),
    );

    let main = view! {
        <main class="canticle">{sections}</main>
    };

    (Some(header), main)
}

pub fn category(locale: &str, content: &Category, version: Version) -> HeaderAndMain {
    let name = t!(&format!("category.{:#?}", content.name));
    let href = lookup_links(locale, &LookupType::Category(version, name.clone()));
    (
        None,
        view! {
            <main class="lookup category">
                <a href={href}>{name}</a>
            </main>
        },
    )
}

pub fn choice(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    choice: &Choice,
) -> HeaderAndMain {
    let selected_str = Behavior::new(choice.selected.to_string());

    // after initial value, edit parent document on every fire
    selected_str.stream().skip(1).create_effect({
        let choice = choice.clone();
        let controller = controller.clone();
        let path = path.clone();
        move |value| {
            let mut new_choice = choice.clone();
            let new_idx = value.parse::<usize>().unwrap_throw();
            new_choice.selected = new_idx;
            if let Err(e) =
                controller.update_content_at_path(path.clone(), Content::Choice(new_choice))
            {
                log(&format!("error updating controller {:#?}", e));
            };
        }
    });

    let options = View::Fragment(
        choice
            .options
            .iter()
            .enumerate()
            .map(|(idx, option)| {
                let label = choice.option_label(option, idx);
                view! {
                    <option value={idx.to_string()}>{label}</option>
                }
            })
            .collect(),
    );

    let main = View::Fragment(
        choice
            .options
            .iter()
            .enumerate()
            .map({
                let selected_str = selected_str.clone();
                move |(idx, option)| {
                    let mut new_path = path.clone();

                    let hidden = selected_str
                        .stream()
                        .map(move |selected_str| selected_str.parse::<usize>().unwrap_or(0) != idx)
                        .boxed_local();

                    new_path.push(idx);

                    view! {
                        <dyn:li
                            class={if idx == choice.selected { "" } else { "hidden"}}
                            class:hidden={hidden}
                        >
                            {document_view(locale, controller, new_path.clone(), option)}
                        </dyn:li>
                    }
                }
            })
            .collect(),
    );

    let on_change = move |ev: Event| selected_str.set(event_target_value(ev));

    (
        None,
        view! {
            <section class="choice">
                <nav>
                    <dyn:select on:change={on_change}>
                        {options}
                    </dyn:select>
                </nav>
                <ol>{main}</ol>
            </section>
        },
    )
}

pub fn canticle_table_entry(locale: &str, entry: &CanticleTableEntry) -> HeaderAndMain {
    let href = lookup_links(locale, &LookupType::Canticle(entry.clone()));
    let main = view! {
        <main class="lookup canticle-table-entry">
            <a href={href}>{t!("lookup.canticle_table")}</a>
        </main>
    };

    (None, main)
}

pub fn collect_of_the_day(locale: &str, _allow_multiple: bool, version: Version) -> HeaderAndMain {
    let href = lookup_links(locale, &LookupType::Collect(version));
    let main = view! {
        <main class="lookup collect-of-the-day">
            <a href={href}>{t!("lookup.collect_of_the_day")}</a>
        </main>
    };

    (None, main)
}

pub fn document_class(doc: &Document) -> &'static str {
    match (doc.display, doc.is_compiled) {
        (Show::Hidden, _) => "document hidden",
        (Show::CompiledOnly, false) => "document hidden",
        (Show::TemplateOnly, true) => "hidden template-only",
        _ => "document",
    }
}

pub fn empty() -> HeaderAndMain {
    (None, View::Empty)
}

pub fn error(error: &DocumentError) -> HeaderAndMain {
    (
        None,
        view! {
            <main class="error">
                <pre>{error.to_string()}</pre>
            </main>
        },
    )
}

pub fn gloria_patri(content: &GloriaPatri) -> HeaderAndMain {
    let display_format = display_format_as_class(content.display_format);
    let (a, b, c, d) = &content.text;
    let main = view! {
        <main class={format!("gloria-patri {}", display_format)}>
            <p>
                <span class="a">{a}</span>
                <span class="b">{b}</span>
                <span class="c">{c}</span>
                <span class="d">{d}</span>
            </p>
        </main>
    };

    (None, main)
}

pub fn heading(heading: &Heading) -> HeaderAndMain {
    let main = match heading {
        Heading::Date(date) => view! {
            <main class="heading">
                <h2 class="date">{date}</h2>
            </main>
        },
        Heading::Day {
            name,
            proper,
            holy_days,
        } => {
            let proper = if let Some(proper) = proper {
                view! {
                    <h3 class="proper">{proper}</h3>
                }
            } else {
                View::Empty
            };

            let holy_days = if let Some(holy_days) = holy_days {
                View::Fragment(
                    holy_days
                        .iter()
                        .map(|holy_day| view! { <li>{holy_day}</li> })
                        .collect(),
                )
            } else {
                view! {}
            };

            view! {
                <main class="heading day">
                    <h2 class="day-name">{name}</h2>
                    {proper}
                    {holy_days}
                </main>
            }
        }
        Heading::Text(level, content) => {
            let h = match level {
                HeadingLevel::Heading1 => view! { <h1>{content}</h1> },
                HeadingLevel::Heading2 => view! { <h2>{content}</h2> },
                HeadingLevel::Heading3 => view! { <h3>{content}</h3> },
                HeadingLevel::Heading4 => view! { <h4>{content}</h4> },
                HeadingLevel::Heading5 => view! { <h5>{content}</h5> },
            };
            view! {
                <main class="heading">{h}</main>
            }
        }

        // InsertDay and InsertDate can be ignored
        _ => view! {},
    };

    (None, main)
}

pub fn lectionary_reading(locale: &str, entry: &LectionaryReading) -> HeaderAndMain {
    let href = lookup_links(locale, &LookupType::Lectionary(entry.lectionary.clone()));
    let main = view! {
        <main class="lookup lectionary">
            <a href={href}>{t!("lookup.lectionary_reading")}</a>
        </main>
    };

    (None, main)
}

pub fn litany(litany: &Litany) -> HeaderAndMain {
    let lines = View::Fragment(
        litany
            .lines
            .iter()
            .map(|line| {
                view! {
                    <p>
                        <span>{line}</span>
                        <strong class="response">{&litany.response}</strong>
                    </p>
                }
            })
            .collect(),
    );

    let main = view! {
        <main class="litany">{lines}</main>
    };

    (None, main)
}

pub fn parallel(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    parallel: &Parallel,
) -> HeaderAndMain {
    let children = View::Fragment(
        parallel
            .iter()
            .enumerate()
            .map({
                move |(idx, doc)| {
                    let mut new_path = path.clone();
                    new_path.push(idx);
                    document_view(locale, controller, new_path.clone(), doc)
                }
            })
            .collect(),
    );

    (
        None,
        view! {
            <section class="parallel">{children}</section>
        },
    )
}

pub fn preces(preces: &Preces) -> HeaderAndMain {
    let lines = View::Fragment(
        preces
            .iter()
            .map(|(label, prayer)| {
                view! {
                    <p class="line">
                        <em class="label">{label}</em>
                        <span class="text">{prayer}</span>
                    </p>
                }
            })
            .collect(),
    );
    let main = view! {
        <main class="preces">{lines}</main>
    };

    (None, main)
}

pub fn psalm(psalm: &Psalm) -> HeaderAndMain {
    let psalm_number = psalm.number;
    let filtered_sections = psalm.filtered_sections();

    let section_1_header = filtered_sections
        .get(0)
        .map(|section| {
            view! {
                <>
                    {if section.local_name.is_empty() {
                        View::Empty
                    } else {
                        view! {
                            <h3 class="local-name">{&section.local_name}</h3>
                        }
                    }}
                    <em class="latin-name">{&section.latin_name}</em>
                    {reference(&section.reference)}
                </>
            }
        })
        .unwrap_or(View::Empty);

    let header_class = filtered_sections
        .get(0)
        .and_then(|section| {
            if section.local_name.is_empty() {
                None
            } else {
                Some("psalm-header with-local-name")
            }
        })
        .unwrap_or("psalm-header");

    let header = view! {
         <header class={header_class}>
            <h3 class="psalm-number">{psalm.number.to_string()}</h3>
            {section_1_header}
        </header>
    };

    let sections = View::Fragment(
        filtered_sections
            .into_iter()
            .enumerate()
            .map(|(idx, section)| {
                let local = section.local_name;
                let latin = section.latin_name;

                let verses = View::Fragment(
                    section
                        .verses
                        .into_iter()
                        .map(|verse| {
                            let number = verse.number;
                            let a = verse.a;
                            let b = verse.b;

                            view! {
                                <p class="verse">
                                    <a id={format!("{}-{}", psalm_number, number)}></a>
                                    <sup class="number">{number.to_string()}</sup>
                                    <span class="a">{a}</span>
                                    <span class="b">{b}</span>
                                </p>
                            }
                        })
                        .collect(),
                );

                let header = if idx > 0 {
                    view! {
                        <header class={if local.is_empty() { "psalm-header section" } else { "psalm-header section with-local-name" }}>
                            {if local.is_empty() {
                                View::Empty
                            } else {
                                view! {
                                    <h3 class="local-name">{local}</h3>
                                }
                            }}
                            <em class="latin-name">{latin}</em>
                        </header>
                    }
                } else {
                    View::Empty
                };

                view! {
                    <section>
                        {header}
                        <main>{verses}</main>
                    </section>
                }
            })
            .collect(),
    );

    let main = view! {
        <main class="psalm">{sections}</main>
    };

    (Some(header), main)
}

pub fn psalm_citation(citation: &PsalmCitation) -> HeaderAndMain {
    (
        None,
        view! {
            <main class="psalm-citation">
                <h3>{citation.to_string()}</h3>
            </main>
        },
    )
}

pub fn reference(reference: &Reference) -> View {
    let href = reference.as_url();
    let text = reference.to_string();
    view! {
        <a class="reference" href={href}>{text}</a>
    }
}

pub fn responsive_prayer(prayer: &ResponsivePrayer) -> HeaderAndMain {
    let lines = View::Fragment(
        prayer
            .iter()
            .enumerate()
            .map(|(n, line)| {
                if n % 2 == 1 {
                    view! {
                        <span>
                            <strong class="response">{line}</strong>
                            <br/>
                        </span>
                    }
                } else {
                    view! {
                        <span>
                            {line}
                            <br/>
                        </span>
                    }
                }
            })
            .collect(),
    );

    let main = view! {
        <main class="responsive-prayer">
            <p>{lines}</p>
        </main>
    };

    (None, main)
}

pub fn rubric(rubric: &Rubric) -> HeaderAndMain {
    (None, {
        view! {
            <p class="rubric">{rubric.to_string()}</p>
        }
    })
}

pub fn sentence(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    sentence: &Sentence,
) -> HeaderAndMain {
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

    let citation = match &sentence.citation {
        Some(citation) => view! { <span class="citation">{citation}</span> },
        None => View::Empty,
    };

    let text = &sentence.text;

    let body = match (&sentence.response, short_text_response) {
        // No response
        (None, _) => view! {
            <p>{text} {citation}</p>
        },
        (_, Some(response)) => view! {
            <p>
                {text}
                <strong class="response">{response.to_string()}</strong>
                {citation}
            </p>
        },
        (Some(response), None) => view! {
            <div>
                <p>{text} {citation}</p>
                {document_view(locale, controller, path, response)}
            </div>
        },
    };

    let main = view! {
        <main class="sentence">{body}</main>
    };

    (None, main)
}

pub fn series(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    series: &Series,
) -> HeaderAndMain {
    (
        None,
        View::Fragment(
            series
                .iter()
                .enumerate()
                .map({
                    move |(idx, doc)| {
                        let mut new_path = path.clone();
                        new_path.push(idx);
                        view! {
                            <article class={document_class(doc)}>{document_view(locale, controller, new_path.clone(), doc)}</article>
                        }
                    }
                })
                .collect(),
        ),
    )
}

pub fn text(text: &Text) -> HeaderAndMain {
    let class = format!("text {}", display_format_as_class(text.display_format));
    let response = &text.response;

    // needs to collect here in order for last element to be checked
    let paragraphs = text
        .text
        .split("\n\n")
        .map(String::from)
        .collect::<Vec<_>>();
    let length = paragraphs.len();
    let paragraphs = View::Fragment(
        paragraphs
            .into_iter()
            .enumerate()
            .map(|(idx, text)| {
                let response = if let Some(response) = response {
                    view! { <strong class="response">{response}</strong> }
                } else {
                    View::Empty
                };

                if idx == length - 1 {
                    view! {
                        <p>{text} " " {response}</p>
                    }
                } else {
                    view! {
                        <p>{text}</p>
                    }
                }
            })
            .collect(),
    );

    (
        None,
        view! {
            <main class={class}>
                {paragraphs}
            </main>
        },
    )
}

fn display_format_as_class(display_format: DisplayFormat) -> &'static str {
    match display_format {
        DisplayFormat::Default => "default",
        DisplayFormat::Abbreviated => "abbreviated",
        DisplayFormat::Omit => "omit",
        DisplayFormat::Unison => "unison",
    }
}
