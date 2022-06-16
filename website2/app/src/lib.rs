#![allow(unused_braces)]
#![feature(iter_intersperse)]
#![feature(async_closure)]

use leptos2::Node;
pub mod api;
pub mod components;
pub mod events;
pub mod fragments;
pub mod pages;
pub mod preferences;
pub mod routes;
pub mod utils;
pub mod views;

#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("translations");

pub trait WebView {
    fn view(&self, locale: &str) -> Node;
}
