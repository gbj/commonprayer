#![allow(unused_braces)]
#![feature(iter_intersperse)]

use leptos::*;

mod condition;
mod content;
mod editor;
use editor::*;

fn main() {
    let body = body();
    let editor = Editor::new();
    if let Some(body) = body {
        body.append_child(&editor.view().client_side_render())
            .expect("couldn't mount editor");
    }
}
