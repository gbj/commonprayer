use crate::{ActionResponse, Body, Loader, Node, Params, Request, RouterError, View};
use std::{collections::HashMap, marker::PhantomData, pin::Pin, sync::Arc};

use futures::Future;

pub(crate) trait AnyRoute
where
    Self: Send + Sync + 'static,
{
    fn search(
        &self,
        req: &Arc<dyn Request>,
        locale: &str,
        parts: &[&str],
        params: &mut HashMap<String, String>,
        query: &HashMap<String, String>,
        matched_route_start: String,
    ) -> Result<Vec<RouteLoader>, RouterError>;

    fn route_name(&self) -> &'static str;

    fn error_boundary(&self, error: RouterError) -> Body;

    fn is_index(&self) -> bool;

    fn create_loader(&self, matched_route: Option<&str>, include_action: bool) -> RouteLoader;
}

pub(crate) struct RouteLoader {
    pub(crate) route_name: &'static str,
    pub(crate) matched_route: Option<String>,
    pub(crate) loader: Box<
        dyn Fn(
            &str,
            Arc<dyn Request>,
            &HashMap<String, String>,
            &HashMap<String, String>,
        ) -> Pin<Box<dyn Future<Output = Result<Box<dyn View>, RouterError>>>>,
    >,
    pub(crate) action: Option<
        Box<
            dyn Fn(
                &str,
                Arc<dyn Request>,
                &HashMap<String, String>,
                &HashMap<String, String>,
            ) -> Pin<Box<dyn Future<Output = ActionResponse>>>,
        >,
    >,
    pub(crate) error_boundary: Box<dyn Fn(RouterError) -> Body>,
}

pub struct Route<T>
where
    T: View + Send + Sync,
{
    route_parts: Vec<String>,
    children: Vec<Box<dyn AnyRoute>>,
    view_type: PhantomData<T>,
    error_boundary: Option<Box<dyn Fn(RouterError) -> Node + Send + Sync>>,
}

impl<T> Route<T>
where
    T: View + Send + Sync,
{
    pub fn new(route: &str) -> Self {
        Self {
            route_parts: route
                .split('/')
                .filter(|n| !n.is_empty())
                .map(String::from)
                .collect(),
            children: vec![],
            view_type: PhantomData,
            error_boundary: None,
        }
    }

    pub fn child<C>(mut self, route: Route<C>) -> Self
    where
        C: Loader + View + Send + Sync + 'static,
        C::Params: Params,
        C::Query: Params,
    {
        self.children.push(Box::new(route));
        self
    }

    pub fn error_boundary(
        mut self,
        error_boundary: impl Fn(RouterError) -> Node + Send + Sync + 'static,
    ) -> Self {
        self.error_boundary = Some(Box::new(error_boundary));
        self
    }
}

