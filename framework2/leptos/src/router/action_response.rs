use std::path::PathBuf;

pub enum ActionResponse {
    None,
    Response(Box<http::Response<()>>),
    Json(String),
    Error(Box<dyn std::error::Error + Send + Sync>),
    File(PathBuf),
}

impl ActionResponse {
    pub fn from_response(res: http::Response<()>) -> Self {
        Self::Response(Box::new(res))
    }

    pub fn from_error(e: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Error(Box::new(e))
    }

    pub fn from_json(data: impl serde::Serialize + 'static) -> Self {
        match serde_json::to_string(&data) {
            Ok(json) => Self::Json(json),
            Err(e) => Self::from_error(e),
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self::File(path)
    }
}
