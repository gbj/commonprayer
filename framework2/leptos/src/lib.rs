#![allow(unused_braces)]
#![feature(negative_impls)]
#![feature(auto_traits)]

pub use async_trait::async_trait;
pub use leptos_macro2::*;
pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use serde_wasm_bindgen;

mod component;
mod custom_element;
mod dom;
mod error;
mod event_emitter;
mod page;
pub mod ssr;
pub mod vdom;

use std::any::Any;

pub use component::*;
pub use custom_element::*;
pub use dom::*;
pub use error::*;
pub use event_emitter::*;
pub use page::*;
pub use ssr::*;
pub use vdom::*;

pub use web_sys::Event;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T> AsAny for T
where
    T: Any + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[macro_export]
macro_rules! ser_attr {
    ($val:expr) => {
        serde_wasm_bindgen::to_value(&$val).unwrap()
    };
}

#[macro_export]
macro_rules! de_attr {
    ($val:expr) => {
        serde_wasm_bindgen::from_value(&$val).unwrap()
    };
}
