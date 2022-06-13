use leptos2::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct Modal {
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
            <Host>
                <style>{include_str!("modal.css")}</style>
                <div class={if self.open { "alert" } else { "alert hidden" }}>
                    <div class="overlay" on:click=|_| ModalMsg::Close></div>
                    <slot name="content" class="content"></slot>
                </div>
                <button on:click=|_| ModalMsg::Open>
                    <slot name="button"></slot>
                </button>
            </Host>
        }
    }
}
