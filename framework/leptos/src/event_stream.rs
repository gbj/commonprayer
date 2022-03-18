use futures::{channel::mpsc::unbounded, Stream};
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Event, HtmlElement};

use crate::{document, is_server, window};

pub fn window_event_stream(event_name: &'static str) -> impl Stream<Item = web_sys::Event> {
    let (tx, rx) = unbounded();

    if !is_server!() {
        let closure = Closure::wrap(
            Box::new(move |ev: Event| tx.unbounded_send(ev).unwrap_throw()) as Box<dyn Fn(_)>,
        )
        .into_js_value();
        window()
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .unwrap_throw();
    }

    rx
}

pub fn document_event_stream(event_name: &'static str) -> impl Stream<Item = web_sys::Event> {
    let (tx, rx) = unbounded();

    if !is_server!() {
        let closure = Closure::wrap(
            Box::new(move |ev: Event| tx.unbounded_send(ev).unwrap_throw()) as Box<dyn Fn(_)>,
        )
        .into_js_value();
        document()
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .unwrap_throw();
    }

    rx
}

pub fn event_stream(
    target: &HtmlElement,
    event_name: &'static str,
) -> impl Stream<Item = web_sys::Event> {
    let (tx, rx) = unbounded();
    let closure = Closure::wrap(
        Box::new(move |ev: Event| tx.unbounded_send(ev).unwrap_throw()) as Box<dyn Fn(_)>,
    )
    .into_js_value();
    target
        .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
        .unwrap_throw();
    rx
}
