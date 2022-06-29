use std::{pin::Pin, sync::Arc};

use futures::Future;
use serde::{Deserialize, Serialize};

use crate::{debug_warn, Element, Node};

pub struct AsyncElement {
    pub pending: Box<Node>,
    pub ready: Option<Pin<Box<dyn Future<Output = Node>>>>,
}

// TODO yikes
impl Clone for AsyncElement {
    fn clone(&self) -> Self {
        crate::debug_warn(&format!("You are cloning an AsyncElement. This is probably not what you're trying to do; it will never resolve, as the Future inside can't be cloned.\n\n{}", std::panic::Location::caller()));
        Self {
            pending: self.pending.clone(),
            ready: None,
        }
    }
}

impl std::fmt::Debug for AsyncElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncElement")
            .field("pending", &self.pending)
            .field("ready", &"<opaque Future<Output = Element>>")
            .finish()
    }
}

impl PartialEq for AsyncElement {
    fn eq(&self, other: &Self) -> bool {
        self.pending == other.pending
    }
}

impl Serialize for AsyncElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.pending.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AsyncElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let pending = Node::deserialize(deserializer)?;
        Ok(Self {
            pending: Box::new(pending),
            ready: None,
        })
    }
}

impl<F> From<(Node, F)> for AsyncElement
where
    F: Future<Output = Node> + 'static,
{
    fn from((pending, ready): (Node, F)) -> Self {
        Self {
            pending: Box::new(pending),
            ready: Some(Box::pin(ready)),
        }
    }
}
