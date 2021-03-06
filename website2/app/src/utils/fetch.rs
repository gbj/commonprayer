use std::fmt::{Debug, Display};

use leptos2::*;
use serde::de::DeserializeOwned;
use thiserror::Error;
use web_sys::{AbortController, AbortSignal, UrlSearchParams};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fetch<T> {
    url: String,
    #[serde(skip)]
    data: Option<UrlSearchParams>,
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
            data: None,
            abort_controller: Self::abort_controller(),
            status: FetchStatus::Idle,
        }
    }

    pub fn with_status(status: FetchStatus<T>) -> Self {
        Self {
            url: String::new(),
            data: None,
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
    Post(String, UrlSearchParams),
    Success(Box<T>),
    Error(FetchError),
    Noop,
}

impl<T> Default for FetchMsg<T> {
    fn default() -> Self {
        Self::Noop
    }
}

impl<T> State for Fetch<T>
where
    T: Clone + Default + Debug + PartialEq + Serialize + DeserializeOwned + 'static,
{
    type Msg = FetchMsg<T>;

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>> {
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
                return Some(self.get());
            }
            FetchMsg::Post(url, data) => {
                if self.is_loading() {
                    self.cancel_search();
                }

                self.url = url;
                self.data = Some(data);
                self.status = FetchStatus::Loading;
                return Some(self.post());
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
}

impl<T> Fetch<T>
where
    T: Clone + Default + Debug + PartialEq + Serialize + DeserializeOwned + 'static,
{
    fn get(&self) -> Cmd<Self> {
        let url = self.url.clone();
        let controller = self.abort_controller.clone();
        Cmd::new(move |_, link| {
            let abort_signal = controller.as_ref().map(|ac| ac.signal());
            let link = link.clone();
            spawn_local(async move {
                match fetch::<T>(&url, abort_signal.as_ref()).await {
                    Ok(res) => link.send(&FetchMsg::Success(Box::new(res))),
                    Err(e) => match e {
                        FetchError::Abort => link.send(&FetchMsg::Noop),
                        _ => link.send(&FetchMsg::Error(e)),
                    },
                };
            });
        })
    }

    fn post(&self) -> Cmd<Self> {
        let url = self.url.clone();
        let controller = self.abort_controller.clone();
        let data = self.data.clone();
        Cmd::new(move |_, link| {
            let abort_signal = controller.as_ref().map(|ac| ac.signal());
            let link = link.clone();
            spawn_local(async move {
                let mut req =
                    reqwasm::http::Request::post(&url).abort_signal(abort_signal.as_ref());
                if let Some(data) = data {
                    req = req.body(data);
                }

                match req.send().await {
                    Ok(res) => match res.json::<T>().await {
                        Ok(res) => link.send(&FetchMsg::Success(Box::new(res))),
                        Err(e) => link.send(&FetchMsg::Error(FetchError::Json)),
                    },
                    Err(e) => link.send(&FetchMsg::Error(FetchError::Server)),
                };
            });
        })
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
    #[error("error with caching middleware")]
    Cache,
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
    reqwest::get(url)
        //.abort_signal(signal) // TODO restore AbortSignal
        //.send()
        .await
        .map_err(|e| {
            todo!()
            /* if e.is_connect() {
                FetchError::Connection
            } else {
                FetchError::Server
            } */
        })?
        .json::<T>()
        .await
        .map_err(|_| FetchError::Json)
}
