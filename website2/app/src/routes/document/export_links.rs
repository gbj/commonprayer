use crate::components::{Modal, Tabs};
use crate::utils::encode_uri;
use crate::Icon;
use leptos2::*;
use liturgy::{Content, Document};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct ExportLinks {
    pub slug: String,
    pub date: String,
    #[prop]
    pub document: Document,
    modal_open: bool,
    status: Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Status {
    Idle,
    Success,
    Error(String),
}

impl Default for Status {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ExportLinksMsg {
    ModalOpen(bool),
    ChoiceChanged(String),
}

#[async_trait(?Send)]
impl State for ExportLinks {
    type Msg = ExportLinksMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            ExportLinksMsg::ModalOpen(modal_open) => {
                self.modal_open = modal_open;
            }
            ExportLinksMsg::ChoiceChanged(content) => {
                leptos2::warn(&format!("choice changed to {}", content));
                let mut parts = content.split('#');
                let path = parts.next();
                let selected = parts.next();
                if let (Some(path), Some(selected)) = (path, selected) {
                    leptos2::warn(&format!("setting choice {} to {}", path, selected));
                    let path = path.split('-').flat_map(|val| val.parse::<usize>());
                    match self.document.at_path_mut(path) {
                        Ok(document) => {
                            if let Content::Choice(ref mut choice) = &mut document.content {
                                if let Ok(selected) = selected.parse::<usize>() {
                                    choice.selected = selected;
                                }
                            }
                        }
                        Err(e) => leptos2::debug_warn(&format!(
                            "[ExportLinks::update] {}\n\ndoc was {:#?}",
                            e, self.document
                        )),
                    }
                } else {
                    leptos2::debug_warn(&format!(
                        "[ExportLinks::update] invalid path#choice value: {}",
                        content
                    ));
                }
            }
        }
        None
    }
}

impl Component for ExportLinks {
    fn view(&self) -> Host {
        // TODO selections
        let text_to_copy = "TODO text to copy";
        let json = serde_json::to_string(&self.document).unwrap();

        view! {
            <Host
                foreign:change=(Tabs::tag(), |ev: web_sys::Event| {
                    let doc_path = ev.target().and_then(|el| el.unchecked_ref::<web_sys::Element>().get_attribute("data-id")).unwrap_or_default();
                    let ev: CustomEvent<usize> = ev.into();
                    let new_selection = ev.detail.unwrap_or_default();
                    ExportLinksMsg::ChoiceChanged(format!("{}#{}", doc_path, new_selection))
                })
            >
                <style>{include_str!("export_links.css")}</style>
                <button
                    on:click=|_| ExportLinksMsg::ModalOpen(true)
                >
                    <img src={Icon::Download} alt={t!("export.export")}/>
                </button>
                <Modal prop:open={self.modal_open} on:close=|_| ExportLinksMsg::ModalOpen(false)>
                    <main slot="content" class="export-alert">
                        //{selections_toggle_view}
                        <ul class="export-links">
                            // Link
                            <button class="link" /* on:click={
                                let status = status.clone();
                                move |_ev: Event| share_link(&status)
                            } */>
                                <img src="/static/icons/tabler-icon-link.svg"/>
                                {t!("export.link")}
                            </button>

                            // Embed code: TODO
                            /* <a class="embed" href="#">
                                <img src="/static/icons/tabler-icon-code.svg"/>
                                {t!("export.embed")}
                            </a> */

                            // Word: posts a hidden form to the server and opens the result in a new tab
                            <form class="word" target="_blank" method="post" action="/api/export/docx">
                                <input type="hidden" name="liturgy" value={&self.slug}/>
                                <input type="hidden" name="date" value={&self.date}/>
                                <input type="hidden" name="doc" value={&json}/>
                                <button type="submit">
                                    <img src="/static/icons/file-word-regular.svg"/>
                                    {t!("export.word")}
                                </button>
                            </form>

                            // Venite
                            <button class="link venite" /* on:click={
                                let status = status.clone();
                                let ctrl = document_controller.clone();
                                move |_ev: Event| copy_ldf(&status, &ctrl, &use_whole_doc)
                            } */>
                                <img src="/static/icons/venite.svg"/>
                                {t!("export.venite")}
                            </button>

                            // JSON: downloads a named JSON file
                            <a
                                class="json"
                                download={&format!("{}{}{}.json", self.slug, if self.date.is_empty() { "" } else { "-" }, self.date)}
                                // TODO selection vs. whole doc
                                href={format!("data:application/json,{}", encode_uri(&json))}
                            >
                                <img src="/static/icons/tabler-icon-download.svg"/>
                                {t!("export.json")}
                            </a>
                        </ul>
                    </main>
                    <footer class="export-status">
                        <p class="success" class:hidden={self.status != Status::Success}>
                            {t!("export.clipboard_success")}
                        </p>
                        <p class="error" class:hidden={!matches!(self.status, Status::Error(_))}>
                            {t!("export.clipboard_error")}
                            <pre>{text_to_copy}</pre>
                        </p>
                    </footer>
                </Modal>
            </Host>
        }
    }
}
