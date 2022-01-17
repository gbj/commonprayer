use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::calendar;

#[wasm_bindgen]
pub fn hydrate_calendar(locale: String, serialized_state: JsValue) -> Result<(), JsValue> {
    hydrate_page(calendar, &locale, serialized_state)
}
