use serde::{Deserialize, Serialize};

use crate::ReadingType;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Reading {
    pub reading_type: ReadingType,
    pub citation: String,
}

impl Reading {
    pub fn new(reading_type: ReadingType, citation: String) -> Self {
        Self {
            reading_type,
            citation,
        }
    }
}
