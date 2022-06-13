use async_trait::async_trait;
use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    StreamExt,
};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{cmd::Cmd, debug_warn, link::StateLink, spawn_local};

#[async_trait(?Send)]
pub trait State
where
    Self: Default + Debug + Sized + PartialEq + 'static,
{
    type Msg: Debug;

    fn init(&self) -> Option<Cmd<Self>> {
        None
    }

    fn update(&mut self, msg: Self::Msg) -> Option<Cmd<Self>>;

    fn nested_states(&mut self) -> Vec<&mut dyn StateMachine> {
        Vec::new()
    }

    fn should_notify_parents(&self, msg: &Self::Msg) -> bool {
        true
    }
}

pub trait StateMachine {
    fn run(&self);

    fn set_host_and_parent(&mut self, parent_tx: Rc<dyn ParentWaker>, host: &web_sys::HtmlElement);
}

#[derive(Clone, Default)]
pub struct NestedState<T>
where
    T: State,
    T::Msg: Clone,
{
    state: Rc<RefCell<T>>,
    parent_link: Option<Rc<dyn ParentWaker>>,
    own_tx: Rc<RefCell<Option<UnboundedSender<Option<T::Msg>>>>>,
    host: Option<web_sys::HtmlElement>,
}

impl<T> PartialEq for NestedState<T>
where
    T: State + PartialEq,
    T::Msg: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.host == other.host
    }
}

impl<T> Serialize for NestedState<T>
where
    T: State + Serialize,
    T::Msg: Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.state.borrow().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for NestedState<T>
where
    T: State + Deserialize<'de>,
    T::Msg: Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let state = T::deserialize(deserializer)?;
        Ok(Self::new(state))
    }
}

impl<T> Debug for NestedState<T>
where
    T: State + Debug,
    T::Msg: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NestedState")
            .field("state", &self.state)
            .field("host", &self.host)
            .finish()
    }
}

pub(crate) struct StateSender<T>(UnboundedSender<Option<T::Msg>>)
where
    T: State;

impl<T> StateSender<T>
where
    T: State,
{
    pub(crate) fn new(sender: UnboundedSender<Option<T::Msg>>) -> Self {
        Self(sender)
    }
}

pub trait ParentWaker {
    fn wake(&self);
}

impl<T> ParentWaker for StateSender<T>
where
    T: State,
{
    fn wake(&self) {
        self.0.unbounded_send(None);
    }
}

impl<T> NestedState<T>
where
    T: State,
    T::Msg: Clone,
{
    pub fn new(state: T) -> Self {
        Self {
            state: Rc::new(RefCell::new(state)),
            parent_link: None,
            own_tx: Rc::new(RefCell::new(None)),
            host: None,
        }
    }

    pub(crate) fn set_host(&mut self, host: web_sys::HtmlElement) {
        self.host = Some(host);
    }

    pub(crate) fn set_parent_waker(&mut self, waker: Rc<dyn ParentWaker>) {
        self.parent_link = Some(waker);
    }

    pub fn send(&self, msg: T::Msg) {
        if let Some(tx) = self.own_tx.borrow().as_ref() {
            tx.unbounded_send(Some(msg));
        }
    }

    pub fn with<V>(&self, cb: impl Fn(&T) -> V) -> V {
        (cb)(&self.state.borrow())
    }

    pub fn run_machine(&self) {
        let (tx, mut rx) = unbounded::<Option<T::Msg>>();
        *self.own_tx.borrow_mut() = Some(tx.clone());

        // init
        let init = self.state.borrow().init();
        if let (Some(cmd), Some(host)) = (init, self.host.clone()) {
            let link = StateLink::<T>::from(tx.clone());
            cmd.call(&host, &link);
        }

        // run update loop
        let state = self.state.clone();
        let host = self.host.clone();
        let tx = self.own_tx.borrow().clone();
        let parent_link = self.parent_link.clone();
        spawn_local(async move {
            while let Some(Some(msg)) = rx.next().await {
                let should_notify = state.borrow().should_notify_parents(&msg);

                let cmd = state.borrow_mut().update(msg);
                if should_notify {
                    if let Some(waker) = &parent_link {
                        waker.wake();
                    }
                }

                if let (Some(cmd), Some(host), Some(tx)) = (cmd, &host, tx.clone()) {
                    let link = StateLink::<T>::from(tx.clone());
                    cmd.call(host, &link);
                }
            }
        })
    }
}

impl<T> StateMachine for NestedState<T>
where
    T: State,
    T::Msg: Clone,
{
    fn run(&self) {
        self.run_machine()
    }

    fn set_host_and_parent(&mut self, parent_tx: Rc<dyn ParentWaker>, host: &web_sys::HtmlElement) {
        self.set_host(host.clone());
        self.set_parent_waker(parent_tx);
    }
}
