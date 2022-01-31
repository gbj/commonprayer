use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::hymnal;

#[wasm_bindgen]
pub fn hydrate_hymnal(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(hymnal, &locale, serialized_state)
}
