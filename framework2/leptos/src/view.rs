use serde::{Serialize, Deserialize};

use crate::Node;
use crate::router::Params;

pub type MetaTags = Vec<(String, String)>;
pub type Styles = Vec<String>;
pub type Body = Vec<Node>;

pub trait View
where
    Self: Sized {

	type Params: Params + 'static;
    type Query: Params + 'static;

    fn loader(
        path: &str,
        params: Self::Params,
        query: Self::Query,
    ) -> Option<Self>;

	fn title(&self) -> String;

	fn meta(&self) -> MetaTags {
		vec![]
	}

	fn styles(&self) -> Styles;

	fn body(&self, nested_view: Option<Node>) -> Body;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RenderedView {
	pub title: String,
	pub meta: MetaTags,
	pub styles: Styles,
	pub body: Node
}