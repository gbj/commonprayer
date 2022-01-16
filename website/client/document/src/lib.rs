use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::document;

#[wasm_bindgen]
pub fn hydrate_document(serialized_state: String) -> Result<(), JsValue> {
    hydrate_page(document, &serialized_state)
}
