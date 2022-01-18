use std::fmt::Display;

use crate::DisplayFormat;
use serde::{Deserialize, Serialize};

/// The Gloria Patri is formatted such that it is broken into four lines rather than two if necessary
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct GloriaPatri {
    pub display_format: DisplayFormat,
    pub text: (String, String, String, String),
}

impl GloriaPatri {
    /// Sets the [DisplayFormat](crate::DisplayFormat) for the document.
    #[must_use]
    pub fn display_format(mut self, display_format: DisplayFormat) -> Self {
        self.display_format = display_format;
        self
    }
}

impl<A, B, C, D> From<(A, B, C, D)> for GloriaPatri
where
    A: Display,
    B: Display,
    C: Display,
    D: Display,
{
    fn from((a, b, c, d): (A, B, C, D)) -> Self {
        Self {
            display_format: DisplayFormat::Default,
            text: (a.to_string(), b.to_string(), c.to_string(), d.to_string()),
        }
    }
}
