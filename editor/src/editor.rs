use std::pin::Pin;

use crate::{condition::ConditionEditor, content::content_editing_view};
use futures::{Stream, StreamExt};
use leptos::*;
use language::Language;
use liturgy::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use std::str::FromStr;
use to_rust_code::ToRustCode;
use wasm_bindgen::JsCast;
use web_sys::DragEvent;

pub struct Editor {
    editable_doc: EditableDocument,
    undo_stack: Behavior<Vec<Document>>,
    redo_stack: Behavior<Vec<Document>>,
}

const AUTOSAVE: &str = "autosave";

#[derive(Copy, Clone, PartialEq, Eq)]
enum SecondPane {
    Preview,
    Json,
    Rust,
}

impl Editor {
    pub fn new() -> Self {
        // load from autosave
        let storage = window().local_storage().unwrap().unwrap();
        let editable_doc = storage
            .get(AUTOSAVE)
            .unwrap()
            .map(|value| EditableDocument::from(serde_json::from_str::<Document>(&value).unwrap()))
            .unwrap_or_else(|| {
                EditableDocument::from(Document::from(Liturgy::from(Series::from(vec![
                    Document::from(""),
                ]))))
            });

        // Update Undo Stack
        let undo_stack = Behavior::new(Vec::new());

        editable_doc.stream().create_effect({
            let undo_stack = undo_stack.clone();
            move |new_doc| undo_stack.update(|stack| stack.push(new_doc.clone()))
        });

        // Autosave
        editable_doc.stream().create_effect(move |new_doc| {
            storage.set(AUTOSAVE, &serde_json::to_string(&new_doc).unwrap());
        });

        Self {
            editable_doc,
            undo_stack,
            redo_stack: Behavior::new(Vec::new()),
        }
    }

    pub fn view(&self) -> View {
        let secondary_pane_mode = Behavior::new(SecondPane::Preview);

        let json_stream = self
            .editable_doc
            .stream()
            .map(|doc| serde_json::to_string_pretty(&doc).unwrap())
            .boxed_local();

        let wc_doc_stream = self
            .editable_doc
            .stream()
            .map(|doc| serde_json::to_string(&doc).unwrap())
            .boxed_local();

        let rust_stream = self
            .editable_doc
            .stream()
            .map(|doc| doc.to_rust_code(0))
            .boxed_local();

        view! {
           <main>
                <h1>"Liturgy Editor"</h1>
                <dyn:button
                    class="hidden"
                    class:hidden={self.undo_stack.stream().map(|stack| stack.len() == 1).boxed_local()}
                    on:click={
                        let editable_doc = self.editable_doc.document.clone();
                        let undo_stack = self.undo_stack.clone();
                        let redo_stack = self.redo_stack.clone();
                        move |_ev: Event| {
                            let mut stack = undo_stack.get();
                            // remove the latest change
                            let this = stack.pop().unwrap();
                            redo_stack.update(move |stack| stack.push(this.clone()));
                            let last = stack.pop();
                            undo_stack.set(stack);
                            editable_doc.set(last.unwrap());
                        }
                    }
                >
                    "Undo"
                </dyn:button>
                <dyn:button
                    class="hidden"
                    class:hidden={self.redo_stack.stream().map(|stack| stack.is_empty()).boxed_local()}
                    on:click={
                        let editable_doc = self.editable_doc.document.clone();
                        let redo_stack = self.redo_stack.clone();
                        move |_ev: Event| {
                            let mut stack = redo_stack.get();
                            let this = stack.pop();
                            redo_stack.set(stack);
                            editable_doc.set(this.unwrap());
                        }
                    }
                >
                    "Redo"
                </dyn:button>
                <dyn:button
                    on:click={
                        let editable_doc = self.editable_doc.document.clone();
                        move |_ev: Event| {
                            editable_doc.set(Document::from(""));
                        }
                    }
                >
                    "Clear"
                </dyn:button>

                <div class="panes">
                    <div class="editor">{self.editable_doc.view()}</div>
                    <div class="secondary">
                        <div class="buttons">
                            <dyn:button on:click={let mode = secondary_pane_mode.clone(); move |_ev: Event| mode.set(SecondPane::Preview)}>"Preview"</dyn:button>
                            <dyn:button on:click={let mode = secondary_pane_mode.clone(); move |_ev: Event| mode.set(SecondPane::Json)}>"JSON"</dyn:button>
                            <dyn:button on:click={let mode = secondary_pane_mode.clone(); move |_ev: Event| mode.set(SecondPane::Rust)}>"Rust"</dyn:button>
                        </div>
                        <dyn:div class:preview={dyn_class_once()} class:hidden={secondary_pane_mode.stream().map(|mode| mode != SecondPane::Preview).boxed_local()}>
                            <dyn:commonprayer_doc doc={wc_doc_stream}></dyn:commonprayer_doc>
                        </dyn:div>
                        <dyn:textarea
                            class:json={dyn_class_once()}
                            class:hidden={secondary_pane_mode.stream().map(|mode| mode != SecondPane::Json).boxed_local()}
                            on:change={
                                let doc = self.editable_doc.document.clone();
                                move |ev: Event| {
                                    let new_json = event_target_value(ev);
                                    match serde_json::from_str::<Document>(&new_json) {
                                        Ok(document) => { doc.set(document); },
                                        Err(e) => { window().alert_with_message(&e.to_string()); }
                                    }
                                }
                            }
                        >
                            {json_stream}
                        </dyn:textarea>
                        <dyn:pre
                            class:json={dyn_class_once()}
                            class:hidden={secondary_pane_mode.stream().map(|mode| mode != SecondPane::Rust).boxed_local()}
                        >
                            {rust_stream}
                        </dyn:pre>
                    </div>
                </div>
           </main>
        }
    }
}

