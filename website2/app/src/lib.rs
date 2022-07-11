#![allow(unused_braces)]
#![allow(unused_parens)]
#![allow(redundant_semicolons)]
#![feature(async_closure)]
#![feature(iter_intersperse)]

use leptos2::Node;
pub mod api;
pub mod components;
pub mod events;
mod icon;
pub mod preferences;
mod user_info;

#[cfg(feature = "routes")]
pub mod routes;
pub use routes::index::UserId;
pub mod utils;

pub use icon::Icon;
pub use user_info::UserInfo;

#[cfg(feature = "routes")]
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
#[cfg(feature = "routes")]
i18n!("translations");

pub trait WebView {
    fn view(&self, locale: &str) -> Node;
}
