use crate::EditableDocument;
use crate::{dyn_attr_once, dyn_class_once};
use futures::StreamExt;
use leptos::*;
use liturgy::*;
use std::str::FromStr;
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn content_editing_view(
    root_doc: &Behavior<Document>,
    path: &[usize],
    doc: &Behavior<Document>,
    content: &Content,
) -> View {
    let content_body = match content {
        Content::Empty => View::Empty,
        Content::Antiphon(content) => edit_antiphon(path, doc, content),
        Content::BiblicalCitation(content) => edit_biblical_citation(path, doc, content),
        Content::Choice(content) => edit_choice(root_doc, path, doc, content),
        Content::Heading(content) => edit_heading(path, doc, content),
        Content::Litany(content) => edit_litany(path, doc, content),
        Content::Liturgy(content) => edit_liturgy(root_doc, path, doc, content),
        Content::Preces(content) => edit_preces(path, doc, content),
        Content::ResponsivePrayer(content) => edit_responsive_prayer(path, doc, content),
        Content::Rubric(content) => edit_rubric(path, doc, content),
        Content::Sentence(content) => edit_sentence(path, doc, content),
        Content::Series(content) => edit_series(root_doc, path, doc, content),
        Content::Text(content) => edit_text(path, doc, content),
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
        "Biblical Citation" => Content::BiblicalCitation(BiblicalCitation::from(curr)),
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
        "Heading" => Content::Heading(Heading::from(curr)),
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

macro_rules! update_field {
    ($root_doc:ident, $content:ident, $update_statement:expr) => {{
        let root_doc = $root_doc.clone();
        let content = $content.clone();
        move |_ev: Event| {
            content.update($update_statement);
            let new_content = content.get();
            root_doc.update(|root_doc| root_doc.content = Content::from(new_content.clone()));
        }
    }};
}

macro_rules! update_field_with_value {
    ($root_doc:ident, $content:ident, $update_statement:expr) => {{
        let root_doc = $root_doc.clone();
        let content = $content.clone();
        move |ev: Event| {
            content.update($update_statement(ev));
            let new_content = content.get();
            root_doc.update(|root_doc| root_doc.content = Content::from(new_content.clone()));
        }
    }};
}

macro_rules! update_optional_string_field {
    ($root_doc:ident, $content:ident, $content_type:ty, $update_field:expr) => {{
        let root_doc = $root_doc.clone();
        let content = $content.clone();
        move |ev: Event| {
            let val = event_target_value(ev);
            let new_value = if val.is_empty() {
                None
            } else {
                Some(val.clone())
            };
            content.update(move |content: &mut $content_type| {
                $update_field(content, new_value.clone())
            });
            let new_content = content.get();
            root_doc.update(|root_doc| root_doc.content = Content::from(new_content.clone()));
        }
    }};
}

macro_rules! update_nth_item_in_vec {
    ($root_doc:ident, $content:ident, $content_type:ty, $idx:ident, $update_field:expr) => {{
        let root_doc = $root_doc.clone();
        let content = $content.clone();
        move |ev: Event| {
            let val = event_target_value(ev);

            content.update(move |content: &mut $content_type| {
                let mut vec = content.clone().into_vec();
                let mut line = vec.get($idx).cloned().unwrap_or_default();
                if let Some(l) = vec.get_mut($idx) {
                    $update_field(&mut line, val.clone());
                    *l = line;
                };
                *content = <$content_type>::from(vec);
            });
            let new_content = content.get();
            root_doc.update(|root_doc| root_doc.content = Content::from(new_content.clone()));
        }
    }};
}

fn edit_antiphon(path: &[usize], doc: &Behavior<Document>, content: &Antiphon) -> View {
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

fn edit_biblical_citation(
    path: &[usize],
    doc: &Behavior<Document>,
    content: &BiblicalCitation,
) -> View {
    let show_intro = Behavior::new(content.intro.is_some());
    let intro = if let Some(intro) = &content.intro {
        EditableDocument::from(Document::from(intro.clone()))
    } else {
        EditableDocument::from(Document::from(""))
    };

    let content = Behavior::new(content.clone());

    fn set_intro(
        doc: &Behavior<Document>,
        content: &Behavior<BiblicalCitation>,
        intro: Option<BiblicalReadingIntro>,
    ) {
        content.update(move |content| {
            content.intro = intro.clone();
        });
        let new_content = content.get();
        doc.update(move |doc| doc.content = Content::BiblicalCitation(new_content.clone()));
    }

    intro.document.stream().skip(1).create_effect({
        let doc = doc.clone();
        let content = content.clone();
        move |intro_doc| set_intro(&doc, &content, Some(BiblicalReadingIntro::from(intro_doc)))
    });

    view! {
        <>
            <label>
                "Citation"
                <dyn:input
                    on:change=update_field_with_value!(doc, content,
                        |ev| {
                            let val = event_target_value(ev);
                            move |content: &mut BiblicalCitation| content.citation = val.clone()
                        }
                    )
                    prop:value={content.stream().map(|v| Some(v.citation)).boxed_local()}
                />
            </label>
            <label class="horizontal">
                "Introduction"
                <dyn:input type={dyn_attr_once("checkbox")}
                    prop:checked={show_intro.stream().map(|checked| if checked { Some("checked".to_string())} else { None }).boxed_local()}
                    on:change={
                        let doc = doc.clone();
                        let show_intro = show_intro.clone();
                        move |ev: Event| {
                            let checked = event_target_checked(ev);
                            show_intro.set(checked);
                            if !checked {
                                set_intro(&doc, &content, None);
                            }
                        }
                    }
                />
            </label>
            <dyn:div class:intro={dyn_class_once()} class:hidden={show_intro.stream().map(|show| !show).boxed_local()}>
                {intro.view()}
            </dyn:div>
        </>
    }
}

fn edit_choice(
    first_root_doc: &Behavior<Document>,
    path: &[usize],
    doc: &Behavior<Document>,
    choice: &Choice,
) -> View {
    let content = Behavior::new(choice.clone());
    let showing_choice = Behavior::new(choice.selected.to_string());

    let list = NaiveList::new(
        |children| {
            view! {
                <div class="choice-children">
                    {children}
                </div>
            }
        },
        content.stream().map(|series| series.options),
        {
            let root_doc = doc.clone();
            let path = path.to_vec();
            let first_root_doc = first_root_doc.clone();
            move |(idx, doc)| {
                let mut editable_doc = EditableDocument::from(doc);
                editable_doc.root_doc = first_root_doc.clone();
                let mut path = path.clone();
                path.push(idx);
                editable_doc.path.set(Some(path.clone()));
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
                    <dyn:div>{editable_doc.view()}</dyn:div>
                }
            }
        },
    );

    view! {
        <>
            <dyn:select
                prop:value={showing_choice.stream().map(Some).boxed_local()}
                on:change=move |ev: Event| showing_choice.set(event_target_value(ev))
            >
                {View::Fragment(
                    choice.options.iter().enumerate()
                        .map(|(idx, child)| view! { <option value={idx.to_string()}>{choice.option_label(child, idx)}</option> })
                        .collect()
                )}
            </dyn:select>
            {list.view()}
            <dyn:button
                on:click=update_field!(doc, content, |content| content.push(Document::from("")))
            >
                "Add Option"
            </dyn:button>
        </>
    }
}

fn edit_heading(path: &[usize], doc: &Behavior<Document>, content: &Heading) -> View {
    let content = Behavior::new(content.clone());
    view! {
        <>
            <dyn:select
                prop:value={content.stream().map(|variant| { let s: &'static str = variant.into(); Some(s.to_string())}).boxed_local()}
                on:change={
                    let content = content.clone();
                    move |ev: Event| {
                        let val = Heading::from_str(&event_target_value(ev)).unwrap();
                        content.set(val);
                    }
                }
            >
                {View::Fragment(
                    Heading::iter()
                        .map(|variant| {
                            let s: &'static str = variant.into();
                            view! { <option>{s}</option> }
                        })
                        .collect()
                )}
            </dyn:select>
            <dyn:select
                class:hidden={content.stream().map(|variant| !matches!(variant, Heading::Text(_, _))).boxed_local()}
                prop:value={content.stream()
                    .map(|variant| {
                        if let Heading::Text(level, _) = variant {
                            let s: &'static str = level.into();
                            Some(level.to_string())
                        } else {
                            None
                        }
                    })
                    .boxed_local()
                }
                on:change={
                    let content = content.clone();
                    let doc = doc.clone();
                    move |ev: Event| {
                        let val = HeadingLevel::from_str(&event_target_value(ev)).unwrap();
                        content.update(|content| if let Heading::Text(_, text) = content {
                            *content = Heading::Text(val, text.to_string());
                        });
                        let new_content = content.get();
                        doc.update(move |doc| doc.content = Content::Heading(new_content.clone()));
                    }
                }
            >
                {View::Fragment(
                    HeadingLevel::iter()
                        .map(|variant| {
                            let s: &'static str = variant.into();
                            view! { <option>{s}</option> }
                        })
                        .collect()
                )}
            </dyn:select>
            <dyn:input
                class:hidden={content.stream().map(|variant| !matches!(variant, Heading::Text(_, _))).boxed_local()}
                prop:value={content.stream()
                    .map(|variant| {
                        if let Heading::Text(_, text) = variant {
                            Some(text)
                        } else {
                            None
                        }
                    })
                    .boxed_local()
                }
                on:change={
                    let doc = doc.clone();
                    move |ev: Event| {
                        let val = event_target_value(ev);
                        content.update(move |content| if let Heading::Text(level, _) = content {
                            *content = Heading::Text(*level, val.clone());
                        });
                        let new_content = content.get();
                        doc.update(move |doc| doc.content = Content::Heading(new_content.clone()));
                    }
                }
            />
        </>
    }
}

fn edit_litany(
    path: &[usize],
    root_doc: &Behavior<Document>,
    content: &Litany,
) -> View {
    let content = Behavior::new(content.clone());
    let list = NaiveList::new(
        |children| {
            view! {
                <p class="litany">
                    {children}
                </p>
            }
        },
        content.stream().map(move |preces| preces.into_vec()),
        {
            let root_doc = root_doc.clone();
            let content = content.clone();
            move |(idx, line)| {
                view! {
                    <div>
                        <dyn:input type="text"
                            value={dyn_attr_once(&line)}
                            on:change=update_nth_item_in_vec!(root_doc, content, Litany, idx, |line: &mut String, val: String| *line = val)
                        />
                        <dyn:button
                            on:click=update_field!(root_doc, content, |content| {content.remove_at_index(idx); })
                        >
                            "x"
                        </dyn:button>
                    </div>
                }
            }
        },
    );

    view! {
        <>
            <label>
                "Response"
                <dyn:input
                    prop:value={content.stream().map(|litany| Some(litany.response)).boxed_local()}
                    on:change=update_field_with_value!(root_doc, content,
                        |ev| {
                            let val = event_target_value(ev);
                            move |content: &mut Litany| content.response = val.clone()
                        }
                    )
                />
            </label>
            {list.view()}
            <dyn:button
                on:click=update_field!(root_doc, content, |content| content.push(String::new()))
            >
                "Add"
            </dyn:button>
        </>
    }
}

fn edit_liturgy(
    root_doc: &Behavior<Document>,
    path: &[usize],
    doc: &Behavior<Document>,
    content: &Liturgy,
) -> View {
    let series_doc = Behavior::new(Document::from(content.body.clone()));
    series_doc.stream().skip(1).create_effect({
        let doc = doc.clone();
        move |new_series| {
            doc.update(|doc| {
                if let (Content::Series(series), Content::Liturgy(liturgy)) =
                    (&new_series.content, &mut doc.content)
                {
                    liturgy.body = series.clone();
                } else {
                    doc.content = new_series.content.clone();
                }
            });
        }
    });
    edit_series(root_doc, path, &series_doc, &content.body)
}

fn edit_rubric(path: &[usize], doc: &Behavior<Document>, content: &Rubric) -> View {
    let content = Behavior::new(content.clone());
    view! {
        <dyn:button
            on:click={
                let content = content.clone();
                let doc = doc.clone();
                move |_ev: Event| {
                    let text = content.get();
                    let new_text = text.text.replace("\n\n", "\\n\\n").replace("\n", " ").replace("\\n\\n", "\n\n");
                    let new_text = new_text.trim();
                    content.update(move |text| {
                        text.text = new_text.to_string();
                    });
                    let new_content = content.get();
                    doc.update(|doc| doc.content = Content::Rubric(new_content.clone()));
                }
            }
        >
            "\\n"
        </dyn:button>
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

fn edit_sentence(path: &[usize], doc: &Behavior<Document>, content: &Sentence) -> View {
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
                    on:change=update_optional_string_field!(doc, content, Sentence, |sentence: &mut Sentence, new_value| sentence.citation = new_value)
                />
            </label>
        </>
    }
}

fn edit_preces(path: &[usize], root_doc: &Behavior<Document>, content: &Preces) -> View {
    let content = Behavior::new(content.clone());
    let list = NaiveList::new(
        |children| {
            view! {
                <table class="preces">
                    {children}
                </table>
            }
        },
        content.stream().map(move |preces| preces.into_vec()),
        {
            let root_doc = root_doc.clone();
            let content = content.clone();
            move |(idx, (v, r))| {
                view! {
                    <tr>
                        <td>
                            <dyn:input type="text"
                                value={dyn_attr_once(&v)}
                                on:change=update_nth_item_in_vec!(root_doc, content, Preces, idx, |line: &mut (String, String), val: String| line.0 = val)
                            />
                        </td>
                        <td>
                            <dyn:input type="text"
                                value={dyn_attr_once(&r)}
                                on:change=update_nth_item_in_vec!(root_doc, content, Preces, idx, |line: &mut (String, String), val: String| line.1 = val)
                            />
                        </td>
                    </tr>
                }
            }
        },
    );

    view! {
        <>
            {list.view()}
            <dyn:button
                on:click=update_field!(root_doc, content, |content| content.push((String::new(), String::new())))
            >
                "Add"
            </dyn:button>
        </>
    }
}

fn edit_responsive_prayer(
    path: &[usize],
    root_doc: &Behavior<Document>,
    content: &ResponsivePrayer,
) -> View {
    let content = Behavior::new(content.clone());
    let list = NaiveList::new(
        |children| {
            view! {
                <p class="responsive-prayer">
                    {children}
                </p>
            }
        },
        content.stream().map(move |preces| preces.into_vec()),
        {
            let root_doc = root_doc.clone();
            let content = content.clone();
            move |(idx, line)| {
                view! {
                    <div>
                        <dyn:input type="text"
                            value={dyn_attr_once(&line)}
                            on:change=update_nth_item_in_vec!(root_doc, content, ResponsivePrayer, idx, |line: &mut String, val: String| *line = val)
                        />
                        <dyn:button
                            on:click=update_field!(root_doc, content, |content| {content.remove_at_index(idx); })
                        >
                            "x"
                        </dyn:button>
                    </div>
                }
            }
        },
    );

    view! {
        <>
            {list.view()}
            <dyn:button
                on:click=update_field!(root_doc, content, |content| content.push(String::new()))
            >
                "Add"
            </dyn:button>
        </>
    }
}

macro_rules! quick_edit_button {
    ($root:ident, $content:ident, $quick_edit:ident, $textarea:ident, $content_type:ty, $label:expr) => {
        view! {
            <dyn:button
                on:click={
                    let root = $root.clone();
                    let content = $content.clone();
                    let quick_edit = $quick_edit.clone();
                    let textarea = $textarea.clone();
                    move |ev: Event| {
                        ev.prevent_default();
                        if let Ok(Some(selection)) = window().get_selection() {
                            // remove text from textarea
                            if let Some(textarea) = textarea.get() {
                                let textarea: web_sys::HtmlTextAreaElement = textarea.unchecked_into();
                                let selection_start = textarea.selection_start().unwrap().unwrap_or(0) as usize;
                                let selection_end = textarea.selection_end().unwrap().unwrap_or(0) as usize;
                                let value = textarea.value();

                                // clear textarea
                                let before_selection = value[0..selection_start].to_string();
                                let after_selection = value[selection_end..].to_string();
                                let new_value = format!("{before_selection}{after_selection}");
                                let new_value = new_value.trim();
                                textarea.set_value(&new_value);
                                window().local_storage().unwrap().unwrap().set("quick-edit", &new_value);
                                quick_edit.set(new_value.to_string());

                                // update content
                                let text = value[selection_start..selection_end].to_string();
                                content.update(move |content| content.push(Document::from(<$content_type>::from(Text::from(text.clone())))));
                                let new_content = content.get();
                                root.update(|root_doc| root_doc.content = Content::from(new_content.clone()));
                            }
                        }
                    }
                }
            >
                {$label}
            </dyn:button>
        }
    };
}

fn edit_series(
    first_root_doc: &Behavior<Document>,
    path: &[usize],
    root_doc: &Behavior<Document>,
    content: &Series,
) -> View {
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
            let path = path.to_vec();
            let first_root_doc = first_root_doc.clone();
            move |(idx, doc)| {
                let mut editable_doc = EditableDocument::from(doc);
                editable_doc.root_doc = first_root_doc.clone();
                let mut path = path.clone();
                path.push(idx);
                editable_doc.path.set(Some(path.clone()));
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

                let drag_inside = Behavior::new(false);

                view! {
                    <li>{editable_doc.view()}</li>
                }
            }
        },
    );

    let storage = window().local_storage().unwrap().unwrap();
    let quick_edit = Behavior::new(storage.get("quick-edit").unwrap().unwrap_or_default());
    let textarea: Behavior<Option<web_sys::Element>> = Behavior::new(None);

    view! {
        <>
            <div class="quick-edit">
                <h3>"Quick Edit"</h3>
                <div class="buttons">
                    {quick_edit_button!(root_doc, content, quick_edit, textarea, Text, "Text")}
                    {quick_edit_button!(root_doc, content, quick_edit, textarea, Heading, "Heading")}
                    {quick_edit_button!(root_doc, content, quick_edit, textarea, Rubric, "Rubric")}
                    {quick_edit_button!(root_doc, content, quick_edit, textarea, Preces, "Preces")}
                    {quick_edit_button!(root_doc, content, quick_edit, textarea, ResponsivePrayer, "ResponsivePrayer")}
                    {quick_edit_button!(root_doc, content, quick_edit, textarea, Litany, "Litany")}
                </div>
                <dyn:textarea
                    dom:ref={&textarea}
                    prop:value={quick_edit.stream().map(Some).boxed_local()}
                    on:change=move |ev: Event| {
                        let val = event_target_value(ev);
                        storage.set("quick-edit", &val);
                        quick_edit.set(val);
                    }
                ></dyn:textarea>
            </div>
            {list.view()}
            <dyn:button
                on:click=update_field!(root_doc, content, |content| content.push(Document::from("")))
            >
                "Add"
            </dyn:button>
        </>
    }
}

fn edit_text(path: &[usize], doc: &Behavior<Document>, text: &Text) -> View {
    let content = Behavior::new(text.clone());
    view! {
        <>
            <dyn:button
                on:click={
                    let content = content.clone();
                    let doc = doc.clone();
                    move |_ev: Event| {
                        let text = content.get();
                        let response = if text.text.ends_with("Amen.") {
                            Some("Amen.".to_string())
                        } else if text.text.ends_with("Amén.") {
                            Some("Amén.".to_string())
                        } else {
                            None
                        };
                        let new_text = text.text.replace("Amen.", "").replace("Amén.", "").replace("\n", " ");
                        let new_text = new_text.trim();
                        content.update(move |text| {
                            text.text = new_text.to_string();
                            text.response = response.clone();
                        });
                        let new_content = content.get();
                        doc.update(|doc| doc.content = Content::Text(new_content.clone()));
                    }
                }
            >
                "Trim"
            </dyn:button>
            <dyn:textarea
                on:change=update_field_with_value!(doc, content,
                    |ev| {
                        let val = event_target_value(ev);
                        move |content: &mut Text| content.text = val.clone()
                    }
                )
                on:keyup=autogrow_textarea
                on:focus=autogrow_textarea
            >
                {content.get().text}
            </dyn:textarea>
            <label>
                "Response"
                <dyn:input
                    prop:value={content.stream().map(|text| text.response).boxed_local()}
                    on:change=update_optional_string_field!(doc, content, Text, |text: &mut Text, new_value| text.response = new_value)
                />
            </label>
            <label>
                "Display Format"
                <dyn:select
                    prop:value={content.stream().map(|text| Some(text.display_format.to_string())).boxed_local()}
                    on:change={
                        let content = content.clone();
                        let doc = doc.clone();
                        move |ev: Event| {
                            let val = DisplayFormat::from_str(&event_target_value(ev)).unwrap();
                            content.update(move |content| content.display_format = val);
                            let new_content = content.get();
                            doc.update(|doc| doc.content = Content::from(new_content.clone()));
                        }
                    }
                >
                    {View::Fragment(
                        DisplayFormat::iter()
                            .map(|variant| view! { <option>{variant.to_string()}</option> })
                            .collect()
                    )}
                </dyn:select>
            </label>
        </>
    }
}
