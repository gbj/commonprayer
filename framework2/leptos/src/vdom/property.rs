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

/*
impl<T> IntoProperty for Box<T>
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

macro_rules! property_type {
    ($prop_type:ty) => {
        impl IntoProperty for $prop_type {
            fn to_property(&self) -> Option<JsValue> {
                Some(self.clone().into())
            }

            fn serialize(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }
        }

        impl IntoProperty for Option<$prop_type> {
            fn to_property(&self) -> Option<JsValue> {
                self.clone().map(|val| val.into())
            }

            fn serialize(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }
        }
    };
}

property_type!(String);
property_type!(&String);
property_type!(&str);
property_type!(bool);
property_type!(usize);
property_type!(u8);
property_type!(u16);
property_type!(u32);
property_type!(u64);
property_type!(u128);
property_type!(isize);
property_type!(i8);
property_type!(i16);
property_type!(i32);
property_type!(i64);
property_type!(i128);
property_type!(f32);
property_type!(f64);
 */
