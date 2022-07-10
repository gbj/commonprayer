use std::time::Duration;

use crate::components::{Modal, Tabs};
use crate::utils::encode_uri;
use crate::utils::share::copy_text;
use crate::Icon;
use leptos2::*;
use liturgy::{BiblicalReading, Content, Document};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct ExportLinks {
    pub slug: String,
    pub date: String,
    #[prop]
    pub document: Document,
    pub buttonlabel: String,
    pub linklabel: String,
    pub wordlabel: String,
    pub venitelabel: String,
    pub jsonlabel: String,
    pub clipboardsuccess: String,
    pub clipboarderror: String,
    modal_open: bool,
    status: Status,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Status {
    Idle,
    Success,
    Error(String),
    LocationError,
}

impl Default for Status {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExportLinksMsg {
    CopyLink,
    CopyLDF,
    ModalOpen(bool),
    ChoiceChanged(String),
    ReadingLoaded(Vec<usize>, BiblicalReading),
    ClipboardError(String),
    ClipboardSuccess,
    ClearStatus,
    Noop,
}

impl State for ExportLinks {
    type Msg = ExportLinksMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            Self::Msg::CopyLink => {
                let location = location();
                return match (location.pathname(), location.search()) {
                    (Ok(pathname), Ok(search)) => {
                        let url =
                            format!("https://www.commonprayeronline.org{}{}", pathname, search);
                        Some(self.copy(url))
                    }
                    _ => {
                        self.status = Status::LocationError;
                        None
                    }
                };
            }
            Self::Msg::CopyLDF => {
                let ldf = ldf::LdfJson::from(self.document.clone());
                return match serde_json::to_string(&ldf.into_inner()) {
                    Ok(json) => Some(self.copy(json)),
                    Err(_) => {
                        self.status = Status::LocationError;
                        None
                    }
                };
            }
            Self::Msg::ModalOpen(modal_open) => {
                self.modal_open = modal_open;
            }
            Self::Msg::ChoiceChanged(content) => {
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
            Self::Msg::ReadingLoaded(path, reading) => {
                leptos2::debug_warn(&format!(
                    "loaded biblical reading at {:?}\n\n{:#?}",
                    path, reading
                ));
                if let Ok(subdoc) = self.document.at_path_mut(path) {
                    *subdoc = Document::from(reading);
                };
            }
            Self::Msg::ClipboardError(text) => {
                self.status = Status::Error(text);
                return Some(self.clear_status_after_timeout(Duration::from_secs(3)));
            }
            Self::Msg::ClipboardSuccess => {
                self.status = Status::Success;
                return Some(self.clear_status_after_timeout(Duration::from_secs(30)));
            }
            Self::Msg::ClearStatus => {
                self.status = Status::Idle;
            }
            Self::Msg::Noop => {}
        }
        None
    }
}

impl Component for ExportLinks {
    fn view(&self) -> Host {
        let json = serde_json::to_string(&self.document).unwrap();
        let json = encode_uri(&json);
        let text_to_copy = String::new(); // TODO

        view! {
            <Host
                window:readingloaded=|ev: Event| {
                    let ev: CustomEvent<(Vec<usize>, BiblicalReading)> = ev.into();
                    leptos2::debug_warn(&format!("window:readingloaded {:#?}", ev.detail));
                    if let Some((path, reading)) = ev.detail {
                        Self::Msg::ReadingLoaded(path, reading)
                    } else {
                        Self::Msg::Noop
                    }
                }
                foreign:change=(Tabs::tag(), |ev: web_sys::Event| {
                    let doc_path = ev.target().and_then(|el| el.unchecked_ref::<web_sys::Element>().get_attribute("data-id")).unwrap_or_default();
                    let ev: CustomEvent<usize> = ev.into();
                    let new_selection = ev.detail.unwrap_or_default();
                    ExportLinksMsg::ChoiceChanged(format!("{}#{}", doc_path, new_selection))
                })
            >
                <style>{include_str!("export_links.css")}</style>
                <button class="open-button"
                    on:click=|_| ExportLinksMsg::ModalOpen(true)
                >
                    <img src={Icon::Download} alt={&self.buttonlabel}/>
                </button>
                <Modal prop:open={self.modal_open} on:close=|_| ExportLinksMsg::ModalOpen(false)>
                    <main slot="content" class="export-alert">
                        //{selections_toggle_view}
                        <ul class="export-links">
                            // Link
                            <button class="link"
                            on:click=|_| Self::Msg::CopyLink>
                                <img src="/static/icons/tabler-icon-link.svg"/>
                                {&self.linklabel}
                            </button>

                            // Embed code: TODO
                            /* <a class="embed" href="#">
                                <img src="/static/icons/tabler-icon-code.svg"/>
                                {t!("export.embed")}
                            </a> */

                            // Word: posts a hidden form to the server and opens the result in a new tab
                            <form class="word" target="_blank" method="post">
                                <input type="hidden" name="liturgy" value={&self.slug}/>
                                <input type="hidden" name="date" value={&self.date}/>
                                <input type="hidden" name="doc" value={&json}/>
                                <button type="submit">
                                    <img src="/static/icons/file-word-regular.svg"/>
                                    {&self.wordlabel}
                                </button>
                            </form>

                            // Venite
                            <button class="link venite" on:click=|_| Self::Msg::CopyLDF>
                                <img src="/static/icons/venite.svg"/>
                                {&self.venitelabel}
                            </button>

                            // JSON: downloads a named JSON file
                            <a
                                class="json"
                                download={&format!("{}{}{}.json", self.slug, if self.date.is_empty() { "" } else { "-" }, self.date)}
                                // TODO selection vs. whole doc
                                href={format!("data:application/json,{}", json)}
                            >
                                <img src="/static/icons/tabler-icon-download.svg"/>
                                {&self.jsonlabel}
                            </a>
                        </ul>
                    </main>
                    <footer class="export-status" slot="content">
                        <p class="success" class:hidden={self.status != Status::Success}>
                            {&self.clipboardsuccess}
                        </p>
                        <p class="error" class:hidden={!matches!(self.status, Status::Error(_) | Status::LocationError)}>
                            {&self.clipboarderror}
                            <pre>{text_to_copy}</pre>
                        </p>
                    </footer>
                </Modal>
            </Host>
        }
    }
}

impl ExportLinks {
    fn copy(&self, text: String) -> Cmd<Self> {
        Cmd::new(|_, link| {
            let link = link.clone();
            spawn_local(async move {
                match copy_text(&text).await {
                    Ok(_) => {
                        leptos2::debug_warn("copied text");

                        link.send(&ExportLinksMsg::ClipboardSuccess);
                    }
                    Err(_) => {
                        leptos2::debug_warn("failed to copy text");

                        link.send(&ExportLinksMsg::ClipboardError(text));
                    }
                }
            })
        })
    }

    fn clear_status_after_timeout(&self, duration: Duration) -> Cmd<Self> {
        Cmd::new(move |_, link| {
            let link = link.clone();
            set_timeout(
                move || {
                    link.send(&ExportLinksMsg::ClearStatus);
                },
                duration,
            )
        })
    }
}
