use leptos::is_server;

pub mod fetch;
pub mod language;
pub mod time;

pub fn decode_uri(encoded: &str) -> String {
    // TODO
    if is_server!() {
        encoded.to_string()
    } else {
        unsafe { js_sys::decode_uri(encoded) }
            .unwrap()
            .as_string()
            .unwrap_or_default()
    }
}
