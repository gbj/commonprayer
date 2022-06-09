mod params;
use leptos_macro2::view;
pub use params::Params;
use crate::{self as leptos2};

use std::{marker::PhantomData, fmt::Debug, collections::HashMap};

use thiserror::Error;

use crate::{view::{View, RenderedView}, Node, text};

pub struct Router {
	root: Box<dyn AnyRoute>
}

impl Router {
	pub fn new<T>(root: Route<T>) -> Self where T: View + Debug + 'static {
		Router { root: Box::new(root) }
	}

	pub fn route(&self, route: &str) -> Result<RenderedView, RouterError> {
		let query = parse_query(route);
		let mut before_query = route.split('?');
		let route = before_query.next().unwrap_or_default();
		let route_parts = route.split('/').collect::<Vec<_>>();
		let mut accumulator = RouteSearchAccumulator::new();
		self.root.search(&route_parts, &query, &mut accumulator)?;

		let meta = accumulator.meta.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();

		accumulator.body.reverse();
		let body = accumulator.body.into_iter().fold(None, |acc, curr| Some((curr)(acc))).unwrap_or_else(|| text(""));

		Ok(RenderedView {
			title: accumulator.title,
			meta,
			styles: accumulator.styles,
			body
		})
	}
}

pub struct FlattenedRoutes(Vec<String>);

///```
/// # use leptos2::router::parse_query;
/// let query = parse_query("https://www.commonprayeronline.org/en/hymnal?q=Search&hymnal=Hymnal1982");
/// assert_eq!(query.get("q"), Some(&"Search"));
/// assert_eq!(query.get("hymnal"), Some(&"Hymnal1982"));
/// let query = parse_query("https://www.commonprayeronline.org/en/hymnal");
/// assert!(query.is_empty());
/// let query = parse_query("https://www.commonprayeronline.org/en/hymnal?");
/// assert!(query.is_empty());
/// let query = parse_query("https://www.commonprayeronline.org/en/hymnal?q=");
/// assert_eq!(query.get("q"), Some(&""));
/// let query = parse_query("/en/hymnal?q=Search&hymnal=Hymnal1982");
/// assert_eq!(query.get("q"), Some(&"Search"));
/// assert_eq!(query.get("hymnal"), Some(&"Hymnal1982"));
///```
pub fn parse_query(route: &str) -> HashMap<String, String> {
	let mut parts = route.split('?');
	parts.next();
	parts.next().map(|query| query.split('&').filter_map(|piece| {
		let mut parts = piece.split('=');
		let (k, v) = (parts.next(), parts.next());
		match k {
			Some(k) if !k.is_empty() => Some((k.to_string(), v.unwrap_or_default().to_string())),
			_ => None
		}
	}).collect()).unwrap_or_default()
}

struct RouteSearchAccumulator<'a> {
	title: String,
	search_parts: Vec<&'a str>,
	matched_path: String,
	matched_params: HashMap<String, String>,
	matched_route: Option<&'a dyn AnyRoute>,
	meta: HashMap<String, String>,
	styles: Vec<String>,
	body: Vec<Box<dyn Fn(Option<Node>) -> Node>>
}

impl<'a> RouteSearchAccumulator<'a> {
	pub fn new() -> Self {
		Self { 
			search_parts: Vec::new(), 
			matched_path: String::new(), 
			matched_params: HashMap::new(), 
			matched_route: None,
			title: String::new(),
			meta: HashMap::new(),
			styles: Vec::new(),
			body: Vec::new()
		}
	}
}

#[derive(Debug)]
pub struct Route<T> where T: View + Debug {
	route_parts: Vec<String>,
	children: Vec<Box<dyn AnyRoute>>,
	view_type: PhantomData<T>
}

trait AnyRoute where Self: Debug + 'static {
	fn search(&self, parts: &[&str], query: &HashMap<String, String>, accumulator: &mut RouteSearchAccumulator) -> Result<(), RouterError>;

	//fn render(&self, matched_path: &str, matched_params: &HashMap<&str, &str>, query: &HashMap<&str, &str>) -> Result<RenderedView, RouterError>;
}

