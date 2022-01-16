use leptos::hydrate_page;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use website::pages::canticle_table;

#[wasm_bindgen]
pub fn hydrate_canticle_table(serialized_state: String) -> Result<(), JsValue> {
    hydrate_page(canticle_table, &serialized_state)
}
