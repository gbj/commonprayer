pub mod operations;

pub use operations::*;

use crate::{link::Link, Element, EventListener, Node};
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};

impl Node {
    pub fn to_node(&mut self, link: &Link) -> web_sys::Node {
        match self {
            Node::Element(element) => element.to_node(link),
            Node::Text(text) => create_text_node(&document(), text).unchecked_into(),
            Node::AsyncElement(async_element) => {
                let pending_node = async_element.pending.to_node(link);
                if let Some(fut) = async_element.ready.take() {
                    let link = link.clone();
                    let pending_node = pending_node.clone();
                    spawn_local(async move {
                        let ready_node = fut.await.to_node(&link);
                        replace_with(pending_node.unchecked_ref(), &ready_node);
                    });
                }
                pending_node
            }
        }
    }
}

impl Element {
    pub fn to_node(&mut self, link: &Link) -> web_sys::Node {
        let el = create_element(&document(), &self.tag);

        // set attributes
        for attr in &self.attrs {
            attr.set(&el);
        }

        add_listeners(el.unchecked_ref(), &self.listeners, link);

        // set innerHTML or append children
        if let Some(html) = &self.inner_html {
            el.set_inner_html(html);
        } else {
            for child in &mut self.children {
                let child_node = child.to_node(link);
                append_child(&el, &child_node);
            }
        }

        el.unchecked_into()
    }

    pub fn hydrate(&self, parent: &web_sys::Element, idx: usize, link: &Link) {
        if let Some(el) = parent.children().item(idx as u32) {
            add_listeners(el.unchecked_ref(), &self.listeners, link);

            // hydrate children
            for (idx, child) in self
                .children
                .iter()
                .filter_map(|node| match node {
                    Node::Element(el) => Some(el),
                    // TODO: hydrating AsyncElement?
                    _ => None,
                })
                .enumerate()
            {
                child.hydrate(&el, idx, link);
            }
        } else {
            debug_warn(&format!(
                "could not find child #{} on {}\n\nchild node was {:#?}",
                idx,
                parent.outer_html(),
                self
            ))
        }
    }
}

pub fn add_listeners(node: &web_sys::Node, listeners: &[EventListener], link: &Link) {
    for listener in listeners {
        add_listener(listener, node, link);
    }
}

pub fn add_listener(listener: &EventListener, node: &web_sys::Node, link: &Link) {
    if let Some(handler) = &listener.handler {
        let link = link.clone();
        let handler = handler.clone();
        let event_handler = move |ev: web_sys::Event| {
            let e = handler.invoke(ev);
            if let Err(e) = link.send(e.as_any()) {
                debug_warn(&format!("[add_listeners] {}", e));
            }
        };
        let js_closure = Closure::wrap(Box::new(event_handler) as Box<dyn Fn(_)>).into_js_value();
        node.add_event_listener_with_callback(&listener.event_name, js_closure.unchecked_ref())
            .unwrap_throw();
        *listener.js_callback.borrow_mut() = Some(js_closure);
    }
}

pub fn remove_listener(listener: &EventListener, node: &web_sys::Node) {
    if let Some(js_callback) = listener.js_callback.borrow().clone() {
        node.remove_event_listener_with_callback(&listener.event_name, js_callback.unchecked_ref());
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
            let ev_name = listener.event_name.clone();
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
            let js_closure =
                Closure::wrap(Box::new(event_handler) as Box<dyn Fn(_)>).into_js_value();
            node.add_event_listener_with_callback(&listener.event_name, js_closure.unchecked_ref())
                .unwrap_throw();
            *listener.js_callback.borrow_mut() = Some(js_closure);
        }
    }
}
