use crate::{Loader, Node, Params, RouterError, View, Body};
use std::{collections::HashMap, marker::PhantomData, pin::Pin};

use futures::Future;

pub(crate) trait AnyRoute
where
    Self: Send + Sync + 'static,
{
    fn search(
        &self,
        locale: &str,
        parts: &[&str],
        params: &mut HashMap<String, String>,
        query: &HashMap<String, String>,
        matched_route_start: String,
    ) -> Result<Vec<RouteLoader>, RouterError>;

    fn route_name(&self) -> &'static str;

    fn error_boundary(&self, error: RouterError) -> Vec<Node>;
}

pub(crate) struct RouteLoader {
    pub(crate) route_name: &'static str,
    pub(crate) matched_route: String,
    //pub(crate) loader: Box<dyn Fn(&str, &str, &HashMap<String, String>, &HashMap<String, String>, Option<Node>) -> Pin<Box<dyn Future<Output = Result<RenderedPartial, RouterError>>>>>//Pin<Box<dyn Future<Output = RenderedPartial>>>,
    pub(crate) loader: Box<
        dyn Fn(
            &str,
            &str,
            &HashMap<String, String>,
            &HashMap<String, String>,
        ) -> Pin<Box<dyn Future<Output = Result<Box<dyn View>, RouterError>>>>,
    >,
    pub(crate) error_boundary: Box<dyn Fn(RouterError) -> Vec<Node>>
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

    fn error_boundary(&self, error: RouterError) -> Vec<Node> {
        T::error_boundary(error)
    }

    fn search(
        &self,
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
                        loaders.push(self.create_loader(&matched_route));
                        return Ok(loaders);
                    } else if part_matches(concrete_part, match_part, params) {
                        matched_idx += 1;
                    } else {
                        return Err(RouterError::NoMatch(format!("/{}/{}", locale, parts.join("/"))));
                    }
                }
                None => return Err(RouterError::NoMatch(format!("/{}/{}", locale, parts.join("/"))))
            }
        }

        // add the loader and matched routes (will only happen if matched; if not matched, has already errored)

        // if there are parts remaining, try to match children
        if parts.len() > matched_idx {
            let remaining_parts = &parts[matched_idx..];
            for child in &self.children {
                if let Ok(matched_loaders) = child.search(locale, remaining_parts, params, query, matched_route.clone()) {
                    for concrete_part in &parts[0..matched_idx] {
                        matched_route.push('/');
                        matched_route.push_str(concrete_part);
                    }
                    loaders.push(self.create_loader(&matched_route));
                    loaders.extend(matched_loaders);
                    return Ok(loaders);
                }
            }
        } else {
            for concrete_part in &parts[0..matched_idx] {
                matched_route.push('/');
                matched_route.push_str(concrete_part);
            }
            loaders.push(self.create_loader(&matched_route));
            return Ok(loaders);
        }

        Err(RouterError::NoMatch(format!("/{}/{}", locale, parts.join("/"))))
    }
}

fn part_matches(concrete_part: &str, route_part: &str, params: &mut HashMap<String, String>) -> bool {
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
    T: Loader + View + Send + Sync + 'static,
{
    fn create_loader(&self, matched_route: &str) -> RouteLoader {
        let matched_route = matched_route.to_string();
        RouteLoader {
            route_name: self.route_name(),
            matched_route: matched_route.to_string(),
            loader: Box::new(move |locale, path, params, query| {
                let locale = locale.to_string();
                let path = path.to_string();
                let params = T::Params::from_map(params);
                let query = T::Query::from_map(query);

                // if Params or Query can't be serialized, either handle error or pass it up
                match (params, query) {
                    (Ok(params), Ok(query)) => {let matched_route = matched_route.to_string(); Box::pin(async move {
                        match T::loader(&locale, &path, params, query).await {
                            None => Err(RouterError::NotFound(matched_route)),
                            Some(data) => Ok(Box::new(data) as Box<dyn View>)
                        }
                    })},
                    (Ok(_), Err(e)) => Box::pin(async { Err(e) }),
                    (Err(e), _) => Box::pin(async { Err(e) }),
                }
            }),
            error_boundary: Box::new(|err| T::error_boundary(err)),
        }
    }
}

impl<T> Route<T> 
where
    T: Default + Loader + View + Send + Sync + 'static
{
    pub fn default_body(nested_view: Option<Node>) -> Body {
        T::default().body(nested_view)
    }
}
