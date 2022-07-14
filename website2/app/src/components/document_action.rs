use crate::api::document_action::*;
use crate::utils::fetch::Fetch;
use crate::utils::time::today;
use crate::{utils::fetch::FetchStatus, Icon};
use leptos2::*;
use liturgy::Document;
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use web_sys::{AbortController, AbortSignal, UrlSearchParams};

#[derive(Clone, Debug, Default, PartialEq, WebComponent)]
pub struct DocumentAction {
    #[prop]
    pub document: Document,
    pub locale: String,
    pub favoritelabel: String,
    pub removefavoritelabel: String,
    visible: bool,
    favorite_status: FavoriteStatus,
    id: Option<FavoriteId>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FavoriteStatus {
    Unmarked,
    Marking(Option<AbortController>),
    Marked(FavoriteId),
    Removing(FavoriteId, Option<AbortController>),
}

impl Default for FavoriteStatus {
    fn default() -> Self {
        Self::Unmarked
    }
}

#[derive(Debug, Clone)]
pub enum DocumentActionMsg {
    SetVisibility(bool),
    ToggleFavorite,
    MarkFavoriteSuccess(FavoriteId),
    MarkFavoriteError,
    RemoveFavoriteSuccess,
    RemoveFavoriteError(FavoriteId),
}

impl State for DocumentAction {
    type Msg = DocumentActionMsg;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
        match msg {
            DocumentActionMsg::SetVisibility(visible) => {
                self.visible = visible;
            }
            DocumentActionMsg::ToggleFavorite => {
                let (new_status, cmd) = match &self.favorite_status {
                    FavoriteStatus::Unmarked => {
                        let controller = self.new_abort_controller();
                        let signal = controller.as_ref().map(|ctrl| ctrl.signal());
                        (
                            FavoriteStatus::Marking(controller),
                            Some(self.mark_favorite(signal)),
                        )
                    }
                    FavoriteStatus::Marking(controller) => {
                        if let Some(controller) = controller {
                            controller.abort();
                        }
                        (FavoriteStatus::Unmarked, None)
                    }
                    FavoriteStatus::Marked(id) => {
                        let controller = self.new_abort_controller();
                        let signal = controller.as_ref().map(|ctrl| ctrl.signal());
                        (
                            FavoriteStatus::Removing(*id, controller),
                            Some(self.remove_favorite(*id, signal)),
                        )
                    }
                    FavoriteStatus::Removing(id, controller) => {
                        if let Some(controller) = controller {
                            controller.abort();
                        }
                        (FavoriteStatus::Marked(*id), None)
                    }
                };
                leptos2::debug_warn(&format!("new status = {:#?}", new_status));
                self.favorite_status = new_status;
                return cmd;
            }
            DocumentActionMsg::MarkFavoriteSuccess(id) => {
                self.favorite_status = FavoriteStatus::Marked(id)
            }
            DocumentActionMsg::MarkFavoriteError => self.favorite_status = FavoriteStatus::Unmarked,
            DocumentActionMsg::RemoveFavoriteSuccess => {
                self.favorite_status = FavoriteStatus::Unmarked
            }
            DocumentActionMsg::RemoveFavoriteError(id) => {
                self.favorite_status = FavoriteStatus::Marked(id)
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
                    <button aria-label={&self.favoritelabel} on:click=|_| Self::Msg::ToggleFavorite>
                        <svg xmlns="http://www.w3.org/2000/svg" class:filled={self.favorite_marked()} class="icon icon-tabler icon-tabler-star" width="24" height="24" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
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
    fn favorite_marked(&self) -> bool {
        matches!(
            self.favorite_status,
            FavoriteStatus::Marking(_) | FavoriteStatus::Marked(_)
        )
    }

    fn new_abort_controller(&self) -> Option<AbortController> {
        AbortController::new().ok()
    }

    fn mark_favorite(&self, signal: Option<AbortSignal>) -> Cmd<Self> {
        let req = self.mark_favorite_request(signal);
        Cmd::new(move |_, link| match req {
            Err(e) => leptos2::debug_warn(&format!("[DocumentAction::mark_favorite] {:?}", e)),
            Ok(req) => spawn_local({
                let link = link.clone();
                async move {
                    match req.send().await {
                        Ok(resp) => match resp.json::<FavoriteId>().await {
                            Err(e) => {
                                leptos2::debug_warn(&format!(
                                    "[DocumentAction::mark_favorite] {}",
                                    e
                                ));
                                link.send(&DocumentActionMsg::MarkFavoriteError);
                            }
                            Ok(id) => {
                                link.send(&DocumentActionMsg::MarkFavoriteSuccess(id));
                            }
                        },
                        Err(e) => {
                            leptos2::debug_warn(&format!("[DocumentAction::mark_favorite] {}", e));
                            link.send(&DocumentActionMsg::MarkFavoriteError);
                        }
                    }
                }
            }),
        })
    }

    fn remove_favorite(&self, id: FavoriteId, signal: Option<AbortSignal>) -> Cmd<Self> {
        let req = self.remove_favorite_request(id, signal);
        Cmd::new(move |_, link| match req {
            Err(e) => leptos2::debug_warn(&format!("[DocumentAction::remove_favorite] {:?}", e)),
            Ok(req) => spawn_local({
                let link = link.clone();
                async move {
                    match req.send().await {
                        Ok(resp) => match resp.json::<FavoriteId>().await {
                            Err(e) => {
                                leptos2::debug_warn(&format!(
                                    "[DocumentAction::remove_favorite] {}",
                                    e
                                ));
                                link.send(&DocumentActionMsg::MarkFavoriteError);
                            }
                            Ok(id) => {
                                link.send(&DocumentActionMsg::RemoveFavoriteSuccess);
                            }
                        },
                        Err(e) => {
                            leptos2::debug_warn(&format!("[DocumentAction::mark_favorite] {}", e));
                            link.send(&DocumentActionMsg::MarkFavoriteError);
                        }
                    }
                }
            }),
        })
    }

    fn remove_favorite_request(
        &self,
        id: FavoriteId,
        signal: Option<AbortSignal>,
    ) -> Result<Request, JsValue> {
        let data = UrlSearchParams::new()?;
        data.append("action", &DocumentActionType::RemoveFavorite.to_string());
        data.append(
            "payload",
            &serde_json::to_string(&id).expect("couldn't serialize FavoriteId"),
        );

        Ok(Request::post(&format!("/{}", self.locale))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .abort_signal(signal.as_ref())
            .body(data))
    }

    fn mark_favorite_request(&self, signal: Option<AbortSignal>) -> Result<Request, JsValue> {
        let data = UrlSearchParams::new()?;
        data.append("action", &DocumentActionType::MarkFavorite.to_string());
        data.append(
            "payload",
            &serde_json::to_string(&self.document).expect("couldn't serialize document"),
        );
        data.append("date_created", &today().to_padded_string());

        Ok(Request::post(&format!("/{}", self.locale))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .abort_signal(signal.as_ref())
            .body(data))
    }
}
