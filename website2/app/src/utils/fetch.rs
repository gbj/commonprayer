use std::fmt::{Debug, Display};

use leptos2::{async_trait, is_server, Deserialize, Serialize, State};
use reqwasm::http::Request;
use serde::de::DeserializeOwned;
use thiserror::Error;
use web_sys::{AbortController, AbortSignal};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fetch<T> {
    url: String,
    #[serde(skip)]
    abort_controller: Option<AbortController>,
    pub status: FetchStatus<T>,
}

impl<T> Fetch<T>
where
    T: PartialEq + Serialize + Deserialize<'static>,
{
    pub fn new(url: impl Display) -> Self {
        Self {
            url: url.to_string(),
            abort_controller: Self::abort_controller(),
            status: FetchStatus::Idle,
        }
    }

    pub fn with_status(status: FetchStatus<T>) -> Self {
        Self {
            url: String::new(),
            abort_controller: None,
            status,
        }
    }

    pub fn is_loading(&self) -> bool {
        self.status == FetchStatus::Loading
    }

    fn abort_controller() -> Option<AbortController> {
        if is_server!() {
            None
        } else {
            AbortController::new().ok()
        }
    }

    fn cancel_search(&mut self) {
        if let Some(controller) = &self.abort_controller {
            controller.abort();
        }
        // for some reason, need to reset the controller to be able to send new searches
        self.abort_controller = Self::abort_controller();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FetchMsg<T> {
    Abort,
    SetUrlAndGet(String),
    Success(Box<T>),
    Error(FetchError),
    Noop,
}

impl<T> Default for FetchMsg<T> {
    fn default() -> Self {
        Self::Noop
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FetchCmd {
    Get(String, Option<AbortController>),
}

#[async_trait(?Send)]
impl<T> State for Fetch<T>
where
    T: Default + Debug + PartialEq + Serialize + DeserializeOwned + 'static,
{
    type Msg = FetchMsg<T>;
    type Cmd = FetchCmd;

    fn update(&mut self, msg: Self::Msg) -> Option<Self::Cmd> {
        if self.abort_controller.is_none() {
            self.abort_controller = Self::abort_controller();
        }

        match msg {
            FetchMsg::Abort => {
                self.cancel_search();
                self.status = FetchStatus::Idle;
            }
            FetchMsg::SetUrlAndGet(url) => {
                if self.is_loading() {
                    self.cancel_search();
                }

                self.url = url;
                self.status = FetchStatus::Loading;
                return Some(FetchCmd::Get(
                    self.url.clone(),
                    self.abort_controller.clone(),
                ));
            }
            FetchMsg::Success(data) => self.status = FetchStatus::Success(data),
            FetchMsg::Error(e) => self.status = FetchStatus::Error(e),
            FetchMsg::Noop => {}
        }
        None
    }

    fn should_notify_parents(&self, msg: &Self::Msg) -> bool {
        !matches!(msg, FetchMsg::Abort | FetchMsg::Noop)
    }

    async fn cmd(cmd: Self::Cmd, _host: web_sys::HtmlElement) -> Option<Self::Msg> {
        match cmd {
            FetchCmd::Get(url, controller) => {
                let abort_signal = controller.as_ref().map(|ac| ac.signal());
                match fetch::<T>(&url, abort_signal.as_ref()).await {
                    Ok(res) => Some(FetchMsg::Success(Box::new(res))),
                    Err(e) => Some(match e {
                        FetchError::Abort => FetchMsg::Noop,
                        _ => FetchMsg::Error(e),
                    }),
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FetchStatus<T> {
    Idle,
    Loading,
    Error(FetchError),
    Success(Box<T>),
}

impl<T> Default for FetchStatus<T> {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Error, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum FetchError {
    #[error("request was aborted by client")]
    Abort,
    #[error("could not connect to server")]
    Connection,
    #[error("internal server error")]
    Server,
    #[error("error deserializing data")]
    Json,
}

pub async fn fetch<T>(url: &str, signal: Option<&AbortSignal>) -> Result<T, FetchError>
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
                } else if e.name == "AbortError" {
                    FetchError::Abort
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
