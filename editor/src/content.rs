use crate::EditableDocument;
use futures::StreamExt;
use leptos::*;
use liturgy::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn content_editing_view(doc: &Behavior<Document>, content: &Content) -> View {
    let content_body = match content {
        Content::Empty => View::Empty,
        Content::Antiphon(content) => edit_antiphon(doc, content),
        Content::Liturgy(content) => edit_liturgy(doc, content),
        Content::Rubric(content) => edit_rubric(doc, content),
        Content::Sentence(content) => edit_sentence(doc, content),
        Content::Series(content) => edit_series(doc, content),
        Content::Text(content) => edit_text(doc, content),
        _ => view! {
            <p>"TODO Content Type"</p>
        },
    };

    view! {
        <div class="content">
            <label>
                "Content Type"
                <dyn:select
                    on:change={
                        let doc = doc.clone();
                        move |ev: Event| {
                            let new_type = event_target_value(ev);
                            let content = swap_content(&doc, &new_type);
                            doc.update(|doc| doc.content = content.clone());
                        }
                    }
                    prop:value={doc.stream().map(|doc| Some(content_type_as_str(&doc.content).to_string())).boxed_local()}
                >
                    <option value="Empty">"—"</option>
                    <option>"Antiphon"</option>
                    <option>"Biblical Citation"</option>
                    <option>"Biblical Reading"</option>
                    <option>"Canticle"</option>
                    <option>"Canticle Table Entry"</option>
                    <option>"Category"</option>
                    <option>"Choice"</option>
                    <option>"Collect of the Day"</option>
                    <option>"Document Link"</option>
                    <option>"Gloria Patri"</option>
                    <option>"Heading"</option>
                    <option>"Hymn Link"</option>
                    <option>"Invitatory"</option>
                    <option>"Lectionary Reading"</option>
                    <option>"Litany"</option>
                    <option>"Liturgy"</option>
                    <option>"Parallel"</option>
                    <option>"Preces"</option>
                    <option>"Psalm"</option>
                    <option>"Psalm Citation"</option>
                    <option>"Responsive Prayer"</option>
                    <option>"Rubric"</option>
                    <option>"Sentence"</option>
                    <option>"Series"</option>
                    <option>"Text"</option>
                </dyn:select>
            </label>
            {content_body}
        </div>
    }
}

fn swap_content(doc: &Behavior<Document>, new_type: &str) -> Content {
    let curr = doc.get().content;
    match new_type {
        "Empty" => Content::Empty,
        "Antiphon" => Content::Antiphon(Antiphon::from(curr)),
        //"Biblical Citation" => Content::BiblicalCitation(BiblicalCitation::from(curr)),
        //"Biblical Reading" => Content::BiblicalReading(BiblicalReading::from(curr)),
        //"Canticle" => Content::Canticle(Canticle::from(curr)),
        //"Canticle Table Entry" => Content::CanticleTableEntry(CanticleTableEntry::from(curr)),
        //"Category" => Content::Category(Category::from(curr)),
        "Choice" => Content::Choice(Choice::from(curr)),
        "Collect of the Day" => Content::CollectOfTheDay {
            allow_multiple: false,
        },
        //"Document Link" => Content::DocumentLink(DocumentLink::from(curr)),
        //"Gloria Patri" => Content::GloriaPatri(GloriaPatri::from(curr)),
        //"Heading" => Content::Heading(Heading::from(curr)),
        //"Hymn Link" => Content::HymnLink(Hymn Link::from(curr)),
        //"Invitatory" => Content::Invitatory(Invitatory::from(curr)),
        //"Lectionary Reading" => Content::LectionaryReading(LectionaryReading::from(curr)),
        "Litany" => Content::Litany(Litany::from(curr)),
        "Liturgy" => Content::Liturgy(Liturgy::from(curr)),
        "Parallel" => Content::Parallel(Parallel::from(curr)),
        "Preces" => Content::Preces(Preces::from(curr)),
        //"Psalm" => Content::Psalm(Psalm::from(curr)),
        //"Psalm Citation" => Content::PsalmCitation(Psalm Citation::from(curr)),
        "Responsive Prayer" => Content::ResponsivePrayer(ResponsivePrayer::from(curr)),
        "Rubric" => Content::Rubric(Rubric::from(curr)),
        "Sentence" => Content::Sentence(Sentence::from(curr)),
        "Series" => Content::Series(Series::from(curr)),
        "Text" => Content::Text(Text::from(curr)),
        // All others => Empty
        _ => Content::Empty,
    }
}

fn content_type_as_str(content: &Content) -> &'static str {
    match content {
        Content::Series(_) => "Series",
        Content::Parallel(_) => "Parallel",
        Content::Choice(_) => "Choice",
        Content::Category(_) => "Category",
        Content::CollectOfTheDay { .. } => "Collect of the Day",
        Content::Empty => "Empty",
        Content::Error(_) => "Error",
        Content::Antiphon(_) => "Antiphon",
        Content::BiblicalCitation(_) => "Biblical Citation",
        Content::BiblicalReading(_) => "Biblical Reading",
        Content::Canticle(_) => "Canticle",
        Content::CanticleTableEntry(_) => "Canticle Table Entry",
        Content::DocumentLink(_, _, _, _) => "Document Link",
        Content::GloriaPatri(_) => "Gloria Patri",
        Content::Heading(_) => "Heading",
        Content::HymnLink(_) => "Hymn Link",
        Content::Invitatory(_) => "Invitatory",
        Content::LectionaryReading(_) => "Lectionary Reading",
        Content::Litany(_) => "Litany",
        Content::Liturgy(_) => "Liturgy",
        Content::Preces(_) => "Preces",
        Content::Psalm(_) => "Psalm",
        Content::PsalmCitation(_) => "Psalm Citation",
        Content::ResponsivePrayer(_) => "Responsive Prayer",
        Content::Rubric(_) => "Rubric",
        Content::Sentence(_) => "Sentence",
        Content::Text(_) => "Text",
    }
}

