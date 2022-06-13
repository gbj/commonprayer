use crate::{
    add_foreign_listeners, add_listeners, append_child, document, link::Link, window, Attribute,
    IntoChildren, Node,
};
use wasm_bindgen::{JsCast, prelude::*};

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
        &mut self,
        host: &web_sys::HtmlElement,
        shadow_root: &web_sys::ShadowRoot,
        link: &Link,
    ) {
        for attr in &self.host_attrs {
            attr.set(host);
        }

        add_listeners(host.unchecked_ref(), &mut self.listeners, link);
        if !self.window_listeners.is_empty() {
            add_listeners(window().unchecked_ref(), &mut self.window_listeners, link);
        }
        if !self.document_listeners.is_empty() {
            add_listeners(
                document().unchecked_ref(),
                &mut self.document_listeners,
                link,
            );
        }

        if !self.foreign_listeners.is_empty() {
            add_foreign_listeners(window().unchecked_ref(), &mut self.foreign_listeners, link);
        }

        for child in &mut self.children {
            let child = child.to_node(link);
            append_child(shadow_root.unchecked_ref(), &child);
        }

        observe_custom_elements(shadow_root.unchecked_ref());
    }

    pub fn hydrate(
        &mut self,
        host: &web_sys::HtmlElement,
        shadow_root: &web_sys::ShadowRoot,
        link: &Link,
    ) {
        add_listeners(host.unchecked_ref(), &mut self.listeners, link);
        if !self.window_listeners.is_empty() {
            add_listeners(window().unchecked_ref(), &mut self.window_listeners, link);
        }
        if !self.document_listeners.is_empty() {
            add_listeners(
                document().unchecked_ref(),
                &mut self.document_listeners,
                link,
            );
        }

        if !self.foreign_listeners.is_empty() {
            add_foreign_listeners(window().unchecked_ref(), &mut self.foreign_listeners, link);
        }

        for (idx, child) in self
            .children
            .iter_mut()
            .filter_map(|node| match node {
                Node::Element(el) => Some(el),
                Node::Text(_) => None,
            })
            .enumerate()
        {
            child.hydrate(shadow_root.unchecked_ref(), idx, link);
        }
        
        observe_custom_elements(shadow_root.unchecked_ref());
    }
}

#[wasm_bindgen(module = "/src/hydration.js")]
extern "C" {
    fn observe_custom_elements(
        root: &web_sys::HtmlElement
    );
}