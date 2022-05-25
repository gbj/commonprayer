use std::{fmt::Debug, rc::Rc};

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::{ser_attr, Attribute};

// Property generics
pub fn property(name: impl std::fmt::Display, value: impl IntoProperty + 'static) -> Attribute {
    Attribute::Property(name.to_string(), PropertyValue(Rc::new(value)))
}

#[derive(Clone, Debug)]
pub struct PropertyValue(Rc<dyn IntoProperty>);

pub trait IntoProperty
where
    Self: Debug,
{
    fn to_property(&self) -> Option<JsValue>;

    fn serialize(&self) -> String;
}

impl IntoProperty for PropertyValue {
    fn to_property(&self) -> Option<JsValue> {
        self.0.to_property()
    }

    fn serialize(&self) -> String {
        self.0.serialize()
    }
}

impl PartialEq for PropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_property() == other.0.to_property()
    }
}

impl<T> IntoProperty for T
where
    T: Serialize + Debug,
{
    fn to_property(&self) -> Option<JsValue> {
        JsValue::from_serde(&self).ok()
    }

    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
