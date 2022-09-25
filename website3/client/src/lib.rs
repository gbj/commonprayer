use commonprayer_site::*;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() {
    console_log::init_with_level(log::Level::Debug);
    let integration = BrowserIntegration {};

    leptos::hydrate(body().unwrap(), move |cx| {
        view! {
            <div id="root">
                <Localizer>
                    <div>
                        <Router mode=BrowserIntegration {}>
                            <App />
                        </Router>
                    </div>
                </Localizer>
            </div>
        }
    });
}
