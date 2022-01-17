use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

use crate::{body, set_panic_hook, Page, PageRenderError};

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

    if let Some(body_fn) = (page)().get_body_fn() {
        (body_fn)(locale, &props).hydrate(&body);
    }

    Ok(())
}
