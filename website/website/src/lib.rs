#![allow(unused_braces)]
#![feature(iter_intersperse)]

pub mod api;
mod components;
pub mod pages;
pub mod preferences;
mod table_of_contents;
pub mod utils;

pub use table_of_contents::*;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("translations");