impl<T> AnyRoute for Route<T> where T: View + Debug + 'static, T::Params: Params, T::Query: Params {
    fn search(&self, parts: &[&str], query: &HashMap<String, String>, accumulator: &mut RouteSearchAccumulator) -> Result<(), RouterError> {
        match parts.get(0) {
			// if the path has ended, return, leaving the accumulator in its current state
			None => Ok(()),
			// if there's another path part, try to match it
			Some(next_part) => {
				if match_parts(parts.iter().copied(), self.route_parts.iter().map(|s| s.as_str())) {
					let mut consuming_match = false;

					// match as much of the route as this route matches
					for (idx, (concrete, matched)) in parts.iter().zip(self.route_parts.iter()).enumerate() {
						accumulator.matched_path.push('/');
						accumulator.matched_path.push_str(concrete);

						// add captures
						if matched.starts_with(':') {
							let key = matched.replace(':', "");
							accumulator.matched_params.insert(key, concrete.to_string());
						}

						// add ** captures
						if matched == "**" {
							consuming_match = true;
							let remaining_parts = parts[idx..].join("/");
							accumulator.matched_path.push('/');
							accumulator.matched_path.push_str(&remaining_parts);
							accumulator.matched_params.insert("remainder".to_string(), remaining_parts);
						}
					}

					// set title, metadata, etc. based on current state
					let de_params = T::Params::from_map(&accumulator.matched_params)?;
					let de_query = T::Query::from_map(query)?;
					let state = T::loader(&accumulator.matched_path, de_params, de_query).ok_or(RouterError::NotFound)?;
					accumulator.title = state.title();
					accumulator.meta.extend(state.meta().into_iter());
					accumulator.styles.extend(state.styles().into_iter());

					let matched_path = accumulator.matched_path.clone();
					accumulator.body.push(Box::new(move |nested_view| view! {
						<div data-route={&matched_path}>
							{state.body(nested_view)}
						</div>
					}));

					// remaining segments => match on children
					if !consuming_match {
						let remaining_parts = &parts[self.route_parts.len()..];
						for child in &self.children { 
							child.search(remaining_parts, &query, accumulator);
						}
					}

					Ok(())
				}
				else {
					for child in &self.children {
						child.search(parts, query, accumulator);
					}
					Ok(())
				}
			}
		}
    }
}

pub(crate) fn match_parts<'a>(concrete_parts: impl Iterator<Item = &'a str>, possible_matches: impl Iterator<Item = &'a str>) -> bool {
	let mut does_match = true;
	for (concrete_part, possible_match) in concrete_parts.zip(possible_matches) {
		if possible_match == "**" {
			return true;
		} else if possible_match != concrete_part && !possible_match.starts_with(':') {
			does_match = false;
		}
	}
	does_match
}

impl<T> Route<T> where T: View + Debug + 'static, T::Params: Params, T::Query: Params {
	fn match_own_route(&self, parts: &[&str], accumulator: &mut RouteSearchAccumulator) -> Result<(), RouterError> {
		todo!()
	}
}

#[derive(Error, Debug)]
pub enum RouterError {
	#[error("loader found no data at this path")]
	NoMatch,
	#[error("route was matched, but loader returned None")]
	NotFound,
	#[error("could not find parameter")]
    MissingParam(String),
	#[error("failed to deserialize parameters")]
    Params(Box<dyn std::error::Error>),
}


impl<T> Route<T> where T: View + Debug {
	pub fn new(route: &str) -> Self  {
		Self {
			route_parts: route.split('/').map(String::from).collect(),
			children: vec![],
			view_type: PhantomData
		}
	}

