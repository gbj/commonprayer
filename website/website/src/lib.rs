#![allow(unused_braces)]

mod components;
pub mod pages;
mod table_of_contents;
pub mod utils;

pub use table_of_contents::TABLE_OF_CONTENTS;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("translations");
