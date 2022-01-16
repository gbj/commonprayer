use std::pin::Pin;

use futures::Stream;

pub enum StaticAttributeValue {
    Static(String),
    StaticOption(Option<String>),
}

impl From<&str> for StaticAttributeValue {
    fn from(s: &str) -> Self {
        Self::Static(s.to_string())
    }
}

impl From<&String> for StaticAttributeValue {
    fn from(s: &String) -> Self {
        Self::Static(s.clone())
    }
}

impl From<String> for StaticAttributeValue {
    fn from(s: String) -> Self {
        Self::Static(s)
    }
}

impl From<Option<&str>> for StaticAttributeValue {
    fn from(s: Option<&str>) -> Self {
        Self::StaticOption(s.map(|s| s.to_string()))
    }
}

impl From<Option<&String>> for StaticAttributeValue {
    fn from(s: Option<&String>) -> Self {
        Self::StaticOption(s.cloned())
    }
}

impl From<Option<String>> for StaticAttributeValue {
    fn from(s: Option<String>) -> Self {
        Self::StaticOption(s)
    }
}

pub enum DynamicAttributeValue {
    Static(String),
    StaticOption(Option<String>),
    Dynamic(Pin<Box<dyn Stream<Item = String>>>),
    DynamicOption(Pin<Box<dyn Stream<Item = Option<String>>>>),
}

impl From<&str> for DynamicAttributeValue {
    fn from(s: &str) -> Self {
        Self::Static(s.to_string())
    }
}

impl From<&String> for DynamicAttributeValue {
    fn from(s: &String) -> Self {
        Self::Static(s.clone())
    }
}

impl From<String> for DynamicAttributeValue {
    fn from(s: String) -> Self {
        Self::Static(s)
    }
}

impl From<Option<&str>> for DynamicAttributeValue {
    fn from(s: Option<&str>) -> Self {
        Self::StaticOption(s.map(|s| s.to_string()))
    }
}

impl From<Option<&String>> for DynamicAttributeValue {
    fn from(s: Option<&String>) -> Self {
        Self::StaticOption(s.cloned())
    }
}

impl From<Option<String>> for DynamicAttributeValue {
    fn from(s: Option<String>) -> Self {
        Self::StaticOption(s)
    }
}

impl From<Pin<Box<dyn Stream<Item = String>>>> for DynamicAttributeValue {
    fn from(stream: Pin<Box<dyn Stream<Item = String>>>) -> Self {
        Self::Dynamic(stream)
    }
}

impl From<Pin<Box<dyn Stream<Item = Option<String>>>>> for DynamicAttributeValue {
    fn from(stream: Pin<Box<dyn Stream<Item = Option<String>>>>) -> Self {
        Self::DynamicOption(stream)
    }
}
