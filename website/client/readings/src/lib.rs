use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::readings;

#[wasm_bindgen]
pub fn hydrate_readings(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(readings, &locale, serialized_state)
}
