use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::document;

#[wasm_bindgen]
pub fn hydrate_document(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(document, &locale, serialized_state)
}
