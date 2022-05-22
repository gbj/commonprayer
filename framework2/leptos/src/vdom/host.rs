use crate::{
    add_foreign_listeners, add_listeners, append_child, document, link::Link, window, Attribute,
    IntoChildren, Node,
};
use wasm_bindgen::JsCast;

use super::event::EventListener;

#[derive(Clone, PartialEq, Debug)]
pub struct Host {
    pub tag: &'static str,
    pub host_attrs: Vec<Attribute>,
    pub state_attrs: Vec<Attribute>,
    pub listeners: Vec<EventListener>,
    pub window_listeners: Vec<EventListener>,
    pub document_listeners: Vec<EventListener>,
    pub foreign_listeners: Vec<(String, EventListener)>,
    pub children: Vec<Node>,
}

impl Host {
    pub fn child(mut self, child: impl IntoChildren) -> Self {
        for child in child.into_children() {
            self.children.push(child);
        }
        self
    }

    pub fn mount(
        &self,
        host: &web_sys::HtmlElement,
        shadow_root: &web_sys::ShadowRoot,
        link: &Link,
    ) {
        for attr in &self.host_attrs {
            attr.set(host);
        }

        add_listeners(host.unchecked_ref(), &self.listeners, link);
        if !self.window_listeners.is_empty() {
            add_listeners(window().unchecked_ref(), &self.window_listeners, link);
        }
        if !self.document_listeners.is_empty() {
            add_listeners(document().unchecked_ref(), &self.document_listeners, link);
        }
        if !self.foreign_listeners.is_empty() {
            add_foreign_listeners(window().unchecked_ref(), &self.foreign_listeners, link);
        }

        for child in &self.children {
            let child = child.to_node(link);
            append_child(shadow_root.unchecked_ref(), &child);
        }
    }

    pub fn hydrate(
        &self,
        host: &web_sys::HtmlElement,
        shadow_root: &web_sys::ShadowRoot,
        link: &Link,
    ) {
        add_listeners(host.unchecked_ref(), &self.listeners, link);
        if !self.window_listeners.is_empty() {
            add_listeners(window().unchecked_ref(), &self.window_listeners, link);
        }
        if !self.document_listeners.is_empty() {
            add_listeners(document().unchecked_ref(), &self.document_listeners, link);
        }
        if !self.foreign_listeners.is_empty() {
            add_foreign_listeners(window().unchecked_ref(), &self.foreign_listeners, link);
        }

        for (idx, child) in self
            .children
            .iter()
            .filter_map(|node| match node {
                Node::Element(el) => Some(el),
                Node::Text(_) => None,
            })
            .enumerate()
        {
            child.hydrate(shadow_root.unchecked_ref(), idx, link);
        }
    }
}
