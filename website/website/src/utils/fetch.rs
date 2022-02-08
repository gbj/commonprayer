use std::fmt::Display;

use leptos::{is_server, Behavior};
use reqwasm::http::Request;
use serde::de::DeserializeOwned;
use thiserror::Error;
use wasm_bindgen_futures::spawn_local;
use web_sys::{AbortController, AbortSignal};

#[derive(Clone)]
pub struct Fetch<T>
where
    T: Clone + DeserializeOwned + 'static,
{
    url: String,
    abort_controller: Option<AbortController>,
    pub state: Behavior<FetchStatus<T>>,
}

impl<T> Fetch<T>
where
    T: Clone + DeserializeOwned + 'static,
{
    pub fn new(url: impl Display) -> Self {
        Self::new_with_status(url, FetchStatus::Idle)
    }

    pub fn new_with_status(url: impl Display, status: FetchStatus<T>) -> Self {
        let abort_controller = if is_server!() {
            None
        } else {
            AbortController::new().ok()
        };
        Self {
            url: url.to_string(),
            abort_controller,
            state: Behavior::new(status),
        }
    }

    pub fn send(&self) {
        if !is_server!() {
            self.state.set(FetchStatus::Loading);
            spawn_local({
                let state = self.state.clone();
                let url = self.url.clone();
                let signal = self.abort_controller.as_ref().map(|ac| ac.signal());
                async move {
                    match fetch(&url, signal.as_ref()).await {
                        Ok(data) => state.set(FetchStatus::Success(Box::new(data))),
                        Err(e) => state.set(FetchStatus::Error(e)),
                    };
                }
            });
        }
    }

    pub fn abort(&self) {
        if let Some(controller) = &self.abort_controller {
            controller.abort();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FetchStatus<T> {
    Idle,
    Loading,
    Error(FetchError),
    Success(Box<T>),
}

#[derive(Error, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FetchError {
    #[error("could not connect to server")]
    Connection,
    #[error("internal server error")]
    Server,
    #[error("error deserializing data")]
    Json,
}

async fn fetch<T>(url: &str, signal: Option<&AbortSignal>) -> Result<T, FetchError>
where
    T: DeserializeOwned,
{
    Request::get(url)
        .abort_signal(signal)
        .send()
        .await
        .map_err(|e| match e {
            reqwasm::Error::JsError(e) => {
                if e.name == "NetworkError" {
                    FetchError::Connection
                } else {
                    FetchError::Server
                }
            }
            reqwasm::Error::SerdeError(_) => FetchError::Json,
        })?
        .json::<T>()
        .await
        .map_err(|_| FetchError::Json)
}
