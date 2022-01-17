use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::daily_office;

#[wasm_bindgen]
pub fn hydrate_daily_office(serialized_state: String) -> Result<(), JsValue> {
    hydrate_page(daily_office, &serialized_state)
}
