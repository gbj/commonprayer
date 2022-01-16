use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::index;

#[wasm_bindgen]
pub fn hydrate_index(serialized_state: String) -> Result<(), JsValue> {
    hydrate_page(index, &serialized_state)
}
