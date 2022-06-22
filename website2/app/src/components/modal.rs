use leptos2::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct Modal {
    pub id: String,
    pub label: String,
    pub desc: String,
    #[prop]
    pub open: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ModalMsg {
    Open,
    Close,
}

impl State for Modal {
    type Msg = ModalMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            ModalMsg::Open => self.open = true,
            ModalMsg::Close => self.open = false,
        };
        Some(if self.open {
            Cmd::event(CustomEvent::new("open").detail(()))
        } else {
            Cmd::event(CustomEvent::new("close").detail(()))
        })
    }
}

impl Component for Modal {
    fn view(&self) -> Host {
        view! {
            <Host
                foreign:click=(format!("[data-modal-open='{}'],[data-modal-close='{}']", self.id, self.id), |ev: web_sys::Event| {
                    let target: web_sys::Element = ev.target().unwrap().unchecked_into::<web_sys::HtmlElement>().closest("[data-modal-open],[data-modal-close]").unwrap().unwrap();
                    if target.get_attribute("data-modal-open").is_some() {
                        ModalMsg::Open
                    } else {
                        ModalMsg::Close
                    }
                })
                role="dialog"
                aria-modal="true"
                aria_labelledby="modal_label"
                aria-describedby="modal_desc"
            >
                <style>{include_str!("modal.css")}</style>
                <div class={if self.open { "alert" } else { "alert hidden" }}>
                    <div class="overlay" on:click=|_| ModalMsg::Close></div>
                    <div class="content">
                        <header>
                            {if self.label.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <h1 id="modal_label">{&self.label}</h1>
                                })
                            }}
                            <button class="close" on:click=|_| ModalMsg::Close>
                                <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <line x1="18" y1="6" x2="6" y2="18"></line>
                                    <line x1="6" y1="6" x2="18" y2="18"></line>
                                </svg>
                            </button>
                        </header>
                        <main>
                            {if self.desc.is_empty() {
                                None
                            } else {
                                Some(view! {
                                    <p id="modal_desc">{&self.desc}</p>
                                })
                            }}
                            <slot name="content"></slot>
                        </main>
                    </div>
                </div>
            </Host>
        }
    }
}
