use serde::{Deserialize, Serialize};

mod async_element;
mod attribute;
mod children;
mod element;
mod event;
mod host;
mod patch;
mod patch_against_dom;
mod property;

pub use async_element::*;
pub use attribute::*;
pub use children::*;
pub use element::*;
pub use event::*;
pub use host::*;
pub use patch::*;
pub use patch_against_dom::*;
pub use property::*;

use crate::{append_child, link::Link};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Node {
    Element(Element),
    Text(String),
    AsyncElement(AsyncElement),
}

impl Node {
    pub fn key(&self) -> Option<&String> {
        if let Node::Element(el) = self {
            el.key.as_ref()
        } else {
            None
        }
    }

    pub fn mount(&mut self, parent: &web_sys::HtmlElement, link: &Link) {
        let child = self.to_node(link);
        append_child(parent, &child);
    }
}

pub fn text(text: impl std::fmt::Display) -> Node {
    Node::Text(text.to_string())
}

pub trait StaticView {
    fn to_node(&self) -> Node;
}

impl Default for Node {
    fn default() -> Self {
        Node::Text(String::new())
    }
}
