use serde::de::DeserializeOwned;

#[cfg(not(feature = "ssr"))]
pub async fn fetch<T>(path: &str) -> Result<T, ()>
where
    T: DeserializeOwned,
{
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
