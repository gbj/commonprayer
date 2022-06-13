use std::borrow::Cow;

use leptos2::{document, is_server, window};
use web_sys::ScrollToOptions;

pub mod fetch;
//pub mod preferences;
pub mod share;
pub mod time;

pub fn encode_uri(unencoded: &str) -> Cow<str> {
    if is_server!() {
        urlencoding::encode(unencoded)
    } else {
        Cow::from(
            js_sys::decode_uri(unencoded)
                .unwrap()
                .as_string()
                .unwrap_or_default(),
        )
    }
}

pub fn decode_uri(encoded: &str) -> String {
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

pub fn scroll_to_element_by_id_with_padding_for_header(id: &str) {
    let el = document().get_element_by_id(id);
    if let Some(el) = el {
        // manually adding padding here is unnecessary because of CSS scroll-margin-top
        el.scroll_to()
    }
}
