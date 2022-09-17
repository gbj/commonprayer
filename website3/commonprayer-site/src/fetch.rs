use serde::de::DeserializeOwned;

#[cfg(not(feature = "ssr"))]
pub async fn fetch<T>(path: &str) -> Result<T, ()>
where
    T: DeserializeOwned,
{
    // serde-wasm-bindgen version
    /* let resp = gloo_net::http::Request::get(path)
        .send()
        .await
        .map_err(|e| log::error!("{e}"))?;
    let promise = resp
        .as_raw()
        .json()
        .map_err(|e| log::error!("{:?}", e.as_string()))?;
    let json = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| log::error!("{:?}", e.as_string()))?;
    let value = serde_wasm_bindgen::from_value::<T>(json).map_err(|e| log::error!("{e}"))?;
    Ok(value) */

    // serde-json version
    gloo_net::http::Request::get(path)
        .send()
        .await
        .map_err(|e| log::error!("{e}"))?
        .json::<T>()
        .await
        .map_err(|e| log::error!("{e}"))
}

#[cfg(feature = "ssr")]
pub async fn fetch<T>(path: &str) -> Result<T, ()>
where
    T: DeserializeOwned,
{
    reqwest::get(path)
        .await
        .map_err(|e| log::error!("{e}"))?
        .json::<T>()
        .await
        .map_err(|e| log::error!("{e}"))
}
