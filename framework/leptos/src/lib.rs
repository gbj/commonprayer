#![allow(unused_braces)]
#![allow(unused_unsafe)]

mod attribute;
mod child;
mod dynamic_element;
mod event_stream;
mod hydrate;
mod operations;
mod reactive;
mod ssg;
mod static_element;
mod view;

pub use child::IntoViewChild;
pub use dynamic_element::DynamicElement;
pub use event_stream::*;
pub use hydrate::hydrate_page;
pub use leptos_macro::view;
pub use operations::*;
pub use reactive::*;
pub use ssg::*;
pub use static_element::StaticElement;
pub use view::View;
pub use web_sys::Event;

pub const DARK_MODE_KEY: &str = "dark_mode";

use cfg_if::cfg_if;

cfg_if! {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function to get better error messages if we ever panic.
  if #[cfg(feature = "console_error_panic_hook")] {
      extern crate console_error_panic_hook;
      pub use console_error_panic_hook::set_once as set_panic_hook;
  } else {
      #[inline]
      pub fn set_panic_hook() {}
  }
}

cfg_if! {
  // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
  // allocator.
  if #[cfg(feature = "wee_alloc")] {
      extern crate wee_alloc;
      #[global_allocator]
      static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
  }
}
