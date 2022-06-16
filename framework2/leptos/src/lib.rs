#![allow(unused_braces)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(type_name_of_val)]

pub use async_trait::async_trait;
pub use leptos_macro2::*;
pub use serde::{Deserialize, Serialize};
pub use serde_json;

mod cmd;
mod component;
mod custom_element;
mod dom;
mod error;
mod event_emitter;
mod page;
pub mod router;
pub mod ssr;
pub mod ssr_streaming;
mod state;
pub mod vdom;
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
pub use router::*;
pub use ssr::*;
pub use ssr_streaming::*;
pub use state::*;
pub use vdom::*;
pub use view::*;

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

pub fn body_scripts() -> Vec<Node> {
    use crate as leptos2;

    view! {
        <>
            <script>{include_str!("polyfills/declarative_shadow_dom.js")}</script>
            <script type="module">
                {include_str!("hydration.js")}
                "observe_custom_elements(document);"
            </script>
            <script>{include_str!("ssr_prop_selector.js")}</script>
        </>
    }
}
