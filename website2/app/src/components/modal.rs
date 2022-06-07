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

#[async_trait(?Send)]
impl State for Modal {
    type Msg = ModalMsg;
    type Cmd = bool;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        leptos2::debug_warn(&format!("Modal received msg {:#?}", msg));
        match msg {
            ModalMsg::Open => self.open = true,
            ModalMsg::Close => self.open = false,
        };
        Some(self.open)
    }

    async fn cmd(
        cmd: Self::Cmd,
        host: web_sys::HtmlElement,
        _link: StateLink<Self>,
    ) -> Option<Self::Msg> {
        let event_emitter = EventEmitter::new(&host);
        if !cmd {
            event_emitter.emit(CustomEvent::<()>::new("close").detail(()));
        } else {
            event_emitter.emit(CustomEvent::<()>::new("open").detail(()));
        }
        None
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
            </Host>
        }
    }
}
