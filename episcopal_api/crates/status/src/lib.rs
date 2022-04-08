use serde::{Deserialize, Serialize};

/// Marks the status of a particular piece of liturgy or calendar observance
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Status {
    /// Fully authorized
    Authorized,
    /// Authorized for trial use
    TrialUse,
    /// Not officially authorized, but traditional or optional
    Unauthorized,
}

impl Default for Status {
    fn default() -> Self {
        Status::Authorized
    }
}

impl Status {
    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
}
