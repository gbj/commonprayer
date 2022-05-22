pub mod operations;
pub use operations::*;

use crate::{link::Link, Element, EventListener, Node};
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};

impl Node {
    pub fn to_node(&self, link: &Link) -> web_sys::Node {
        match self {
            Node::Element(element) => element.to_node(link),
            Node::Text(text) => create_text_node(&document(), text).unchecked_into(),
        }
    }
}

impl Element {
    pub fn to_node(&self, link: &Link) -> web_sys::Node {
        let el = create_element(&document(), &self.tag);

        // set attributes
        for attr in &self.attrs {
            attr.set(&el);
        }

        add_listeners(el.unchecked_ref(), &self.listeners, link);

        // append children
        for child in &self.children {
            let child_node = child.to_node(link);
            append_child(&el, &child_node);
        }

        el.unchecked_into()
    }

    pub fn hydrate(&self, parent: &web_sys::Element, idx: usize, link: &Link) {
        let el = parent.children().item(idx as u32).unwrap();
        add_listeners(el.unchecked_ref(), &self.listeners, link);

        // append children
        for (idx, child) in self
            .children
            .iter()
            .filter_map(|node| match node {
                Node::Element(el) => Some(el),
                Node::Text(_) => None,
            })
            .enumerate()
        {
            child.hydrate(&el, idx, link);
        }
    }
}

pub fn add_listeners(node: &web_sys::Node, listeners: &[EventListener], link: &Link) {
    for listener in listeners {
        if let Some(handler) = &listener.handler {
            let link = link.clone();
            let handler = handler.clone();
            let event_handler = move |ev: web_sys::Event| {
                let e = handler.invoke(ev);
                if let Err(e) = link.send(e.as_any()) {
                    debug_warn(&format!("[add_listeners] {}", e));
                }
            };
            node.add_event_listener_with_callback(
                &listener.event_name,
                Closure::wrap(Box::new(event_handler) as Box<dyn Fn(_)>)
                    .into_js_value()
                    .unchecked_ref(),
            )
            .unwrap_throw();
        }
    }
}

pub fn add_foreign_listeners(
    node: &web_sys::Node,
    listeners: &[(String, EventListener)],
    link: &Link,
) {
    for (selector, listener) in listeners {
        let selector = selector.clone();
        if let Some(handler) = &listener.handler {
            let link = link.clone();
            let handler = handler.clone();
            let event_handler = move |mut ev: web_sys::Event| {
                if let Some(target_el) = ev
                    .target()
                    .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                {
                    if let Ok(Some(selected_ancestor)) = target_el.closest(&selector) {
                        let e = handler.invoke(ev);
                        if let Err(e) = link.send(e.as_any()) {
                            debug_warn(&format!("[add_foreign_listeners] {}", e));
                        }
                    }
                }
            };
            node.add_event_listener_with_callback(
                &listener.event_name,
                Closure::wrap(Box::new(event_handler) as Box<dyn Fn(_)>)
                    .into_js_value()
                    .unchecked_ref(),
            )
            .unwrap_throw();
        }
    }
}
