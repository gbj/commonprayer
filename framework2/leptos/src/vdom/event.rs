use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::AsAny;

#[derive(Clone)]
pub struct EventListener {
    pub event_name: String,
    pub handler: Option<Rc<dyn EventCallback>>,
}

impl Serialize for EventListener {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.event_name.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EventListener {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let event_name = String::deserialize(deserializer)?;
        Ok(Self {
            event_name,
            handler: None,
        })
    }
}

impl PartialEq for EventListener {
    fn eq(&self, other: &Self) -> bool {
        self.event_name == other.event_name && self.handler.is_some() == other.handler.is_some()
    }
}

impl std::fmt::Debug for EventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListener")
            .field("event_name", &self.event_name)
            .field("handler", &"<fn>")
            .finish()
    }
}

pub trait EventCallback {
    fn invoke(&self, ev: web_sys::Event) -> Box<dyn AsAny>;
}
impl<T, U> EventCallback for T
where
    T: Fn(web_sys::Event) -> Box<U> + Clone,
    U: AsAny + 'static,
{
    fn invoke(&self, ev: web_sys::Event) -> Box<dyn AsAny> {
        (self)(ev)
    }
}

type ListenerList<M> = Vec<(String, Rc<dyn Fn(web_sys::Event) -> M>)>;
pub struct Listeners<M>(ListenerList<M>);

impl<T, U, M> From<T> for Listeners<M>
where
    M: 'static,
    T: IntoIterator<Item = (U, Rc<dyn Fn(web_sys::Event) -> M>)>,
    U: std::fmt::Display,
{
    fn from(l: T) -> Self {
        Listeners(
            l.into_iter()
                .map(|(event_name, cb)| (event_name.to_string(), cb))
                .collect(),
        )
    }
}

pub fn build_listeners<M: AsAny + 'static>(listeners: Listeners<M>) -> Vec<EventListener> {
    listeners
        .0
        .into_iter()
        .map(|(event_name, cb)| EventListener {
            event_name,
            handler: Some(Rc::new(move |ev| Box::new(cb(ev)))),
        })
        .collect()
}

pub fn build_foreign_listeners<M: AsAny + 'static>(
    listeners: Vec<(String, String, Rc<dyn Fn(web_sys::Event) -> M>)>,
) -> Vec<(String, EventListener)> {
    listeners
        .into_iter()
        .map(|(event_name, selector, cb)| {
            (
                selector,
                EventListener {
                    event_name,
                    handler: Some(Rc::new(move |ev| Box::new(cb(ev)))),
                },
            )
        })
        .collect()
}
