use document::{DocumentComponent, DocumentMsg};
use liturgy::*;
use lookup::{lookup_links, LookupType};
use sauron::prelude::*;
use sauron::{node, Application, Cmd, Node};

mod document;
mod fetch_reading;
mod lookup;

#[derive(Debug)]
pub enum Msg {
    SetDocument(Document),
    ChildMsg(DocumentMsg),
    SetContent(Vec<usize>, Content),
}

pub struct Viewer {
    pub document: Document,
    pub dynamic: bool,
    pub lookup_links: fn(&LookupType) -> String,
}

impl Viewer {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            dynamic: false,
            lookup_links,
        }
    }

    pub fn to_html(&self) -> String {
        let view = self.view();
        let mut buffer = String::new();
        view.render(&mut buffer).expect("failed to render document");
        buffer
    }
}

impl Default for Viewer {
    fn default() -> Self {
        Self::new()
    }
}
impl From<Document> for Viewer {
    fn from(document: Document) -> Self {
        Self {
            document,
            dynamic: false,
            lookup_links,
        }
    }
}

impl Application<Msg> for Viewer {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::none()
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg>
    where
        Self: Sized + 'static,
    {
        let cmd = match msg {
            Msg::ChildMsg(msg) => match msg {
                DocumentMsg::LoadCitation(path, citation) => {
                    let doc = self.document.at_path_mut(path.clone());
                    if let Ok(doc) = doc {
                        doc.content = Content::Text(liturgy::Text::from("..."));
                    }
                    Some(self.fetch_biblical_reading(path, &citation))
                }
                DocumentMsg::SelectOption(path, event) => {
                    let doc = self.document.at_path_mut(path);
                    if let Ok(doc) = doc {
                        if let Content::Choice(choice) = &mut doc.content {
                            let new_index = choice
                                .options
                                .iter()
                                .enumerate()
                                .find(|(index, doc)| {
                                    choice.option_label(doc, *index) == event.value
                                })
                                .map(|(index, _)| index)
                                .unwrap_or(0);
                            choice.selected = new_index;

                            // this below should work -- right now there appears to be an
                            // issue in Sauron with option values, which I can report
                            /* let new_index = event.value.parse();
                            if let Ok(new_index) = new_index {
                                choice.selected = new_index;
                            } */
                        }
                    }
                    None
                }
            },
            Msg::SetContent(path, content) => {
                if let Ok(doc) = self.document.at_path_mut(path) {
                    doc.content = content;
                };
                None
            }
            Msg::SetDocument(document) => {
                self.document = document;
                None
            }
        };
        cmd.unwrap_or_else(Cmd::none)
    }

    fn view(&self) -> Node<Msg> {
        let component = DocumentComponent {
            document: self.document.clone(),
            top_level: true,
            path: vec![],
            dynamic: self.dynamic,
            lookup_links: self.lookup_links,
        };
        node! {
            <main>
                {component.view().map_msg(Msg::ChildMsg)}
            </main>
        }
    }
}

/// Creates a `Viewer` from the JSON in `serialized_state` and replaces the DOM Element found in `query_selector`
/// with that `Viewer`.
#[wasm_bindgen]
pub fn initialize_from_json(query_selector: String, serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();

    let mut app = if let Ok(doc) = serde_json::from_str::<Document>(&serialized_state) {
        Viewer::from(doc)
    } else {
        Viewer::default()
    };
    app.dynamic = true;

    /* If there's a window (i.e., if this is running in the browser)
     * then mount the app by swapping out the <main> tag */
    match web_sys::window() {
        Some(window) => {
            let el = window
                .document()
                .expect("document not found")
                .query_selector(&query_selector)
                .expect("no element found matching the query")
                .expect("third level crash oops");

            Program::replace_mount(app, &el);
        }
        None => {
            Program::mount_to_body(app);
        }
    }
}
