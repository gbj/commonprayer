use app::pages::*;
use leptos2::Page;
use wasm_bindgen::prelude::wasm_bindgen;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn define_custom_elements() {}

#[wasm_bindgen]
pub fn on_load() {
    DailyOfficePage::on_load();
}
