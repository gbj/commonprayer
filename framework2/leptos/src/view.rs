use std::sync::Arc;

use crate::{self as leptos2, router::ActionResponse};
use async_trait::async_trait;
use leptos_macro2::view;

use crate::router::Params;
use crate::{Node, Request, RouterError};

pub type MetaTags = Vec<(String, String)>;
pub type Styles = Vec<String>;
pub type Body = Vec<Node>;

#[async_trait(?Send)]
pub trait Loader
where
    Self: Sized,
{
    type Params: Params + Send + Sync + 'static;
    type Query: Params + Send + Sync + 'static;

    async fn loader(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self>;

    async fn action(
        locale: &str,
        req: Arc<dyn Request>,
        params: Self::Params,
        query: Self::Query,
    ) -> ActionResponse {
        ActionResponse::None
    }
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
