use serde::{Deserialize, Serialize};

use crate::{Attribute, IntoChildren, Node};

use super::event::EventListener;

#[derive(Clone, PartialEq, Debug)]
pub struct Element {
    pub tag: String,
    pub key: Option<String>,
    pub attrs: Vec<Attribute>,
    pub listeners: Vec<EventListener>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(&self, tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            key: None,
            attrs: Vec::new(),
            listeners: Vec::new(),
            children: Vec::new(),
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
