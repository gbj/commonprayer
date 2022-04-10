use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Marks the status of a particular piece of liturgy or calendar observance
#[derive(
    Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize, EnumIter, EnumString, Display,
)]
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
