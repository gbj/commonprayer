use std::{collections::HashSet, pin::Pin};

use futures::{Stream, StreamExt};
use leptos::*;
use liturgy::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::{
    components::*,
    utils::fetch::{Fetch, FetchError, FetchStatus},
};

use super::lookup::{lookup_links, LookupType};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SelectionMode {
    Idle,
    Auto,
    Manual,
}

#[derive(Clone)]
pub struct DocumentController {
    // this intentionally only updates in one direction: it should always hold the current
    // state of the document and sub-documents, but updating it won't necessarily rip out
    // and update the whole View for the Document
    state: Behavior<Document>,
    // manages which subdocuments, if any, have been selected
    pub selections: Option<Behavior<HashSet<Vec<usize>>>>,
    pub selection_mode: Behavior<SelectionMode>,
}

impl From<Behavior<Document>> for DocumentController {
    fn from(state: Behavior<Document>) -> Self {
        Self {
            state,
            selections: None,
            selection_mode: Behavior::new(SelectionMode::Idle),
        }
    }
}

impl DocumentController {
    pub fn new(document: Document) -> Self {
        Self {
            state: Behavior::new(document),
            selections: None,
            selection_mode: Behavior::new(SelectionMode::Idle),
        }
    }

    pub fn new_with_selections(
        document: Document,
        selections: Behavior<HashSet<Vec<usize>>>,
    ) -> Self {
        Self {
            state: Behavior::new(document),
            selections: Some(selections),
            selection_mode: Behavior::new(SelectionMode::Idle),
        }
    }

    pub fn view(&self, locale: &str) -> View {
        document_view(locale, self, vec![], &self.state.get())
    }

    pub fn get_state(&self) -> Document {
        self.state.get()
    }

    /// If there are selections listed, gives a Series of the selected Documents.
    /// If there are no selections, returns the whole Document.
    pub fn get_state_filtered_by_selections(&self) -> Document {
        let doc = self.get_state();
        if let Some(selections) = &self.selections {
            let selections = selections.get();
            let mut docs = doc
                .flatten_with_path()
                .filter_map(|(path, doc)| {
                    if selections.contains(&path) {
                        Some(doc)
                    } else {
                        None
                    }
                })
                .cloned();
            Document::series_or_document(&mut docs).unwrap_or_else(|| doc.clone())
        } else {
            doc
        }
    }

    pub fn stream(&self) -> impl Stream<Item = Document> {
        self.state.stream()
    }

