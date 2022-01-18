use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::settings;

#[wasm_bindgen]
pub fn hydrate_settings(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(settings, &locale, serialized_state)
}
