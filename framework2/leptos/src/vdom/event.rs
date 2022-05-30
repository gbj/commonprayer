use std::{
    any::{type_name, type_name_of_val},
    cell::RefCell,
    panic::Location,
    rc::Rc,
};

use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

use crate::{debug_warn, AsAny, Link};

#[derive(Clone)]
pub struct EventListener {
    pub event_name: String,
    pub handler: Option<Rc<dyn EventCallback>>,
    pub location: &'static Location<'static>,
    pub js_callback: Rc<RefCell<Option<JsValue>>>,
}

impl EventListener {
    pub fn add(&self, node: &web_sys::Node, link: &Link) {
        if let Some(handler) = &self.handler {
            let link = link.clone();
            let handler = handler.clone();
            let event_handler = move |ev: web_sys::Event| {
                let e = handler.invoke(ev);
                if let Err(e) = link.send(e.as_any()) {
                    debug_warn(&format!("[add_listeners] {}", e));
                }
            };
            let js_closure =
                Closure::wrap(Box::new(event_handler) as Box<dyn Fn(_)>).into_js_value();
            node.add_event_listener_with_callback(&self.event_name, js_closure.unchecked_ref())
                .unwrap_throw();
            *self.js_callback.borrow_mut() = Some(js_closure);
        }
    }

    /* pub fn remove(&self, node: &web_sys::Node) {
        if let Some(js_callback) = &self.js_callback {
            node.remove_event_listener_with_callback(&self.event_name, js_callback.unchecked_ref());
        }
    } */
}

/* impl Serialize for EventListener {
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
            location: 0,
        })
    }
} */

impl PartialEq for EventListener {
    fn eq(&self, other: &Self) -> bool {
        self.event_name == other.event_name
            && self.handler.is_some() == other.handler.is_some()
            && self.location == other.location
    }
}

impl std::fmt::Debug for EventListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventListener")
            .field("event_name", &self.event_name)
            .field("handler", &format!("<fn id={:#?}>", self.location))
            .finish()
    }
}

pub trait EventCallback {
    fn invoke(&self, ev: web_sys::Event) -> Box<dyn AsAny>;

    fn type_name(&self) -> &'static str;
}
impl<T, U> EventCallback for T
where
    T: Fn(web_sys::Event) -> Box<U> + Clone,
    U: AsAny + 'static,
{
    fn invoke(&self, ev: web_sys::Event) -> Box<dyn AsAny> {
        (self)(ev)
    }

    fn type_name(&self) -> &'static str {
        type_name::<U>()
    }
}

type ListenerList<M> = Vec<(
    String,
    &'static Location<'static>,
    Rc<dyn Fn(web_sys::Event) -> M>,
)>;
pub struct Listeners<M>(ListenerList<M>);

impl<T, U, M> From<T> for Listeners<M>
where
    M: 'static,
    T: IntoIterator<
        Item = (
            U,
            &'static Location<'static>,
            Rc<dyn Fn(web_sys::Event) -> M>,
        ),
    >,
    U: std::fmt::Display,
{
    fn from(l: T) -> Self {
        Listeners(
            l.into_iter()
                .map(|(event_name, location, cb)| (event_name.to_string(), location, cb))
                .collect(),
        )
    }
}

pub fn build_listeners<M: AsAny + 'static>(listeners: Listeners<M>) -> Vec<EventListener> {
    listeners
        .0
        .into_iter()
        .map(|(event_name, location, cb)| EventListener {
            event_name,
            handler: Some(Rc::new(move |ev| Box::new(cb(ev)))),
            location,
            js_callback: Rc::new(RefCell::new(None)),
        })
        .collect()
}

pub fn build_foreign_listeners<M: AsAny + 'static>(
    listeners: Vec<(
        String,
        String,
        &'static Location<'static>,
        Rc<dyn Fn(web_sys::Event) -> M>,
    )>,
) -> Vec<(String, EventListener)> {
    listeners
        .into_iter()
        .map(|(event_name, selector, location, cb)| {
            (
                selector,
                EventListener {
                    event_name,
                    handler: Some(Rc::new(move |ev| Box::new(cb(ev)))),
                    location,
                    js_callback: Rc::new(RefCell::new(None)),
                },
            )
        })
        .collect()
}
