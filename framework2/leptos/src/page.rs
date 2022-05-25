use crate::{serialize_props, text, view};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Node;
use crate::{self as leptos2};

use thiserror::Error;

/// Takes locale (as `&str`), page state (`T`), and render state (`R`) to render a [Node](Node)
type BodyFn<T> = fn(&str, &T) -> Vec<Node>;
type HeadFn<T> = BodyFn<T>;

/// Takes locale (as `&str`), path (as `&str`), and typed path params to create static props (`T`).
/// `None` should lead to a 404 error.
type HydrationStateFn<T, P> = fn(&str, &str, &P) -> Option<T>;

type BuildPathsFn = fn() -> Vec<String>;

#[derive(Error, Debug)]
pub enum PageRenderError {
    #[error("could not find a page at that path")]
    NotFound,
    #[error("could not serialize props to JSON")]
    SerializingProps,
    #[error("could not deserialize props from JSON")]
    DeserializingProps,
}

#[derive(Clone)]
pub struct Page<T, P>
where
    P: DeserializeOwned, // URL params
{
    pub name: &'static str,
    head: Option<HeadFn<T>>,
    body: Option<BodyFn<T>>,
    props_fn: Option<HydrationStateFn<T, P>>,
    build_paths: Option<BuildPathsFn>,
    pub incremental_generation: bool,
    pub static_page: bool,
}

impl<T, P> Page<T, P>
where
    T: Serialize + DeserializeOwned,
    P: DeserializeOwned,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            head: None,
            body: None,
            props_fn: None,
            build_paths: None,
            incremental_generation: false,
            static_page: false,
        }
    }

    pub fn head_fn(mut self, head_fn: HeadFn<T>) -> Self {
        self.head = Some(head_fn);
        self
    }

    pub fn body_fn(mut self, body_fn: BodyFn<T>) -> Self {
        self.body = Some(body_fn);
        self
    }

    pub fn incremental_generation(mut self) -> Self {
        self.incremental_generation = true;
        self
    }

    pub fn get_body_fn(&self) -> Option<BodyFn<T>> {
        self.body
    }

    pub fn build_paths_fn(mut self, build_paths_fn: BuildPathsFn) -> Self {
        // static paths logic and any embedded data should not be included on WASM target
        if !cfg!(target_arch = "wasm32") {
            self.build_paths = Some(build_paths_fn);
            self
        } else {
            self
        }
    }

    pub fn get_static_paths(&self) -> Vec<String> {
        if let Some(f) = self.build_paths {
            f()
        } else {
            vec!["".into()]
        }
    }

    pub fn get_absolute_paths(&self) -> Vec<String> {
        let mut paths = self
            .get_static_paths()
            .iter()
            .map(|path| {
                let separator = if path.is_empty() { "" } else { "/" };
                format!("{}{}{}", self.name, separator, path)
            })
            .collect::<Vec<_>>();
        if self.incremental_generation {
            paths.push(format!("{}/*", self.name));
        }
        paths
    }

    pub fn state(mut self, state_fn: HydrationStateFn<T, P>) -> Self {
        self.props_fn = Some(state_fn);
        self
    }

    pub fn static_page(mut self) -> Self {
        self.static_page = true;
        self
    }

    pub fn build(
        &self,
        locale: &str,
        path: &str,
        params: P,
        global_body_code: Option<Node>,
    ) -> Result<Node, PageRenderError> {
        let state = (self.props_fn)
            .expect("a Page should have a defined hydrate_fn to before build() is called")(
            locale, path, &params,
        )
        .ok_or(PageRenderError::NotFound)?;

        self.render(locale, state, global_body_code)
    }

    pub fn render(
        &self,
        locale: &str,
        state: T,
        global_body_code: Option<Node>,
    ) -> Result<Node, PageRenderError> {
        let name = self.name.replace('-', "_");
        let hydration_js = format!(
            r#"
                import("/pkg/{}/{}_client.js").then(async pkg => {{
                    await pkg.default();
                    pkg.define_custom_elements();
                }});
                "#,
            &name, &name,
        );

        let body = self
            .body
            .map(|body_fn| (body_fn)(locale, &state))
            .unwrap_or_default();

        let serialized_props = serialize_props(&body).map(|props| {
            view! {
                <script>"var _PROPS = " {props}";"</script>
            }
        });

        Ok(view! {
            <html lang={locale}>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    {self.head.map(|head_fn| (head_fn)(locale, &state)).unwrap_or_default()}
                </head>
                <body>
                    // the page's body
                    {body}

                    // additional code to be injected -- can be specified by server
                    {global_body_code.unwrap_or_else(|| text(""))}

                    // serialized state
                    {serialized_props}

                    // custom element code
                    {if self.static_page {
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
        })
    }
}
