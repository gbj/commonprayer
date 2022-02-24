use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::lectionary;

#[wasm_bindgen]
pub fn hydrate_lectionary(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(lectionary, &locale, serialized_state)
}
