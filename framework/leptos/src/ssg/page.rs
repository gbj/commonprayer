use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{self as leptos, is_server};
use crate::{view, View};

use thiserror::Error;

/// Takes locale (as `&str`), hydration state (`T`), and render state (`R`) to render a [View](leptos::View)
type BodyFn<T, R> = fn(&str, &T, &R) -> View;
type HeadFn<T, R> = BodyFn<T, R>;

/// Takes locale (as `&str`), path (as `&str`), and typed path params to create static props (`T`).
/// `None` should lead to a 404 error.
type HydrationStateFn<T, P> = fn(&str, &str, &P) -> Option<T>;

/// State that is known only on the server-side, and never serialized and sent.
type RenderStateFn<R, P> = fn(&str, &str, &P) -> Option<R>;

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
pub struct Page<T, P, R>
where
    T: Serialize + DeserializeOwned, // hydration state
    P: DeserializeOwned,             // URL params
    R: Default, // render-only state (known only on server-side, never serialized)
{
    pub name: &'static str,
    head: Option<HeadFn<T, R>>,
    body: Option<BodyFn<T, R>>,
    hydration_state_fn: Option<HydrationStateFn<T, P>>,
    render_state_fn: Option<RenderStateFn<R, P>>,
    build_paths: Option<BuildPathsFn>,
    incremental_generation: bool,
    static_page: bool,
}

impl<T, P, R> Page<T, P, R>
where
    T: Serialize + DeserializeOwned,
    P: DeserializeOwned,
    R: Default,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            head: None,
            body: None,
            hydration_state_fn: None,
            render_state_fn: None,
            build_paths: None,
            incremental_generation: false,
            static_page: false,
        }
    }

    pub fn head_fn(mut self, head_fn: HeadFn<T, R>) -> Self {
        self.head = Some(head_fn);
        self
    }

    pub fn body_fn(mut self, body_fn: BodyFn<T, R>) -> Self {
        self.body = Some(body_fn);
        self
    }

    pub fn get_body_fn(&self) -> Option<BodyFn<T, R>> {
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

    pub fn hydration_state(mut self, hydration_state_fn: HydrationStateFn<T, P>) -> Self {
        // logic and any embedded data should not be included on WASM target
        if !cfg!(target_arch = "wasm32") {
            self.hydration_state_fn = Some(hydration_state_fn);
            self
        } else {
            self
        }
    }

    pub fn render_state(mut self, render_state_fn: RenderStateFn<R, P>) -> Self {
        // render state logic and any embedded data should not be included on WASM target
        if !cfg!(target_arch = "wasm32") {
            self.render_state_fn = Some(render_state_fn);
            self
        } else {
            self
        }
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
        global_body_code: Option<View>,
    ) -> Result<View, PageRenderError> {
        let hydration_state = (self.hydration_state_fn)
            .expect("a Page should have a defined hydrate_fn to before build() is called")(
            locale, path, &params,
        )
        .ok_or(PageRenderError::NotFound)?;

        let render_state = if is_server!() {
            (self.render_state_fn).and_then(|f| (f)(locale, path, &params))
        } else {
            None
        }
        .unwrap_or_default();

        self.render(locale, hydration_state, render_state, global_body_code)
    }

    pub fn render(
        &self,
        locale: &str,
        hydration_state: T,
        render_state: R,
        global_body_code: Option<View>,
    ) -> Result<View, PageRenderError> {
        let hydration_function_name = self.name.replace('-', "_");
        let serialized_state = serde_json::to_string(&hydration_state)
            .map_err(|_| PageRenderError::SerializingProps)?
            .replace('\\', "\\\\");
        let hydration_js = format!(
            r#"
                import init, {{ hydrate_{} }} from '/pkg/{}_page.js';
                const state = JSON.parse(`{}`);
                async function main() {{
                    await init();
                    hydrate_{}("{}", state);
                }}
                main();
                "#,
            &hydration_function_name,
            &hydration_function_name,
            serialized_state,
            &hydration_function_name,
            locale
        );
        Ok(view! {
            <!DOCTYPE html>
            <html lang={locale}>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    {self.head.map(|head_fn| (head_fn)(locale, &hydration_state, &render_state)).unwrap_or(View::Empty)}
                </head>
                <body>
                    // the page's body
                    {self.body.map(|body_fn| (body_fn)(locale, &hydration_state, &render_state)).unwrap_or(View::Empty)}

                    // additional code to be injected -- can be specified by server
                    {global_body_code.unwrap_or(View::Empty)}

                    // hydration code
                    {if self.static_page {
                        View::Empty}
                    else {
                        view! {
                            <script type="module">
                            {hydration_js}
                            </script>
                        }
                    }}
                </body>
            </html>
        })
    }
}
