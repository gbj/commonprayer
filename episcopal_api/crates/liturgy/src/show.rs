use serde::{Deserialize, Serialize};

/// Determines when a piece of a document should be displayed
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
