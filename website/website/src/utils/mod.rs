use leptos::is_server;

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