	pub fn child<C>(mut self, route: Route<C>) -> Self where C: View + Debug + 'static, C::Params: Params, C::Query: Params {
		self.children.push(Box::new(route));
		self
	}
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{view::View, Node};
	use crate::{self as leptos2, text, view};

    use super::RouteSearchAccumulator;
    use super::{Route, Router, params::Params};
	use leptos_macro2::Params;

	#[derive(Debug)]
	pub struct FakeView { }
	impl View for FakeView {
		type Params = ();
		type Query = ();

		fn loader(
			_path: &str,
			_params: Self::Params,
			_query: Self::Query,
		) -> Option<Self> {
			Some(Self { })
		}

		fn title(&self) -> String {
			String::from("Index!")
		}

		fn styles(&self) -> Vec<String> {
			vec![]
		}

		fn body(&self, nested_view: Option<Node>) -> Vec<Node> {
			vec![nested_view.unwrap_or_else(|| text("[FakeView] nothing nested here!"))]
		}
	}

	#[derive(Debug)]
	pub struct CalendarView { }
	impl View for CalendarView {
		type Params = ();
		type Query = ();

		fn loader(
			_path: &str,
			_params: Self::Params,
			_query: Self::Query,
		) -> Option<Self> {
			Some(Self { })
		}

		fn title(&self) -> String {
			String::from("Calendar!")
		}

		fn styles(&self) -> Vec<String> {
			vec![]
		}

		fn body(&self, nested_view: Option<Node>) -> Vec<Node> {
			vec![nested_view.unwrap_or_else(|| text("[CalendarView] nothing nested here!"))]
		}
	}

	#[derive(Params)]
	pub struct UserParams {
		id: String
	}
	
	#[derive(Debug)]
	pub struct UserView {
		id: String
	}
	impl View for UserView {
		type Params = UserParams;
		type Query = ();

		fn loader(
			path: &str,
			params: Self::Params,
			query: Self::Query,
		) -> Option<Self> {
			Some(Self { id: params.id })
		}

		fn title(&self) -> String {
			self.id.clone()
		}

		fn styles(&self) -> Vec<String> {
			vec![]
		}

		fn body(&self, nested_view: Option<Node>) -> Vec<Node> {
			view! {
				<>
					<h1>{&self.id}</h1>
					{nested_view.unwrap_or_else(|| text(""))}
				</>
			}
		}
	}

	#[derive(Params)]
	pub struct ProfileParams {
		id: String
	}
	
	#[derive(Debug)]
	pub struct ProfileView {
		id: String
	}
	impl View for ProfileView {
		type Params = ProfileParams;
		type Query = ();

		fn loader(
			path: &str,
			params: Self::Params,
			query: Self::Query,
		) -> Option<Self> {
			Some(Self { id: params.id })
		}

		fn title(&self) -> String {
			format!("{} - Profile", self.id)
		}

		fn styles(&self) -> Vec<String> {
			vec![]
		}

		fn body(&self, nested_view: Option<Node>) -> Vec<Node> {
			view! {
				<>
					<h2>"Profile"</h2>
					{nested_view.unwrap_or_else(|| text("[ProfileView] nothing nested"))}
				</>
			}
		}
	}

	#[derive(Params)]
	pub struct PostParams {
		id: String
	}
	
	#[derive(Debug)]
	pub struct PostView {
		id: String
	}
	impl View for PostView {
		type Params = PostParams;
		type Query = ();

		fn loader(
			path: &str,
			params: Self::Params,
			query: Self::Query,
		) -> Option<Self> {
			Some(Self { id: params.id })
		}

		fn title(&self) -> String {
			format!("{} - Posts", self.id)
		}

		fn styles(&self) -> Vec<String> {
			vec![]
		}

		fn body(&self, nested_view: Option<Node>) -> Vec<Node> {
			view! {
				<>
					<h1>"Posts"</h1>
				</>
			}
		}
	}

	#[test]
	fn test_match_fn() {
		use super::match_parts;
		assert!(match_parts([""].into_iter(), [""].into_iter()));
		assert!(!match_parts([""].into_iter(), ["user"].into_iter()));
		assert!(match_parts(["user"].into_iter(), ["user"].into_iter()));
		assert!(match_parts(["user", "gbj"].into_iter(), ["user", ":id"].into_iter()));
		assert!(!match_parts(["user", "gbj"].into_iter(), ["user", "profile"].into_iter()));
		assert!(match_parts(["document", "office", "morning-prayer"].into_iter(), ["document", "**"].into_iter()));
	}

	#[test]
	fn test_search_fn() {
		let routes = Router::new(
			Route::<FakeView>::new("")
			.child(Route::<CalendarView>::new("calendar"))
			.child(Route::<UserView>::new("user/:id")
				.child(Route::<ProfileView>::new("profile"))
				.child(Route::<PostView>::new("posts")))
			.child(Route::<FakeView>::new("about"))
		);

		let mut acc = RouteSearchAccumulator::new();
		routes.root.search(&["user", "gbj"], &HashMap::new(), &mut acc);
		assert_eq!(acc.matched_params.get("id"), Some(&"gbj".to_string()));
		assert_eq!(acc.title, "Index!".to_string());

		let mut acc = RouteSearchAccumulator::new();
		routes.root.search(&["user", "gbj", "profile"], &HashMap::new(), &mut acc);
		assert_eq!(acc.matched_params.get("id"), Some(&"gbj".to_string()));
		assert_eq!(acc.title, "gbj - Profile".to_string());
	}

	#[test]
	fn test_routing() {
		let routes = Router::new(
			Route::<FakeView>::new("")
			.child(Route::<CalendarView>::new("calendar"))
			.child(Route::<UserView>::new("user/:id")
				.child(Route::<ProfileView>::new("profile"))
				.child(Route::<PostView>::new("posts")))
			.child(Route::<FakeView>::new("about"))
		);

		let view = routes.route("user/gbj/profile").unwrap();
		assert_eq!(view.title, "gbj - Profile");
	}
}