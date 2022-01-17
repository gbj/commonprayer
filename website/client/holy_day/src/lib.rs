use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::holy_day;

#[wasm_bindgen]
pub fn hydrate_holy_day(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(holy_day, &locale, serialized_state)
}