    pub fn filtered_stream(&self) -> impl Stream<Item = Document> {
        if let Some(selections) = &self.selections {
            Box::pin(
                (self.stream(), selections.stream())
                    .lift()
                    .filter_map(|(state, selections)| async {
                        match (state, selections) {
                            (Some(state), Some(selections)) => Some((state, selections)),
                            _ => None,
                        }
                    })
                    .map(|(state, selections)| {
                        let mut docs = state
                            .flatten_with_path()
                            .filter_map(|(path, doc)| {
                                if selections.contains(&path) {
                                    Some(doc)
                                } else {
                                    None
                                }
                            })
                            .cloned();
                        Document::series_or_document(&mut docs).unwrap_or_else(|| state.clone())
                    }),
            ) as Pin<Box<dyn Stream<Item = Document>>>
        } else {
            Box::pin(self.stream()) as Pin<Box<dyn Stream<Item = Document>>>
        }
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

    pub fn update_document_at_path(
        &self,
        path: impl IntoIterator<Item = usize>,
        document: Document,
    ) -> Result<(), PathError> {
        let mut current_state = self.get_state();
        let subdoc = current_state.at_path_mut(path)?;
        *subdoc = document;

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
    } else {
        match (&doc.label, &doc.subtitle) {
            (None, None) => View::Empty,
            (Some(label), None) => view! {
                <h3>{label}</h3>
            },
            (None, Some(subtitle)) => view! {
                <h4 class="subtitle">{subtitle}</h4>
            },
            (Some(label), Some(subtitle)) => view! {
                <header class="label-and-subtitle">
                    <h3>{label}</h3>
                    <h4 class="subtitle">{subtitle}</h4>
                </header>
            },
        }
    };

    let header_and_main = {
        let path = path.clone();
        match &doc.content {
            Content::Series(content) => series(locale, controller, path, content),
            Content::Liturgy(content) => liturgy(
                locale,
                controller,
                path,
                content,
                &doc.source,
                &doc.alternate_sources,
            ),
            Content::Rubric(content) => rubric(content),
            Content::Text(content) => text(content),
            Content::Choice(content) => choice(locale, &controller.clone(), path, content),
            Content::Parallel(content) => parallel(locale, controller, path, content),
            Content::CollectOfTheDay { allow_multiple } => {
                collect_of_the_day(locale, *allow_multiple, doc.version)
            }
            Content::DocumentLink { label, path, .. } => document_link(locale, label, path),
            Content::Empty => empty(),
            Content::Error(content) => error(content),
            Content::Antiphon(content) => antiphon(content),
            Content::BiblicalCitation(content) => (
                None,
                view! { <dyn:view view={biblical_citation(locale, controller, path, content, doc.version)}/>},
            ),
            Content::BiblicalReading(content) => {
                biblical_reading(locale, controller, path, content)
            }
            Content::Canticle(content) => canticle(content, doc.version, path, controller),
            Content::CanticleTableEntry(content) => canticle_table_entry(locale, content),
            Content::GloriaPatri(content) => gloria_patri(content),
            Content::Heading(content) => heading(locale, content),
            Content::HymnLink(content) => hymn_link(locale, content),
            Content::Invitatory(content) => invitatory(content),
            Content::LectionaryReading(content) => lectionary_reading(locale, content),
            Content::Litany(content) => litany(content),
            Content::Preces(content) => preces(content),
            Content::Psalm(content) => psalm(content),
            Content::PsalmCitation(content) => psalm_citation(content),
            Content::ResponsivePrayer(content) => responsive_prayer(content),
            Content::Sentence(content) => sentence(locale, controller, path, content),
        }
    };

    let header = if let Some(header) = header_and_main.0 {
        header
    } else {
        View::Empty
    };

    let (.., main) = header_and_main;

    // manage whether this item is being selected
    let dom_ref = Behavior::<Option<web_sys::Element>>::new(None);
    let selected = Behavior::new(false);

    let selection_change = document_event_stream("selectionchange");
    selection_change.create_effect({
        let dom_ref = dom_ref.clone();
        let selected = selected.clone();
        let manual_mode = controller.selection_mode.clone();
        move |_| {
            if manual_mode.get() == SelectionMode::Auto {
                if let Ok(selection) = window().get_selection() {
                    if let Some(selection) = selection {
                        if let Some(el) = dom_ref.get() {
                            let is_selected = selection.contains_node(&el).unwrap_or(false)
                                || descendants(&el)
                                    .any(|node| selection.contains_node(&node).unwrap_or(false));
                            selected.set(is_selected);
                        }
                    } else {
                        selected.set(false);
                    }
                }
            }
        }
    });

    let is_leaf_doc = doc.content.is_leaf();
    let is_selected = selected
        .stream()
        .map(move |selected| selected && is_leaf_doc)
        .boxed_local();

    let hide_selection_checkbox = controller
        .selection_mode
        .stream()
        .map(|mode| mode != SelectionMode::Manual)
        .boxed_local();

    // register selection with root DocumentController
    if is_leaf_doc {
        let controller = controller.clone();
        let path = path.clone();
        if let Some(selections) = controller.selections {
            selected.stream().create_effect(move |selected| {
                selections.update(|selections| {
                    if selected {
                        selections.insert(path.clone());
                    } else {
                        selections.remove(&path);
                    }
                })
            });
        }
    }

    let checkbox = if let Some(selections) = &controller.selections {
        if is_leaf_doc {
            view! {
                <dyn:input
                    type="checkbox"
                    class="manual-select hidden"
                    class:hidden={hide_selection_checkbox}
                    on:change={
                        let selections = selections.clone();
                        let manual_mode = controller.selection_mode.clone();
                        move |ev: Event| {
                            if manual_mode.get() == SelectionMode::Manual {
                                let checked = event_target_checked(ev);
                                selections.update(|selections| {
                                    if checked {
                                        selected.set(true);
                                        selections.insert(path.clone());
                                    } else {
                                        selected.set(false);
                                        selections.remove(&path);
                                    }
                                });
                            }
                        }
                    }
                />
            }
        } else {
            View::Empty
        }
    } else {
        View::Empty
    };

    let main = view! {
        <dyn:article class={document_class(doc)}
            dom:ref={&dom_ref}
            class:selected={is_selected}
        >
            {main}
        </dyn:article>
    };

    view! {
        // this could be a fragment, but causes problems in web components
        // because a DocumentFragment can't be replaceNode'd with another one
        <div class="document">
            {checkbox}
            {label}
            {header}
            {main}
        </div>
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
        view! {
            <section class="reading-intro">
                {document_view(locale, controller, path, &doc)}
            </section>
        }
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
                  <span>{small_capify(verse_text)}</span>
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

pub fn canticle(
    content: &Canticle,
    default_version: Version,
    path: Vec<usize>,
    controller: &DocumentController,
) -> HeaderAndMain {
    let current_content = Behavior::new(content.clone());

    // Canticle swap
    let canticle_options = Fetch::<Vec<Document>>::new("/api/canticles.json");
    let show_option_list = Behavior::new(false);

    let change_canticle = {
        let canticle_options = canticle_options.clone();
        let show_option_list = show_option_list.clone();
        move |_ev: Event| {
            canticle_options.send();
            show_option_list.set(true);
        }
    };

    let version_filter = SegmentButton::new_with_default_value(
        "canticle-version",
        Some(t!("settings.version")),
        [
            (None, t!("canticle_swap.any"), None),
            (Some(Version::RiteI), t!("rite_i"), None),
            (Some(Version::RiteII), t!("rite_ii"), None),
            (Some(Version::EOW), t!("eow"), None),
        ],
        Some(default_version),
    );

    let option_list = canticle_options
        .state
        .stream()
        .map({
            let controller = controller.clone();
            let current_content = current_content.clone();
            let show_option_list = show_option_list.clone();
            let version_filter_value = version_filter.value.clone();
            move |status| match status {
                FetchStatus::Idle => View::Empty,
                FetchStatus::Loading => view! { <p class="loading">{t!("loading")}</p> },
                FetchStatus::Error(e) => {
                    match e {
                        FetchError::Connection => view! { <p class="error">{t!("canticle_swap.connection_error")}</p> },
                        _ => view! { <p class="error">{t!("canticle_swap.error")}</p> }
                    }
                }
                FetchStatus::Success(docs) => {
                    let docs = View::Fragment(
                        docs.iter()
                            .filter_map({
                                let path = path.clone();
                                let controller = controller.clone();
                                let current_content = current_content.clone();
                                let show_option_list = show_option_list.clone();
                                let version_filter_value = version_filter_value.clone();
                                move |doc| {
                                    if let Content::Canticle(content) = &doc.content {
                                        let doc_version = doc.version;
                                        let hidden = version_filter_value.stream().map(move |version| version.is_some() && version.unwrap() != doc_version).boxed_local();

                                        Some(view! {
                                            <dyn:li
                                                role="button"
                                                class:hidden={hidden}
                                                on:click={
                                                    let doc = doc.clone();
                                                    let controller = controller.clone();
                                                    let path = path.clone();
                                                    let current_content = current_content.clone();
                                                    let show_option_list = show_option_list.clone();
                                                    move |_ev: Event| {
                                                        let update_result = controller.update_document_at_path(path.clone(), doc.clone());
                                                        if let Err(e) = update_result {
                                                            warn(&format!("[error when calling controller.update_document_at_path({:#?}, ..)]\n\n{:#?}", path, e));
                                                        }
                                                        show_option_list.set(false);
                                                        if let Content::Canticle(canticle) = doc.content.clone() {
                                                            current_content.set(canticle);
                                                        }
                                                    }
                                                }
                                            >
                                                {content.number.to_string()} ". " {&content.local_name}
                                            </dyn:li>
                                        })
                                    } else {
                                        None
                                    }
                                }
                            })
                            .collect(),
                    );

                    view! {
                        <ul>
                            {docs}
                        </ul>
                    }
                }
        }})
        .boxed_local();

    let canticle_swap = view! {
        <nav class="canticle-swap-menu">
            <dyn:button
                on:click=change_canticle
            >
                <img src={Icon::Swap.to_string()} alt=""/>
                {t!("canticle_swap.change_canticle")}
            </dyn:button>
            <dyn:div
                class:overlay={show_option_list.stream().map(|_| true).boxed_local()}
                class:shown={show_option_list.stream().boxed_local()}
                on:click={
                    let show_option_list = show_option_list.clone();
                    move |_ev: Event| show_option_list.set(false)
                }
            >
            </dyn:div>
            <dyn:section
                class:menu_content={show_option_list.stream().map(|_| true).boxed_local()}
                class:shown={show_option_list.stream().boxed_local()}
            >
                <header>
                    <h1>{t!("canticle_swap.choose")}</h1>
                    <dyn:button
                        on:click=move |_ev: Event| show_option_list.set(false)
                    >
                        <img src={Icon::Close.to_string()} alt={t!("canticle_swap.close")}/>
                    </dyn:button>
                </header>
                <main>
                    {version_filter.view()}
                    {option_list}
                </main>
            </dyn:section>
        </nav>
    };

    // Header proper
    let number = current_content
        .stream()
        .map(|content| content.number.to_string())
        .boxed_local();
    let local = current_content
        .stream()
        .map(|content| content.local_name)
        .boxed_local();
    let latin = current_content
        .stream()
        .map(|content| content.latin_name.unwrap_or_default())
        .boxed_local();
    let citation = current_content
        .stream()
        .map(|content| content.citation.unwrap_or_default())
        .boxed_local();

    let header = view! {
        <>
            <dyn:view view={canticle_swap} />
            <header class="canticle-header">
                <dyn:h3 class="canticle-number">{number}</dyn:h3>
                <dyn:h4 class="local-name">{local}</dyn:h4>
                <dyn:em class="latin-name">{latin}</dyn:em>
                <dyn:p class="citation">{citation}</dyn:p>
            </header>
        </>
    };

    // Main
    let rubric = current_content
        .stream()
        .map(|content| {
            if let Some(rubric) = &content.rubric {
                view! {
                    <em class="rubric">{rubric}</em>
                }
            } else {
                View::Empty
            }
        })
        .boxed_local();

    let gloria = current_content
        .stream()
        .map(|content| {
            content
                .gloria_patri
                .as_ref()
                .map(|content| {
                    let gloria_main = gloria_patri(content).1;
                    view! { <article class="document">{gloria_main}</article> }
                })
                .unwrap_or(View::Empty)
        })
        .boxed_local();

    let sections = current_content
        .stream()
        .map(|content| {
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
                                            <span class="a">{small_capify(&verse.a)}</span>
                                            <span class="b">{small_capify(&verse.b)}</span>
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
            view! {
                <div>{sections}</div>
            }
        })
        .boxed_local();

    (
        Some(header),
        view! {
            <main class="canticle">
                {rubric}
                {sections}
                {gloria}
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
    let max_idx = choice.options.len() - 1;

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

    // View
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

                    // swiping left and right on touch-enabled devices will rotate among selections
                    let swipestart = Behavior::new(0);
                    let swipe_offset = Behavior::new(0);
                    let swipe_offset_style = swipe_offset.stream().map(|offset| if offset == 0 { "".to_string() } else { format!("translateX({}px)", offset) }).boxed_local();
                    let can_swipe_left = idx > 0;
                    let can_swipe_right = idx < max_idx;

                    view! {
                        <dyn:li
                            class={if idx == choice.selected { "" } else { "hidden"}}
                            class:hidden={hidden}
                            style:transform={swipe_offset_style}
                            on:touchstart={
                                let swipestart = swipestart.clone();
                                let swipe_offset = swipe_offset.clone();
                                move |ev: Event| {
                                    // reset offset and set base X coordinate
                                    swipe_offset.set(0);
                                    swipestart.set(ev.unchecked_into::<web_sys::TouchEvent>().touches().get(0).unwrap().screen_x());
                                }
                            }
                            on:touchmove={
                                let swipestart = swipestart.clone();
                                let swipe_offset = swipe_offset.clone();
                                move |ev: Event| {
                                    // set offset (moves item on screen)
                                    let current_x = ev.clone().unchecked_into::<web_sys::TouchEvent>().touches().get(0).unwrap().screen_x();
                                    let offset = current_x - swipestart.get();

                                    // if it's clear that they're trying to swipe, and not scroll (i.e., offset has
                                    // gotten big enough), then prevent other events (like click and scroll)
                                    if offset.abs() > 30 {
                                        ev.prevent_default();
                                    }

                                    // update offset
                                    if offset == 0 || (offset < 0 && can_swipe_right) || (offset > 0 && can_swipe_left) {
                                        swipe_offset.set(offset);
                                    } else {
                                        log("can't swipe further in the desire direction");
                                    }
                                }
                            }
                            on:touchend={
                                let selected_str = selected_str.clone();
                                move |_ev: Event| {
                                    let offset = swipe_offset.get();
                                    // if you've swiped 100px or more in either direction, swap
                                    if let Ok(current_idx) = selected_str.get().parse::<usize>() {
                                        if offset <= -100 && can_swipe_right {
                                            selected_str.set((current_idx + 1).to_string());
                                        } else if offset >= 100 && can_swipe_left {
                                            selected_str.set((current_idx - 1).to_string());
                                        }
                                    }

                                    // reset screen offset and base, so it snaps back and is ready to be swiped again
                                    swipestart.set(0);
                                    swipe_offset.set(0);
                                }
                            }
                        >
                            {document_view(locale, controller, new_path.clone(), option)}
                        </dyn:li>
                    }
                }
            })
            .collect(),
    );

    let on_change = {
        let selected_str = selected_str.clone();
        move |ev: Event| selected_str.set(event_target_value(ev))
    };

    let menu = if choice.options.len() > 1 {
        view! {
            <nav>
                <dyn:select
                    prop:value={selected_str.stream().map(Some).boxed_local()}
                    on:change={on_change}
                >
                    {options}
                </dyn:select>
            </nav>
        }
    } else {
        View::Empty
    };

    (
        None,
        view! {
            <section class="choice">
                {menu}
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

pub fn document_class(doc: &Document) -> String {
    format!(
        "{}{}",
        if doc.optional { "optional " } else { "" },
        match (doc.display, doc.is_compiled) {
            (Show::Hidden, _) => "document hidden",
            (Show::CompiledOnly, false) => "document hidden",
            (Show::TemplateOnly, true) => "hidden template-only",
            _ => "document",
        }
    )
}

pub fn document_link(locale: &str, label: &str, path: &SlugPath) -> HeaderAndMain {
    let path = path
        .into_iter()
        .map(Slug::slugify)
        .intersperse_with(|| String::from("/"))
        .collect::<String>();
    let href = format!("/{locale}/document/{path}");
    (
        None,
        view! {
            <main class="lookup document">
                <a href={href}>{label}</a>
            </main>
        },
    )
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
                <br/>
                <span class="c">{c}</span>
                <span class="d">{d}</span>
            </p>
        </main>
    };

    (None, main)
}

pub fn heading(locale: &str, heading: &Heading) -> HeaderAndMain {
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
                        .map(|(feast, name)| view! {
                            <li>
                                <a href={&format!("/{}/holy-day/{:#?}", locale, feast)}>{name}</a>
                            </li>
                        })
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
            let content = View::Fragment(
                content
                    .split('\n')
                    .map(|text| View::StaticText(text.to_string()))
                    .intersperse_with(|| view! { <br/> })
                    .collect(),
            );
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

pub fn hymn_link(locale: &str, content: &HymnLink) -> HeaderAndMain {
    let href = match content {
        HymnLink::Hymnals => format!("/{}/hymnal", locale),
        HymnLink::Hymnal(hymnal_id) => format!("/{}/hymnal/{:#?}", locale, hymnal_id),
        HymnLink::Hymn(hymnal_id, number) => {
            format!("/{}/hymnal/{:#?}/{}", locale, hymnal_id, number)
        }
        HymnLink::Tag(tag) | HymnLink::TagWithLabel(tag, _) => format!("/{}/hymnal?q=tag:{}", locale, tag),
    };

    let label = match content {
        HymnLink::Hymnals => t!("menu.hymnal"),
        HymnLink::Hymnal(hymnal_id) => hymnal_id.to_string(),
        HymnLink::Hymn(hymnal_id, number) => {
            format!("{} {}", hymnal_id, number)
        }
        HymnLink::Tag(tag) => t!("hymnal.category_lookup", category = tag),
        HymnLink::TagWithLabel(_, label) => label.clone()
    };

    (
        None,
        view! {
            <main class="lookup hymnal">
                <a href={href}>{label}</a>
            </main>
        },
    )
}

pub fn invitatory(psalm: &Invitatory) -> HeaderAndMain {
    let latin_name = if let Some(latin) = &psalm.latin_name {
        view! {
            <h4 class="latin-name">{latin}</h4>
        }
    } else {
        View::Empty
    };

    let citation = if let Some(citation) = &psalm.citation {
        view! {
            <p class="citation">{citation}</p>
        }
    } else {
        View::Empty
    };

    let header = view! {
         <header class="invitatory-header">
            <h3 class="local-name">{&psalm.local_name}</h3>
            {latin_name}
            {citation}
        </header>
    };

    let sections = View::Fragment(
        psalm
            .sections
            .iter()
            .map(|section| {
                let verses = View::Fragment(
                    section
                        .verses
                        .iter()
                        .map(|verse| {
                            let a = small_capify(&verse.a);
                            let b = small_capify(&verse.b);

                            view! {
                                <p class="verse">
                                    <span class="a">{a}</span>
                                    <span class="b">{b}</span>
                                </p>
                            }
                        })
                        .collect(),
                );

                let antiphon = if let SeasonalAntiphon::Antiphon(ant) = &psalm.antiphon {
                    view! {
                        <section class="repeat-antiphon">{antiphon(ant).1}</section>
                    }
                } else {
                    View::Empty
                };

                view! {
                    <section>
                        <main>{verses}</main>
                        {antiphon}
                    </section>
                }
            })
            .collect(),
    );

    let antiphon_before = if let SeasonalAntiphon::Antiphon(ant) = &psalm.antiphon {
        antiphon(ant).1
    } else {
        View::Empty
    };

    let antiphon_after = if let SeasonalAntiphon::Antiphon(ant) = &psalm.antiphon {
        antiphon(ant).1
    } else {
        View::Empty
    };

    let main = view! {
        <main class="invitatory">
            {antiphon_before}
            {sections}
            {antiphon_after}
        </main>
    };

    (Some(header), main)
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
                let is_optional = line.starts_with("| ");
                let line = line.replace("| ", "");
                let short_response = litany.response.len() <= 8;
                let class = if is_optional { "optional" } else { "" };

                view! {
                    <p class={class}>
                        <span>{minimal_markdown(&line)}</span>
                        {if short_response {
                            View::StaticText(" ".to_string())
                        } else {
                            view! { <br/> }
                        }}
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

pub fn liturgy(
    locale: &str,
    controller: &DocumentController,
    path: Vec<usize>,
    liturgy: &Liturgy,
    source: &Option<Reference>,
    alternate_sources: &[Reference],
) -> HeaderAndMain {
    let (header, main) = series(locale, controller, path, &liturgy.body);

    let source_links = if source.is_some() || !alternate_sources.is_empty() {
        let alternates = View::Fragment(
            alternate_sources
                .iter()
                .map(|source| source_link(&Some(*source)))
                .collect(),
        );
        view! {
            <div class="source-links">
                {source_link(source)}
                {alternates}
            </div>
        }
    } else {
        View::Empty
    };

    (
        header,
        view! {
            <article class="liturgy-with-source-links">
                {source_links}
                {main}
            </article>
        },
    )
}

pub fn source_link(reference: &Option<Reference>) -> View {
    if let Some(reference) = reference {
        view! {
            <a class="source-link"
                href={reference.as_url()}
                target="_blank"
            >
                <span>{t!("source")}</span>
                {t!("reference", source = reference.source.to_string().as_str(), page = reference.page.to_string().as_str())}
            </a>
        }
    } else {
        View::Empty
    }
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
                let lines = View::Fragment(
                    prayer
                        .split('\n')
                        .map(minimal_markdown)
                        .intersperse_with(|| view! { <br/> })
                        .collect(),
                );

                view! {
                    <p class="line">
                        <em class="label">{label}</em>
                        <span class="text">{lines}</span>
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
                            let a = small_capify(&verse.a);
                            let b = small_capify(&verse.b);

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
                let line = View::Fragment(
                    line.split('\n')
                        .map(minimal_markdown)
                        .intersperse_with(|| view! { <br/> })
                        .collect(),
                );

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
    let class = if rubric.long { "rubric-long" } else { "rubric" };

    (
        None,
        View::Fragment(
            rubric
                .text
                .split("\n\n")
                .map(|rubric| {
                    view! {
                        <p class={class}>{minimal_markdown(rubric)}</p>
                    }
                })
                .collect(),
        ),
    )
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
                " "
                <strong class="response">{response.to_string()}</strong>
                {citation}
            </p>
        },
        (Some(response), None) => view! {
            <div>
                <p>{text} {citation}</p>
                " "
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
                        document_view(locale, controller, new_path.clone(), doc)
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

                let separator = if text.is_empty() { "" } else { " " };

                if idx == length - 1 {
                    view! {
                        <p>{minimal_markdown(&text)} {separator} {response}</p>
                    }
                } else {
                    view! {
                        <p>{minimal_markdown(&text)}</p>
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

fn minimal_markdown(s: &str) -> View {
    View::Fragment(
        s.replace("\\*", "∗") // replace escaped asterisks with another Unicode asterisk
            .split("**")
            .enumerate()
            .map(|(bold_idx, piece)| {
                let italic_view = View::Fragment(
                    piece
                        .split('*')
                        .enumerate()
                        .map(|(italic_idx, text)| {
                            if italic_idx % 2 == 1 {
                                view! {
                                    <em>{text.replace('∗', "*")}</em>
                                }
                            } else {
                                View::StaticText(text.replace('∗', "*"))
                            }
                        })
                        .collect(),
                );

                if bold_idx % 2 == 1 {
                    view! {
                        <strong>{italic_view}</strong>
                    }
                } else {
                    italic_view
                }
            })
            .collect(),
    )
}

pub fn small_capify(s: &str) -> View {
    View::Fragment(
        s.split_inclusive("LORD")
            .flat_map(|s| s.split_inclusive("GOD"))
            .flat_map(|s| s.split_inclusive("YAHWEH"))
            .flat_map(|piece| {
                if piece.ends_with("LORD") {
                    [
                        View::StaticText(piece.replace("LORD", "")),
                        view! { <span class="lord">"Lord"</span> },
                    ]
                    .into_iter()
                } else if piece.ends_with("GOD") {
                    [
                        View::StaticText(piece.replace("GOD", "")),
                        view! { <span class="lord">"God"</span> },
                    ]
                    .into_iter()
                } else if piece.ends_with("YAHWEH") {
                    [
                        View::StaticText(piece.replace("YAHWEH", "")),
                        view! { <span class="lord">"Yahweh"</span> },
                    ]
                    .into_iter()
                } else {
                    [View::Empty, View::StaticText(piece.to_string())].into_iter()
                }
            })
            .collect::<Vec<_>>(),
    )
}
