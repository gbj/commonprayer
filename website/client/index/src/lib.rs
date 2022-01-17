use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::index;

#[wasm_bindgen]
pub fn hydrate_index(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(index, &locale, serialized_state)
}