fn autogrow_textarea(ev: Event) {
    let target: HtmlElement = ev.target().unwrap().unchecked_into();
    if target.client_height() < target.scroll_height() {
        let style = target.style();
        style
            .set_property("height", &format!("{}px", target.scroll_height()))
            .expect("couldn't set textarea height");
    }
}

fn edit_antiphon(doc: &Behavior<Document>, content: &Antiphon) -> View {
    let content = Behavior::new(content.clone());
    view! {
        <dyn:input
            on:change={
                let doc = doc.clone();
                move |ev: Event| {
                    let v = event_target_value(ev);
                    doc.update(move |doc| doc.content = Content::Antiphon(Antiphon::from(v.clone())))
                }
            }
            prop:value={content.stream().map(|v| Some(v.to_string())).boxed_local()}
        />
    }
}

fn edit_liturgy(doc: &Behavior<Document>, content: &Liturgy) -> View {
    edit_series(doc, &content.body)
}

fn edit_rubric(doc: &Behavior<Document>, content: &Rubric) -> View {
    let content = Behavior::new(content.clone());
    view! {
        <dyn:textarea
            on:change={
                let doc = doc.clone();
                move |ev: Event| {
                    let v = event_target_value(ev);
                    doc.update(move |doc| doc.content = Content::Rubric(Rubric::from(v.clone())))
                }
            }
            on:keyup=autogrow_textarea
            on:focus=autogrow_textarea
        >
            {content.get().to_string()}
        </dyn:textarea>
    }
}

fn edit_sentence(doc: &Behavior<Document>, content: &Sentence) -> View {
    let content = Behavior::new(content.clone());
    view! {
        <>
            <dyn:textarea
                on:change={
                    let doc = doc.clone();
                    let content = content.clone();
                    move |ev: Event| {
                        let v = event_target_value(ev);
                        content.update(move |content| content.text = v.clone());
                        let new_content = Content::Sentence(content.get());
                        doc.update(move |doc| doc.content = new_content.clone())
                    }
                }
                on:keyup=autogrow_textarea
                on:focus=autogrow_textarea
            >
                {content.get().text}
            </dyn:textarea>
            <label>
                "Citation"
                <dyn:input
                    prop:value={content.stream().map(|sentence| sentence.citation).boxed_local()}
                    on:change={
                        let doc = doc.clone();
                        move |ev: Event| {
                            let v = event_target_value(ev);
                            if v.is_empty() {
                                content.update(|content| content.citation = None);
                            } else {
                                content.update(move |content| content.citation = Some(v.clone()));
                            }
                            let new_content = Content::Sentence(content.get());
                            doc.update(move |doc| doc.content = new_content.clone())
                        }
                    }
                />
            </label>
        </>
    }
}

fn edit_series(root_doc: &Behavior<Document>, content: &Series) -> View {
    let content = Behavior::new(content.clone());
    let list = NaiveList::new(
        |children| {
            view! {
                <ul class="series">
                    {children}
                </ul>
            }
        },
        content.stream().map(|series| series.into_vec()),
        {
            let root_doc = root_doc.clone();
            move |(idx, doc)| {
                let editable_doc = EditableDocument::from(doc);
                editable_doc.document.stream().skip(1).create_effect({
                    let root_doc = root_doc.clone();
                    move |doc| {
                        let mut root_doc_curr = root_doc.get();
                        match root_doc_curr.at_path_mut(vec![idx]) {
                            Ok(doc_at_path) => {
                                *doc_at_path = doc;
                                root_doc.set(root_doc_curr)
                            }
                            Err(e) => log(&format!("{:#?}", e)),
                        };
                    }
                });

                view! {
                    <li>{editable_doc.view()}</li>
                }
            }
        },
    );

    view! {
        <>
            {list.view()}
            <dyn:button
                on:click={
                    let root_doc = root_doc.clone();
                    move |_ev: Event| {
                        content.update(|content| content.push(Document::from("")));
                        let new_content = content.get();
                        root_doc.update(|root_doc| root_doc.content = Content::Series(new_content.clone()));
                    }
                }
            >
                "Add"
            </dyn:button>
        </>
    }
}

fn edit_text(doc: &Behavior<Document>, text: &Text) -> View {
    let content = Behavior::new(text.clone());
    view! {
        <>
            <dyn:textarea
                on:change={
                    let doc = doc.clone();
                    let content = content.clone();
                    move |ev: Event| {
                        let v = event_target_value(ev);
                        content.update(move |content| content.text = v.clone());
                        let new_content = Content::Text(content.get());
                        doc.update(move |doc| doc.content = new_content.clone())
                    }
                }
                on:keyup=autogrow_textarea
                on:focus=autogrow_textarea
            >
                {content.get().text}
            </dyn:textarea>
            <label>
                "Response"
                <dyn:input
                    prop:value={content.stream().map(|text| text.response).boxed_local()}
                    on:change={
                        let doc = doc.clone();
                        move |ev: Event| {
                            let v = event_target_value(ev);
                            if v.is_empty() {
                                content.update(|text| text.response = None);
                            } else {
                                content.update(move |text| text.response = Some(v.clone()));
                            }
                            let new_content = Content::Text(content.get());
                            doc.update(move |doc| doc.content = new_content.clone())
                        }
                    }
                />
            </label>
        </>
    }
}
