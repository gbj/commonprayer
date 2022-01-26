use std::pin::Pin;

use futures::Stream;

use crate::{attribute::DynamicAttributeValue, child::IntoDynamicViewChild, view::View};

/// An HTML [Element](https://developer.mozilla.org/en-US/docs/Web/API/Element)
/// that can be rendered partially on the server side, but will need to be hydrated on the
/// client side because the value of its attributes or properties may change,
/// or it needs event listeners attached.
pub struct DynamicElement {
    pub(crate) tag_name: &'static str,
    pub(crate) attributes: Vec<(&'static str, DynamicAttributeValue)>,
    pub(crate) classes: Vec<(&'static str, ClassStream)>,
    pub(crate) styles: Vec<(&'static str, StyleStream)>,
    pub(crate) properties: Vec<(&'static str, PropertyStream)>,
    pub(crate) event_listeners: Vec<(&'static str, EventListener)>,
    pub(crate) children: Vec<View>,
}

pub type ClassStream = Pin<Box<dyn Stream<Item = bool>>>;
pub type PropertyStream = Pin<Box<dyn Stream<Item = Option<String>>>>;
pub type StyleStream = Pin<Box<dyn Stream<Item = String>>>;
pub type EventListener = Box<dyn Fn(web_sys::Event)>;

pub enum TextValue {
    Static(String),
    Dynamic(Pin<Box<dyn Stream<Item = String>>>),
}

impl From<String> for TextValue {
    fn from(value: String) -> Self {
        Self::Static(value)
    }
}

impl From<&str> for TextValue {
    fn from(value: &str) -> Self {
        Self::Static(value.to_owned())
    }
}

impl From<Pin<Box<dyn Stream<Item = String> + Send>>> for TextValue {
    fn from(value: Pin<Box<dyn Stream<Item = String> + Send>>) -> Self {
        Self::Dynamic(value)
    }
}

impl DynamicElement {
    pub fn new(tag_name: &'static str) -> Self {
        Self {
            tag_name,
            attributes: Vec::new(),
            classes: Vec::new(),
            styles: Vec::new(),
            properties: Vec::new(),
            event_listeners: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoDynamicViewChild) -> Self {
        self.children.push(child.into_dynamic_view_child());
        self
    }

    pub fn attribute(
        mut self,
        attr_name: &'static str,
        value: impl Into<DynamicAttributeValue>,
    ) -> Self {
        self.attributes.push((attr_name, value.into()));
        self
    }

    pub fn class(mut self, class_name: &'static str, value: ClassStream) -> Self {
        self.classes.push((class_name, value));
        self
    }

    pub fn style(mut self, style_name: &'static str, value: StyleStream) -> Self {
        self.styles.push((style_name, value));
        self
    }

    pub fn property(mut self, prop_name: &'static str, value: PropertyStream) -> Self {
        self.properties.push((prop_name, value));
        self
    }

    pub fn event(mut self, event_name: &'static str, value: Box<dyn Fn(web_sys::Event)>) -> Self {
        self.event_listeners.push((event_name, value));
        self
    }
}
