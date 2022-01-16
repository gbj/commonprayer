use serde::{Deserialize, Serialize};

/// Marks the status of a particular piece of liturgy
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Status {
    /// Fully authorized
    Authorized,
    /// Authorized for trial use
    TrialUse,
    /// Not officially authorized, but traditional or optional
    Unauthorized,
}
