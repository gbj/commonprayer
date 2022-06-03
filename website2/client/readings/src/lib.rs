use app::{components::*, pages::*};
use leptos2::{Page, WebComponent};
use wasm_bindgen::prelude::wasm_bindgen;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn define_custom_elements() {
    // readings
    Tabs::define();
    DailyOfficeView::define();
    Toggle::define();
    DatePicker::define();
    BiblicalCitationLoader::define();
}
