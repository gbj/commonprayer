use leptos::{document, is_server, window};
use wasm_bindgen::JsCast;
use web_sys::{Event, ScrollToOptions};

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

pub fn scroll_to_element_by_id_with_padding_for_header(id: &str) {
    let el = document().get_element_by_id(id);
    if let Some(el) = el {
        // scroll into view, with some padding at the top for the menu
        // uses scroll_by rather than scroll_to because the DomRect is apparently relative to the current position
        let rect = el.get_bounding_client_rect();
        window().scroll_by_with_scroll_to_options(ScrollToOptions::new().top(rect.y() - 150.0));
    }
}
