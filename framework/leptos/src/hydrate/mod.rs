use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

use crate::{body, set_panic_hook, Page, PageRenderError};

pub fn hydrate_page<T, P>(page: fn() -> Page<T, P>, serialized_state: &str) -> Result<(), JsValue>
where
    T: Serialize + DeserializeOwned,
    P: DeserializeOwned,
{
    set_panic_hook();

    let locale = "en"; // TODO

    let props = serde_json::from_str(serialized_state)
        .map_err(|_| PageRenderError::DeserializingProps.to_string())?;

    let body = body().unwrap();

    if let Some(body_fn) = (page)().get_body_fn() {
        (body_fn)(locale, &props).hydrate(&body);
    }

    Ok(())
}
