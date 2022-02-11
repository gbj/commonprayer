mod biblical_citation;
mod date_picker;
mod document;
mod export_links;
mod header;
mod icon;
mod js_redirect_to_current_date;
mod lookup;
mod menu;
mod observance_picker;
mod search_bar;
mod segment_button;
mod toggle;

pub use biblical_citation::*;
pub use date_picker::*;
pub use document::*;
pub use export_links::*;
pub use header::*;
pub use icon::*;
pub use js_redirect_to_current_date::*;
use leptos::{view, View};
pub use menu::*;
pub use observance_picker::*;
pub use search_bar::*;
pub use segment_button::*;
pub use toggle::*;

// Plausible.io is an open-source analytics software as a service that uses no cookies and collects/sells no user data
// It is an alternative to Google Analytics, etc. with strong privacy protections
// Rather than an advertising based model, I pay a subscription fee to support their service
pub fn analytics() -> View {
    view! {
        <script defer data-domain="commonprayeronline.org" src="https://plausible.io/js/plausible.js"></script>
    }
}
