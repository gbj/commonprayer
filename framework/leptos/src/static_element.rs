use crate::{attribute::StaticAttributeValue, child::IntoViewChild, view::View};

/// An HTML [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// whose attributes are known at render time, and which has no event listeners
/// attached to it. This will be rendered to HTML by the server, or rendered to a DOM
/// element using client-side rendering, but will be ignored by client-side hydration.
pub struct StaticElement {
    pub(crate) tag_name: &'static str,
    pub(crate) attributes: Vec<(&'static str, StaticAttributeValue)>,
    pub(crate) children: Vec<View>,
}

impl StaticElement {
    pub fn new(tag_name: &'static str) -> Self {
        Self {
            tag_name,
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoViewChild) -> Self {
        self.children.push(child.into_view_child());
        self
    }

    pub fn attribute(
        mut self,
        attr_name: &'static str,
        value: impl Into<StaticAttributeValue>,
    ) -> Self {
        self.attributes.push((attr_name, value.into()));
        self
    }
}
