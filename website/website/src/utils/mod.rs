use leptos::is_server;
use wasm_bindgen::JsCast;
use web_sys::Event;

pub mod fetch;
pub mod language;
pub mod preferences;
pub mod time;

pub fn decode_uri(encoded: &str) -> String {
    // TODO
    if is_server!() {
        urlencoding::decode(encoded)
            .map(String::from)
            .unwrap_or_else(|_| encoded.to_string())
    } else {
        js_sys::decode_uri(encoded)
            .unwrap()
            .as_string()
            .unwrap_or_default()
    }
}

pub fn event_target_checked(ev: Event) -> bool {
    ev.target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .checked()
}
