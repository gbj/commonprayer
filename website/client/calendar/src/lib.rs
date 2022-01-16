use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::calendar;

#[wasm_bindgen]
pub fn hydrate_calendar(serialized_state: String) -> Result<(), JsValue> {
    hydrate_page(calendar, &serialized_state)
}
