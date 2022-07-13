use crate::api::document_action::*;
use crate::{utils::fetch::FetchStatus, Icon};
use leptos2::*;
use liturgy::Document;
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use web_sys::UrlSearchParams;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize, WebComponent)]
pub struct DocumentAction {
    #[prop]
    pub document: Document,
    pub locale: String,
    pub favoritelabel: String,
    pub removefavoritelabel: String,
    visible: bool,
    favorite_marked: bool,
    id: Option<i64>,
}

#[derive(Debug, Clone)]
pub enum DocumentActionMsg {
    SetVisibility(bool),
    MarkFavorite,
    RemoveFavorite,
    MarkFavoriteSuccess(i64),
    MarkFavoriteError,
}

impl State for DocumentAction {
    type Msg = DocumentActionMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            DocumentActionMsg::SetVisibility(visible) => {
                self.visible = visible;
            }
            DocumentActionMsg::MarkFavorite => {
                self.favorite_marked = true;
                return Some(self.mark_favorite());
            }
            DocumentActionMsg::RemoveFavorite => {
                self.favorite_marked = false;
                return Some(self.remove_favorite());
            }
            DocumentActionMsg::MarkFavoriteSuccess(id) => {
                self.id = Some(id);
            }
            DocumentActionMsg::MarkFavoriteError => {
                self.favorite_marked = false;
            }
        }
        None
    }
}

impl Component for DocumentAction {
    fn view(&self) -> Host {
        view! {
            <Host
                on:focus=|_| Self::Msg::SetVisibility(true)
                on:blur=|_| Self::Msg::SetVisibility(false)
            >
                <style>{include_str!("document_action.css")}</style>
                <div class="buttons" class:hidden={!self.visible}>
                    <button aria-label={&self.favoritelabel} on:click=|_| Self::Msg::MarkFavorite>
                        <svg xmlns="http://www.w3.org/2000/svg" class:filled={self.favorite_marked} class="icon icon-tabler icon-tabler-star" width="24" height="24" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M12 17.75l-6.172 3.245l1.179 -6.873l-5 -4.867l6.9 -1l3.086 -6.253l3.086 6.253l6.9 1l-5 4.867l1.179 6.873z"></path>
                        </svg>
                    </button>
                </div>

                // Document contents
                <slot></slot>
            </Host>
        }
    }
}

impl DocumentAction {
    fn mark_favorite(&self) -> Cmd<Self> {
        let req = self.mark_favorite_request();
        Cmd::new(move |_, link| match req {
            Err(e) => leptos2::debug_warn(&format!("[DocumentAction::mark_favorite] {:?}", e)),
            Ok(req) => spawn_local({
                let link = link.clone();
                async move {
                    match req.send().await {
                        Ok(resp) => {
                            leptos2::debug_warn(&format!(
                                "[DocumentAction::mark_favorite] Received response\n\n {:#?}",
                                resp
                            ));
                        }
                        Err(e) => {
                            leptos2::debug_warn(&format!("[DocumentAction::mark_favorite] {}", e));
                            link.send(&DocumentActionMsg::MarkFavoriteError);
                        }
                    }
                }
            }),
        })
    }

    fn remove_favorite(&self) -> Cmd<Self> {
        todo!()
    }

    fn mark_favorite_request(&self) -> Result<Request, JsValue> {
        let data = UrlSearchParams::new()?;
        data.append("action", &DocumentActionType::MarkFavorite.to_string());
        data.append(
            "payload",
            &serde_json::to_string(&self.document).expect("couldn't serialize document"),
        );

        Ok(Request::post(&format!("/{}", self.locale))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(data))
    }
}
