use leptos::window;
use thiserror::Error;
use wasm_bindgen_futures::JsFuture;

#[derive(Error, Debug)]
pub enum ClipboardError {
    #[error("Clipboard API not available")]
    NotAvailable,
    #[error("Could not write text to clipboard")]
    Write,
}

#[cfg(web_sys_unstable_apis)]
pub async fn copy_text(text: &str) -> Result<(), ClipboardError> {
    let clipboard = window()
        .navigator()
        .clipboard()
        .ok_or(ClipboardError::NotAvailable)?;
    JsFuture::from(clipboard.write_text(text))
        .await
        .map_err(|_| ClipboardError::Write)?;
    Ok(())
}

#[cfg(not(web_sys_unstable_apis))]
pub async fn copy_text(_text: &str) -> Result<(), ClipboardError> {
    Err(ClipboardError::NotAvailable)
}
