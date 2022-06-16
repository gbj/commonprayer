use crate as leptos2;
use async_trait::async_trait;
use leptos_macro2::view;
use serde::{Deserialize, Serialize};

use crate::router::Params;
use crate::{Node, RouterError};

pub type MetaTags = Vec<(String, String)>;
pub type Styles = Vec<String>;
pub type Body = Vec<Node>;

#[async_trait]
pub trait Loader
where
    Self: Sized,
{
    type Params: Params + 'static;
    type Query: Params + 'static;

    async fn loader(
        locale: &str,
        path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self>;
}

pub trait View {
    fn title(&self) -> String;

    fn meta(&self) -> MetaTags {
        vec![]
    }

    fn links(&self) -> Vec<Node> {
        vec![]
    }

    fn styles(&self) -> Styles;

    fn body(self: Box<Self>, nested_view: Option<Node>) -> Body;

    fn error_boundary(error: RouterError) -> Body
    where
        Self: Sized,
    {
        view! {
            <>
                <main class="error-boundary">
                    <h1>"Error!"</h1>
                    <pre class="error">{error.to_string()}</pre>
                </main>
            </>
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct RenderedView {
    pub locale: String,
    pub title: String,
    pub links: Vec<Node>,
    pub meta: MetaTags,
    pub styles: Styles,
    pub body: Node,
}
