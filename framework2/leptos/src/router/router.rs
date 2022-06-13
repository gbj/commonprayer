use std::collections::HashMap;

use futures::future::join_all;
use leptos_macro2::view;
use crate::{self as leptos2, Element, Request};

use crate::{AnyRoute, Loader, Node, RenderedView, Route, View};

use super::{partial::RenderedPartial, query::parse_query};

pub struct Router<R> where
R: Default + Loader + View + Send + Sync + 'static {
    root: Route<R>,
    pub locales: Vec<String>,
}

impl<R> Router<R> 
where
        R: Default + Loader + View + Send + Sync + 'static{
    pub fn new(
        root: Route<R>,
        locales: &[&str],
    ) -> Self
    {
        Router {
            root,
            locales: locales.iter().copied().map(String::from).collect()
        }
    }

    pub async fn route(&self, req: &impl Request) -> RenderedView {
        let path = req.path();
        let locale = self.locales.iter().find(|locale| path.starts_with(&format!("/{}", locale))).cloned().unwrap_or_else(|| "en".to_string());
        let path = path.replace(&format!("/{}", locale), "");
        let query = parse_query(req.query_string());
        let parts = path
            .split('/')
            .filter(|n| !n.is_empty())
            .collect::<Vec<_>>();
        let mut params = HashMap::new();

        match self
            .root
            .search(&locale, &parts, &mut params, &query, String::new())
        {
            Ok(loaders) => {
                let loader_futures = loaders
                    .iter()
                    .map(|loader| (loader.loader)(&locale, &path, &params, &query));
                let partial = join_all(loader_futures).await.into_iter().enumerate().fold(
                    RenderedPartial::default(),
                    |mut acc, (loader_idx, curr)| {
						let body = match curr {
							Ok(curr) => {
                                acc.locale = locale.clone();
								acc.title = curr.title();
								acc.links.extend(curr.links());
								acc.meta.extend(curr.meta());
								acc.styles.extend(curr.styles());
								Box::new(move |nested_view| curr.body(nested_view)) as Box<dyn FnOnce(Option<Node>) -> Vec<Node>>
							},
							Err(e) => {
								let error_boundary = (loaders[loader_idx].error_boundary)(e);
								Box::new(move |_| error_boundary) as Box<dyn FnOnce(Option<Node>) -> Vec<Node>>
							},
						};

						let matched_route = (&loaders[loader_idx].matched_route).clone();
						let locale = locale.to_string();

                        acc.body.push(Box::new(move |nested_view| view! {
							<div data-locale={locale} data-route={matched_route}>
								{(body)(nested_view)}
							</div>
						}));

						acc
                    },
                );
                partial.into()
            }
            Err(e) => {
				let error_boundary = Node::Element(Element::new("div").child( R::error_boundary(e)));
				RenderedView {
                    locale,
                    title: R::default().title(),
                    styles: R::default().styles(),
                    links: R::default().links(),
                    meta: R::default().meta(),
					body: Node::Element(Element::new("div data-route").child(Route::<R>::default_body(Some(error_boundary)))),
				}
			}
        }
    }
}
