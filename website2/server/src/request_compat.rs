pub struct RequestCompat(actix_web::HttpRequest, Vec<u8>);

impl leptos2::Request for RequestCompat {
    fn path(&self) -> &str {
        self.0.path()
    }

    fn query_string(&self) -> &str {
        self.0.query_string()
    }

    fn method(&self) -> http::Method {
        self.0.method().into()
    }

    fn headers(&self) -> leptos2::http::HeaderMap {
        let actix_map = self.0.headers();
        let mut map = leptos2::http::HeaderMap::new();
        for (key, val) in actix_map.into_iter() {
            map.insert(key, val.clone());
        }
        map
    }

    fn body(&self) -> Option<leptos2::RequestBody> {
        Some(leptos2::RequestBody::from(self.1.as_slice()))
    }
}

impl RequestCompat {
    pub fn new(req: actix_web::HttpRequest, body_bytes: Vec<u8>) -> Self {
        Self(req, body_bytes)
    }
}
