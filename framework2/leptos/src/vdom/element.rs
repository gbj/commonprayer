use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{Attribute, Host, IntoChildren, Node};

use super::event::EventListener;

#[derive(Clone)]
pub struct Element {
    pub tag: String,
    pub key: Option<String>,
    pub attrs: Vec<Attribute>,
    pub listeners: Vec<EventListener>,
    pub children: Vec<Node>,
    pub shadow_root: Option<Rc<dyn Fn() -> Host>>,
}

impl Element {
    pub fn new(&self, tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            key: None,
            attrs: Vec::new(),
            listeners: Vec::new(),
            children: Vec::new(),
            shadow_root: None,
        }
    }

    pub fn is_self_closing(&self) -> bool {
        matches!(
            self.tag.as_str(),
            "area"
                | "base"
                | "br"
                | "col"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "param"
                | "source"
                | "track"
                | "wbr"
        )
    }

    pub fn has_keyed_child(&self) -> bool {
        self.children.iter().any(|child| {
            if let Node::Element(el) = child {
                el.key.is_some()
            } else {
                false
            }
        })
    }

    pub fn child(mut self, child: impl IntoChildren) -> Self {
        for child in child.into_children() {
            self.children.push(child);
        }
        self
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
            && self.key == other.key
            && self.attrs == other.attrs
            && self.listeners == other.listeners
            && self.children == other.children
            && match (&self.shadow_root, &other.shadow_root) {
                (Some(lhs), Some(rhs)) => (lhs)() == (rhs)(),
                _ => false,
            }
    }
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("tag", &self.tag)
            .field("key", &self.key)
            .field("attrs", &self.attrs)
            .field("listeners", &self.listeners)
            .field("children", &self.children)
            .finish()
    }
}
