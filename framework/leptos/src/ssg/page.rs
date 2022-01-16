use serde::de::DeserializeOwned;
use serde::Serialize;

use crate as leptos;
use crate::{view, View};

use thiserror::Error;

/// Takes locale (as `&str`) and static props (`T`) to render a [View](leptos::View)
type BodyFn<T> = fn(&str, &T) -> View;
type HeadFn<T> = BodyFn<T>;

/// Takes locale (as `&str`), path (as `&str`), and typed path params to create static props (`T`)
type StaticPropsFn<T, P> = fn(&str, &str, P) -> T;

type BuildPathsFn = fn() -> Vec<String>;

#[derive(Error, Debug)]
pub enum PageRenderError {
    #[error("could not serialize props to JSON")]
    SerializingProps,
    #[error("could not deserialize props from JSON")]
    DeserializingProps,
}

#[derive(Clone)]
pub struct Page<T, P>
where
    T: Serialize + DeserializeOwned,
    P: DeserializeOwned,
{
    pub name: &'static str,
    head: Option<HeadFn<T>>,
    body: Option<BodyFn<T>>,
    static_props: Option<StaticPropsFn<T, P>>,
    build_paths: Option<BuildPathsFn>,
    incremental_generation: bool,
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
            static_props: None,
            build_paths: None,
            incremental_generation: false,
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

    pub fn static_props_fn(mut self, static_props_fn: StaticPropsFn<T, P>) -> Self {
        // static props logic and any embedded data should not be included on WASM target
        if !cfg!(target_arch = "wasm32") {
            self.static_props = Some(static_props_fn);
            self
        } else {
            self
        }
    }

    pub fn build(&self, locale: &str, path: &str, params: P) -> Result<View, PageRenderError> {
        let props = (self.static_props)
            .expect("a Page should have a defined static_props_fn to before build() is called")(
            locale, path, params,
        );
        self.render(locale, props)
    }

    pub fn render(&self, locale: &str, props: T) -> Result<View, PageRenderError> {
        let hydration_function_name = self.name.replace('-', "_");
        let serialized_state = serde_json::to_string(&props)
            .map_err(|_| PageRenderError::SerializingProps)?
            .replace('\\', "\\\\");
        let hydration_js = format!(
            "
                import init, {{ hydrate_{} }} from '/pkg/{}_page.js';
                const state = `{}`;
                async function main() {{
                    await init();
                    hydrate_{}(state);
                }}
                main();
                ",
            &hydration_function_name,
            &hydration_function_name,
            serialized_state,
            &hydration_function_name
        );
        Ok(view! {
            <!DOCTYPE html>
            <html lang={locale}>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    {self.head.map(|head_fn| (head_fn)(locale, &props)).unwrap_or(View::Empty)}
                </head>
                <body>
                {self.body.map(|body_fn| (body_fn)(locale, &props)).unwrap_or(View::Empty)}
                </body>
                <script type="module">
                {hydration_js}
                </script>
            </html>
        })
    }
}