impl<T> AnyRoute for Route<T>
where
    T: Loader + View + Send + Sync + 'static,
{
    fn route_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn error_boundary(&self, error: RouterError) -> Body {
        T::error_boundary(error)
    }

    fn is_index(&self) -> bool {
        self.route_parts.is_empty()
    }

    fn search(
        &self,
        req: &Arc<dyn Request>,
        locale: &str,
        parts: &[&str],
        params: &mut HashMap<String, String>,
        query: &HashMap<String, String>,
        mut matched_route: String,
    ) -> Result<Vec<RouteLoader>, RouterError> {
        let mut loaders = Vec::new();
        let mut matched_idx = 0;

        // first, consume own route parts
        for (idx, match_part) in self.route_parts.iter().enumerate() {
            match parts.get(idx) {
                Some(concrete_part) => {
                    if match_part == "**" {
                        let remaining_parts = parts[idx..].join("/");
                        for concrete_part in &parts[0..matched_idx] {
                            matched_route.push('/');
                            matched_route.push_str(concrete_part);
                        }
                        matched_route.push('/');
                        matched_route.push_str(&remaining_parts);
                        params.insert("remainder".to_string(), remaining_parts);
                        loaders.push(self.create_loader(Some(&matched_route), true));

                        return Ok(loaders);
                    } else if part_matches(concrete_part, match_part, params) {
                        matched_idx += 1;
                    } else {
                        return Err(RouterError::NoMatch(format!(
                            "/{}/{}",
                            locale,
                            parts.join("/")
                        )));
                    }
                }
                None => {
                    return Err(RouterError::NoMatch(format!(
                        "/{}/{}",
                        locale,
                        parts.join("/")
                    )))
                }
            }
        }

        // add the loader and matched routes (will only happen if matched; if not matched, has already errored)

        // if there are parts remaining, try to match children
        if parts.len() > matched_idx {
            let remaining_parts = &parts[matched_idx..];
            for child in &self.children {
                if let Ok(matched_loaders) = child.search(
                    req,
                    locale,
                    remaining_parts,
                    params,
                    query,
                    matched_route.clone(),
                ) {
                    for concrete_part in &parts[0..matched_idx] {
                        matched_route.push('/');
                        matched_route.push_str(concrete_part);
                    }
                    loaders.push(self.create_loader(Some(&matched_route), false));
                    loaders.extend(matched_loaders);

                    return Ok(loaders);
                }
            }
        } else {
            for concrete_part in &parts[0..matched_idx] {
                matched_route.push('/');
                matched_route.push_str(concrete_part);
            }
            loaders.push(self.create_loader(Some(&matched_route), true));

            // add index route if it exists
            if let Some(index_route) = self.children.iter().find(|route| route.is_index()) {
                loaders.push(index_route.create_loader(None, true))
            }

            return Ok(loaders);
        }

        Err(RouterError::NoMatch(format!(
            "/{}/{}",
            locale,
            parts.join("/")
        )))
    }

    fn create_loader(&self, matched_route: Option<&str>, include_action: bool) -> RouteLoader {
        let matched_route = matched_route.map(String::from);
        RouteLoader {
            route_name: self.route_name(),
            matched_route: matched_route.clone(),
            loader: Box::new(move |locale, req, params, query| {
                let locale = locale.to_string();
                let params = T::Params::from_map(params);
                let query = T::Query::from_map(query);

                // if Params or Query can't be serialized, either handle error or pass it up
                match (params, query) {
                    (Ok(params), Ok(query)) => {
                        let matched_route = matched_route.clone();
                        let req = req.clone();
                        Box::pin(async move {
                            match T::loader(&locale, req, params, query).await {
                                None => {
                                    Err(RouterError::NotFound(matched_route.unwrap_or_default()))
                                }
                                Some(data) => Ok(Box::new(data) as Box<dyn View>),
                            }
                        })
                    }
                    (Ok(_), Err(e)) => Box::pin(async { Err(e) }),
                    (Err(e), _) => Box::pin(async { Err(e) }),
                }
            }),
            action: if include_action {
                Some(Box::new(move |locale, req, params, query| {
                    let locale = locale.to_string();
                    let params = T::Params::from_map(params);
                    let query = T::Query::from_map(query);

                    // if Params or Query can't be serialized, either handle error or pass it up
                    match (params, query) {
                        (Ok(params), Ok(query)) => {
                            Box::pin(async move { T::action(&locale, req, params, query).await })
                        }
                        (Ok(_), Err(e)) => Box::pin(async { ActionResponse::Error(Box::new(e)) }),
                        (Err(e), _) => Box::pin(async { ActionResponse::Error(Box::new(e)) }),
                    }
                }))
            } else {
                None
            },
            error_boundary: Box::new(|err| T::error_boundary(err)),
        }
    }
}

fn part_matches(
    concrete_part: &str,
    route_part: &str,
    params: &mut HashMap<String, String>,
) -> bool {
    if concrete_part == route_part {
        true
    } else if route_part.starts_with(':') {
        params.insert(route_part.replacen(':', "", 1), concrete_part.to_string());
        true
    } else {
        false
    }
}

impl<T> Route<T>
where
    T: Default + Loader + View + Send + Sync + 'static,
{
    pub fn default_body(nested_view: Option<Node>) -> Body {
        Box::new(T::default()).body(nested_view)
    }
}
