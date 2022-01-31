use serde::{Deserialize, Serialize};

mod h82;
mod levas;
mod wlp;

pub use h82::*;
pub use levas::*;
pub use wlp::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Hymnals {
    Hymnal1982,
    LEVAS,
    WLP,
}

#[derive(Clone, Debug, Serialize)]
pub struct Hymnal {
    pub title: &'static str,
    pub subtitle: &'static str,
    pub copyright: &'static str,
    pub year: u16,
    pub hymns: &'static [Hymn],
}

#[derive(Clone, Debug, Serialize)]
pub struct Hymn {
    pub source: Hymnals,
    pub number: HymnNumber,
    pub title: &'static str,
    pub tune: &'static str,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HymnNumber {
    S(u16),
    H(u16),
}
