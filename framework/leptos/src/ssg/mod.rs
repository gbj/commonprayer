mod page;

pub use page::*;

#[macro_export]
macro_rules! is_server {
    () => {
        !cfg!(target_arch = "wasm32")
    };
}
