use std::pin::Pin;

use crate::{condition::ConditionEditor, content::content_editing_view};
use futures::{Stream, StreamExt};
use leptos::*;
use liturgy::*;

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
                    </div>
                </div>
           </main>
        }
    }
}

pub struct EditableDocument {
    pub document: Behavior<Document>,
}

impl From<Document> for EditableDocument {
    fn from(document: Document) -> Self {
        Self {
            document: Behavior::new(document),
        }
    }
}

impl From<Behavior<Document>> for EditableDocument {
    fn from(document: Behavior<Document>) -> Self {
        Self { document }
    }
}

impl EditableDocument {
    pub fn view(&self) -> View {
        let metadata_open = Behavior::new(false);

        let doc_handle = self.document.clone();
        let content_view = View::ViewStream(
            self.document
                .stream()
                .map(move |doc| content_editing_view(&doc_handle, &doc.content))
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

        view! {
            <article>
                // Toggle Metadata
                <dyn:button class={dyn_attr_once("show-metadata")}
                    on:click={
                        let metadata_open = metadata_open.clone();
                        move |_ev: Event| metadata_open.set(!metadata_open.get())
                    }
                >
                    "Toggle Metadata"
                </dyn:button>

                <div class="condition-field">
                    <label class="condition">
                        "Conditional"
                        <dyn:input type={dyn_attr_once("checkbox")}
                            prop:checked={condition_editor.condition.stream().map(|condition| condition.map(|_| "checked".to_string())).boxed_local()}
                            on:change={
                                let show_condition = show_condition.clone();
                                let doc = self.document.clone();
                                move |ev: Event| {
                                    let checked = event_target_checked(ev);
                                    show_condition.set(checked);
                                    if !checked {
                                        set_condition(&doc, None);
                                    }
                                }
                            }
                        />
                    </label>
                    <dyn:div class:hidden={show_condition.stream().map(|show| !show).boxed_local()}>
                        {condition_editor.view()}
                    </dyn:div>
                </div>

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

                    // TODO: Language
                    // TODO: Source
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
            </article>
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
