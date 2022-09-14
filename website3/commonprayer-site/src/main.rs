use commonprayer_site::{App, AppProps};
use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount_to_body(|cx| view! { <App/> });
}