pub struct EditableDocument {
    pub root_doc: Behavior<Document>,
    pub document: Behavior<Document>,
    pub path: Behavior<Option<Vec<usize>>>,
}

impl From<Document> for EditableDocument {
    fn from(document: Document) -> Self {
        let document = Behavior::new(document);
        Self {
            root_doc: document.clone(),
            document,
            path: Behavior::new(None),
        }
    }
}

impl From<Behavior<Document>> for EditableDocument {
    fn from(document: Behavior<Document>) -> Self {
        Self {
            root_doc: document.clone(),
            document,
            path: Behavior::new(None),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct PathAndDocument {
    path: Vec<usize>,
    document: Document,
}

impl EditableDocument {
    pub fn view(&self) -> View {
        let metadata_open = Behavior::new(false);

        let doc_handle = self.document.clone();
        let content_view = View::ViewStream(
            self.document
                .stream()
                .map({
                    let root_doc = self.root_doc.clone();
                    move |doc| content_editing_view(&root_doc, &[], &doc_handle, &doc.content)
                })
                .boxed_local(),
        );

        fn set_condition(doc: &Behavior<Document>, condition: Option<Condition>) {
            doc.update(move |doc| doc.condition = condition.clone());
        }

        let initial_condition = self.document.get().condition;
        let show_condition = Behavior::new(initial_condition.is_some());
        let condition_editor = ConditionEditor::from(initial_condition);
        condition_editor.condition.stream().skip(1).create_effect({
            let doc = self.document.clone();
            move |new_condition| set_condition(&doc, new_condition)
        });

        let dragging = Behavior::new(false);

        view! {
            <dyn:article
                prop:draggable={dragging.stream().map(|v| if v { Some("true".into()) } else { None }).boxed_local()}
                class:droppable={dyn_class_once()}
                on:dragstart={
                    let path = self.path.clone();
                    move |ev: Event| {
                        let drag_event: DragEvent = ev.unchecked_into();
                        if let Some(data_transfer) = drag_event.data_transfer() {
                            if let Some(path) = path.get() {
                                data_transfer.set_data("application/json", &serde_json::to_string(&path).unwrap());
                            }
                        }
                    }
                }
                on:dragover=|ev: Event| {
                    let ev: DragEvent = ev.unchecked_into();
                    //ev.prevent_default();
                }
                on:drop={
                    let path = self.path.clone();
                    let doc = self.document.clone();
                    let root_doc = self.root_doc.clone();
                    move |ev: Event| {
                        let ev: DragEvent = ev.unchecked_into();
                        ev.prevent_default();
                        if let Some(data_transfer) = ev.data_transfer() {
                            if let Ok(start_path) = data_transfer.get_data("application/json") {
                                if let Some(dest_path) = path.get() {
                                    let start_path: Vec<usize> = serde_json::from_str(&start_path).unwrap();
                                    root_doc.update(|doc| {
                                        doc.move_subdocument(&start_path, &dest_path);
                                    });
                                }
                            }
                        }
                    }
                }
            >
                <div class="metadata-buttons">
                    <dyn:button
                        class:hidden={self.document.stream().map(|doc| matches!(doc.content, Content::Liturgy(_) | Content::Series(_))).boxed_local()}
                        on:click={let dragging = dragging.clone(); move |_ev: Event| dragging.set(!dragging.get())}
                    >
                        {View::ViewStream(dragging.stream().map(|dragging| if dragging { View::StaticText("Drag".into()) } else { View::StaticText("⇵".into()) }).boxed_local())}
                    </dyn:button>
                    <dyn:button
                        on:click={
                            let show_condition = show_condition.clone();
                            move |_ev: Event| show_condition.set(!show_condition.get())
                        }
                    >
                        "Condition"
                    </dyn:button>

                    <dyn:button
                        on:click={
                            let metadata_open = metadata_open.clone();
                            move |_ev: Event| metadata_open.set(!metadata_open.get())
                        }
                    >
                        "Metadata"
                    </dyn:button>

                    <dyn:button
                        class:hidden={self.path.stream().map(|path| path.is_none()).boxed_local()}
                        on:click={
                            let path = self.path.clone();
                            let root_doc = self.root_doc.clone();
                            move |_ev: Event| {
                                if let Some(path) = path.get() {
                                    root_doc.update(move |doc| { doc.remove_at_path(&path); });
                                }
                            }
                        }
                    >
                        "X"
                    </dyn:button>
                </div>

                <dyn:div class:hidden={show_condition.stream().map(|show| !show).boxed_local()}>
                    {condition_editor.view()}
                </dyn:div>

                // Edit Metadata
                <dyn:fieldset
                    class:metadata={dyn_class_once()}
                    class:hidden={metadata_open.stream().map(|open| !open).boxed_local()}
                >
                    // Label
                    <label>
                        "Label"
                        <dyn:input type={dyn_attr_once("text")}
                            on:change={
                                let doc = self.document.clone();
                                move |ev| {
                                    let v = event_target_value(ev);
                                    doc.update(move |doc| {
                                        if v.is_empty() {
                                            doc.label = None;
                                        } else {
                                            doc.label = Some(v.clone());
                                        }
                                    })
                                }
                            }
                        />
                    </label>

                    // Subtitle
                    <label>
                        "Subtitle"
                        <dyn:input type={dyn_attr_once("text")}
                            on:change={
                                let doc = self.document.clone();
                                move |ev| {
                                    let v = event_target_value(ev);
                                    doc.update(move |doc| {
                                        if v.is_empty() {
                                            doc.subtitle = None;
                                        } else {
                                            doc.subtitle = Some(v.clone());
                                        }
                                    })
                                }
                            }
                        />
                    </label>

                     <label>
                        "Language"
                        <dyn:input type={dyn_attr_once("text")}
                            on:change={
                                let doc = self.document.clone();
                                move |ev| {
                                    let v = event_target_value(ev);
                                    doc.update(move |doc| {
                                        if v.is_empty() {
                                            doc.subtitle = None;
                                        } else {
                                            doc.subtitle = Some(v.clone());
                                        }
                                    })
                                }
                            }
                        />
                        <dyn:select
                            on:change={
                                let doc = self.document.clone();
                                move |ev| {
                                    let v = Language::from_str(&event_target_value(ev)).unwrap();
                                    doc.update(move |doc| {
                                        doc.language = v;
                                    })
                                }
                            }
                        >
                            {View::Fragment(
                                Language::iter()
                                    .map(|variant| view! { <option>{variant.to_string()}</option> })
                                    .collect()
                            )}
                        </dyn:select>
                    </label>

                    // TODO: Source
                    <div class="source">
                        <label>
                            "Source"
                            <dyn:select
                                prop:value={self.document.stream().map(|doc| doc.source.map(|reference| { let s: &'static str = reference.source.into(); s.to_string()})).boxed_local()}
                                on:change={
                                    let doc = self.document.clone();
                                    move |ev| {
                                        let v = Source::from_str(&event_target_value(ev)).unwrap();
                                        doc.update(move |doc| {
                                            if let Some(mut reference) = doc.source {
                                                reference.source = v;
                                            } else {
                                                doc.source = Some(Reference {
                                                    source: v,
                                                    page: 0
                                                })
                                            }
                                        })
                                    }
                                }
                            >
                                {View::Fragment(
                                    Source::iter()
                                        .map(|variant| { let s: &'static str = variant.into(); view! { <option>{s}</option> } })
                                        .collect()
                                )}
                            </dyn:select>
                        </label>
                        <label>
                            "Page"
                            <dyn:input
                                type={dyn_attr_once("number")}
                                prop:value={self.document.stream().map(|doc| doc.source.map(|reference| reference.page.to_string())).boxed_local()}
                                on:change={
                                    let doc = self.document.clone();
                                    move |ev| {
                                        let v = event_target_value(ev).parse().unwrap();
                                        doc.update(move |doc| {
                                            if let Some(mut reference) = doc.source {
                                                reference.page = v;
                                            } else {
                                                doc.source = Some(Reference {
                                                    source: Source::default(),
                                                    page: v
                                                })
                                            }
                                        })
                                    }
                                }
                            />
                        </label>
                    </div>
                    // TODO: alternate_sources
                    // TODO: Status

                    // Display
                    <label>
                        "Display"
                        <dyn:select on:change={
                            let doc = self.document.clone();
                            move |ev: Event| {
                                let v = event_target_value(ev);
                                doc.update(move |doc| match v.as_str() {
                                    "Always" => doc.display = Show::Always,
                                    "Hidden" => doc.display = Show::Hidden,
                                    "TemplateOnly" => doc.display = Show::TemplateOnly,
                                    "CompiledOnly" => doc.display = Show::CompiledOnly,
                                    _ => doc.display = Show::Always
                                });
                            }
                        }>
                            <option>"Always"</option>
                            <option>"Hidden"</option>
                            <option>"TemplateOnly"</option>
                            <option>"CompiledOnly"</option>
                        </dyn:select>
                    </label>

                    // TODO: Version

                    // Version Label
                    <label>
                        "Version Label"
                        <dyn:input type={dyn_attr_once("text")}
                            on:change={
                                let doc = self.document.clone();
                                move |ev| {
                                    let v = event_target_value(ev);
                                    doc.update(move |doc| {
                                        if v.is_empty() {
                                            doc.version_label = None;
                                        } else {
                                            doc.version_label = Some(v.clone());
                                        }
                                    })
                                }
                            }
                        />
                    </label>

                    // Optional
                    <label class="horizontal">
                        "Optional"
                        <dyn:input type={dyn_attr_once("checkbox")}
                            on:change={
                                let doc = self.document.clone();
                                move |ev: Event| {
                                    let checked = event_target_checked(ev);
                                    if checked {
                                        doc.update(|doc| doc.optional = true);
                                    } else {
                                        doc.update(|doc| doc.optional = false);
                                    }
                                }
                            }
                        />
                    </label>

                    // Explainer
                    <label>
                        "Explainer"
                        <dyn:textarea
                            on:change={
                                let doc = self.document.clone();
                                move |ev| {
                                    let v = event_target_value(ev);
                                    doc.update(move |doc| {
                                        if v.is_empty() {
                                            doc.explainer = None;
                                        } else {
                                            doc.explainer = Some(v.clone());
                                        }
                                    })
                                }
                            }
                        >
                            {self.document.stream().map(|doc| doc.explainer.unwrap_or_default()).boxed_local()}
                        </dyn:textarea>
                    </label>

                    // Tags
                     <label>
                        "Tags"
                        <dyn:input type={dyn_attr_once("text")}
                            on:change={
                                let doc = self.document.clone();
                                move |ev: Event| {
                                    let v = event_target_value(ev);
                                    doc.update(move |doc| {
                                        if v.is_empty() {
                                            doc.tags = Vec::new();
                                        } else {
                                            doc.tags = v.split(',').map(|tag| tag.trim().to_string()).collect();
                                        }
                                    })
                                }
                            }
                        />
                    </label>
                </dyn:fieldset>

                // Edit Content
                <div class="content">
                    {content_view}
                </div>
            </dyn:article>
        }
    }

    pub fn stream(&self) -> impl Stream<Item = Document> {
        self.document.stream()
    }
}

pub fn dyn_class_once() -> Pin<Box<dyn Stream<Item = bool>>> {
    futures::stream::once(async { true }).boxed_local()
}

pub fn dyn_attr_once(value: &str) -> Pin<Box<dyn Stream<Item = String>>> {
    let value = value.to_string();
    futures::stream::once(async move { value }).boxed_local()
}
