use leptos2::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct ChoiceView {
    pub selection: usize,
}

#[async_trait(?Send)]
impl Component for ChoiceView {
    type Msg = usize;
    type Cmd = ();

    fn update(&mut self, msg: &Self::Msg) -> Option<Self::Cmd> {
        self.selection = *msg;
        None
    }

    async fn cmd(_cmd: Self::Cmd, _host: web_sys::HtmlElement) -> Option<Self::Msg> {
        None
    }

    fn view(&self) -> Host {
        view! {
            <Host>
                <style>{include_str!("choice.css")}</style>
                <div class="tabs">
                    <slot name="choices"></slot>
                </div>
                <slot name="children"></slot>
            </Host>
        }
    }
}
