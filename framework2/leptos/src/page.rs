use crate::{text, view};

use crate::Node;
use crate::{self as leptos2};

use serde::de::DeserializeOwned;
use thiserror::Error;

pub trait Page
where
    Self: Sized,
{
    type Params: DeserializeOwned + 'static;
    type Query: DeserializeOwned + 'static;

    fn name() -> &'static str;

    fn paths() -> Vec<String>;

    fn build_state(
        locale: &str,
        path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self>;

    fn head(&self, locale: &str) -> Vec<Node>;

    fn body(&self, locale: &str) -> Vec<Node>;

    /// Runs only on the client side, and can be used to add arbitrary
    /// dynamic code to the initial page load without needing to create a web component.
    fn on_load() {}

    /// A page that generates HTML only, with no client-side Rust code to be run
    fn static_page() -> bool {
        false
    }

    /// A "pure" page depends only on its path, and not data from the server or session,
    /// so it can be incrementally generated and stored for future access.
    fn pure() -> bool {
        true
    }

    fn get_absolute_paths() -> Vec<String> {
        let mut paths = Self::paths()
            .iter()
            .map(|path| {
                let separator = if path.is_empty() { "" } else { "/" };
                format!("{}{}{}", Self::name(), separator, path)
            })
            .collect::<Vec<_>>();
        paths
    }

    fn render(&self, locale: &str, global_body_code: Option<Node>) -> Node {
        let name = Self::name().replace('-', "_");
        let hydration_js = format!(
            r#"
                import("/pkg/{}/{}_client.js").then(async pkg => {{
                    await pkg.default();
                    if(pkg.on_load) {{
                        pkg.on_load();
                    }}
                    pkg.define_custom_elements();
                }});
                "#,
            &name, &name,
        );

        let body = self.body(locale);

        view! {
            <html lang={locale}>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    {self.head(locale)}
                </head>
                <body>
                    // the page's body
                    {body}

                    // additional code to be injected -- can be specified by server
                    {global_body_code.unwrap_or_else(|| text(""))}

                    // on_load and Custom Element code
                    {if Self::static_page() {
                        text("")
                    } else {
                        view! {
                            <script type="module">
                            {hydration_js}
                            </script>
                        }
                    }}
                    <script>{include_str!("./polyfills/declarative_shadow_dom.js")}</script>
                </body>
            </html>
        }
    }
}

#[derive(Error, Debug)]
pub enum PageRenderError {
    #[error("could not find a page at that path")]
    NotFound,
    #[error("could not serialize props to JSON")]
    SerializingProps,
    #[error("could not deserialize props from JSON")]
    DeserializingProps,
}
