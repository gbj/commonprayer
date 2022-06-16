use std::collections::HashMap;

use crate::{Node, RenderedView, Styles};

#[derive(Default)]
pub(crate) struct RenderedPartial {
    pub(crate) locale: String,
    pub(crate) title: String,
    pub(crate) links: Vec<Node>,
    pub(crate) meta: HashMap<String, String>,
    pub(crate) styles: Styles,
    pub(crate) body: Vec<Box<dyn FnOnce(Option<Node>) -> Node>>,
}

impl From<RenderedPartial> for RenderedView {
    fn from(partial: RenderedPartial) -> Self {
        let meta = partial.meta.into_iter().collect();
        let mut bodies = partial.body;
        bodies.reverse();
        let body = bodies
            .into_iter()
            .fold(None, |acc, curr| Some((curr)(acc)))
            .unwrap_or_default();

        RenderedView {
            locale: partial.locale,
            title: partial.title,
            meta,
            links: partial.links,
            styles: partial.styles,
            body,
        }
    }
}
