#![allow(unused_braces)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(type_name_of_val)]

pub use async_trait::async_trait;
pub use leptos_macro2::*;
pub use serde::{Deserialize, Serialize};
pub use serde_json;

mod component;
mod custom_element;
mod dom;
mod error;
mod event_emitter;
mod page;
pub mod ssr;
mod state;
pub mod vdom;
mod cmd;
pub mod router;
pub mod view;

use std::any::Any;

pub use cmd::*;
pub use component::*;
pub use custom_element::*;
pub use dom::*;
pub use error::*;
pub use event_emitter::*;
pub use link::*;
pub use page::*;
pub use ssr::*;
pub use state::*;
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
        wasm_bindgen::JsValue::from_serde(&$val).unwrap()
    };
}

#[macro_export]
macro_rules! de_attr {
    ($val:expr) => {
        wasm_bindgen::JsValue::to_serde(&$val).unwrap()
    };
}
