use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The Gloria Patri is formatted such that it is broken into four lines rather than two if necessary
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct DocumentError {
    message: String,
}

impl From<&str> for DocumentError {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl From<String> for DocumentError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
