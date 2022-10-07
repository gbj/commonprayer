use commonprayer_site::*;
use leptos::*;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    console_log::init_with_level(log::Level::Debug);

    leptos::hydrate(body().unwrap(), move |cx| {
        provide_context(cx, RouterIntegrationContext(Rc::new(BrowserIntegration {})));

        view! {
            <App/>
        }
    });
}
