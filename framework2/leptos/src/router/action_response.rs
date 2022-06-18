pub enum ActionResponse {
    None,
    Response(Box<http::Response<()>>),
    Error(Box<dyn std::error::Error + Send + Sync>),
}

impl ActionResponse {
    pub fn from_response(res: http::Response<()>) -> Self {
        Self::Response(Box::new(res))
    }

    pub fn from_error(e: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Error(Box::new(e))
    }
}
