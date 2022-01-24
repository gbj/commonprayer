use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

use crate::{body, log, set_panic_hook, window, Page, PageRenderError, DARK_MODE_KEY};

pub fn hydrate_page<T, P>(
    page: fn() -> Page<T, P>,
    locale: &str,
    serialized_state: JsValue,
) -> Result<(), JsValue>
where
    T: Serialize + DeserializeOwned,
    P: DeserializeOwned,
{
    set_panic_hook();

    let props = serialized_state
        .into_serde()
        .map_err(|_| PageRenderError::DeserializingProps.to_string())?;

    let body = body().unwrap();

    // set dark mode from user preference, as necessary
    if let Ok(Some(local_storage)) = window().local_storage() {
        if let Ok(Some(dark_mode_pref)) = local_storage.get(DARK_MODE_KEY) {
            if let Err(e) = body
                .class_list()
                .add_1(&format!("dark-mode-{}", dark_mode_pref))
            {
                log(&format!(
                    "[hydration] error while setting Dark Mode class: {:#?}",
                    e
                ));
            }
        }
    }

    // hydrate the page
    if let Some(body_fn) = (page)().get_body_fn() {
        (body_fn)(locale, &props).hydrate(&body);
    }

    Ok(())
}
