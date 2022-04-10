use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Determines when a piece of a document should be displayed
#[derive(
    Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display,
)]
pub enum Show {
    /// Displayed
    Always,
    /// Not displayed
    Hidden,
    /// Displayed only in template mode, i.e., when the document has not been compiled for a particular date
    TemplateOnly,
    /// Displayed only in concrete mode, i.e., when a document has been compiled for a particular date
    CompiledOnly,
}

impl Default for Show {
    fn default() -> Self {
        Self::Always
    }
}

impl Show {
    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
}
