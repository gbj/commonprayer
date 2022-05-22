use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::CustomEventInit;

pub struct EventEmitter {
    target: web_sys::Element,
}

impl EventEmitter {
    pub fn new(target: &web_sys::Element) -> Self {
        Self {
            target: target.clone(),
        }
    }

    pub fn emit<T>(&self, event: CustomEvent<T>)
    where
        T: Serialize + DeserializeOwned,
    {
        let event: web_sys::CustomEvent = event.into();
        self.target
            .unchecked_ref::<web_sys::EventTarget>()
            .dispatch_event(event.unchecked_ref())
            .unwrap_throw();
    }
}

pub struct CustomEvent<T> {
    pub name: String,
    pub detail: Option<T>,
    pub bubbles: bool,
    pub cancelable: bool,
    pub composed: bool,
}

impl<T> CustomEvent<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(name: impl std::fmt::Display) -> Self {
        Self {
            name: name.to_string(),
            detail: None,
            bubbles: false,
            cancelable: false,
            composed: false,
        }
    }

    pub fn bubbles(mut self) -> Self {
        self.bubbles = true;
        self
    }

    pub fn cancelable(mut self) -> Self {
        self.cancelable = true;
        self
    }

    pub fn composed(mut self) -> Self {
        self.composed = true;
        self
    }

    pub fn detail(mut self, detail: T) -> Self {
        self.detail = Some(detail);
        self
    }
}

impl<T> From<CustomEvent<T>> for web_sys::CustomEvent
where
    T: Serialize + DeserializeOwned,
{
    fn from(ev: CustomEvent<T>) -> Self {
        let detail = ev
            .detail
            .and_then(|detail| JsValue::from_serde(&detail).ok());
        web_sys::CustomEvent::new_with_event_init_dict(
            &ev.name,
            CustomEventInit::new()
                .bubbles(ev.bubbles)
                .cancelable(ev.cancelable)
                .composed(ev.composed)
                .detail(&detail.unwrap_or(JsValue::NULL)),
        )
        .unwrap_throw()
    }
}

impl<T> From<web_sys::Event> for CustomEvent<T>
where
    T: Serialize + DeserializeOwned,
{
    fn from(ev: web_sys::Event) -> Self {
        let ev = ev.unchecked_into::<web_sys::CustomEvent>();
        Self {
            name: ev.type_(),
            bubbles: ev.bubbles(),
            cancelable: ev.cancelable(),
            composed: ev.composed(),
            detail: if ev.detail() == JsValue::NULL {
                None
            } else {
                Some(JsValue::into_serde(&ev.detail()).unwrap_or_else(|e| {
                    crate::debug_warn(&format!("{:#?}", e));
                    panic!()
                }))
            },
        }
    }
}
